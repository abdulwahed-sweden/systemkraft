use std::sync::Arc;

use rustio_admin::admin::Admin;
use rustio_admin::middleware::{self, CsrfGuard};
use rustio_admin::templates::Templates;
use rustio_admin::{
    auth, background, migrations, register_admin_routes, Db, Error, Request, Response, Result,
    Router, Server,
};

// rustio: modules
mod case_study;
mod client;
mod engagement;
mod inquiry;
mod service;
mod service_category;

// rustio: imports
use case_study::CaseStudy;
use client::Client;
use engagement::Engagement;
use inquiry::Inquiry;
use service::Service;
use service_category::ServiceCategory;

// Public pages, baked at compile time so the binary stays single-file.
// Home and investment are fully static; assessment and dashboard carry
// `{{...}}` markers filled per request (CSRF token / live Postgres figures).
const HOMEPAGE_HTML: &str = include_str!("../templates/home.html");
const INVESTMENT_HTML: &str = include_str!("../templates/investment.html");
const ASSESSMENT_HTML: &str = include_str!("../templates/assessment.html");
const DASHBOARD_HTML: &str = include_str!("../templates/dashboard_preview.html");

// Shared front-end assets, served at /assets/* with explicit content-types
// (the framework sends `nosniff`, so the type must be exact). Single source
// of truth for every page's styling and the theme toggle.
const MAIN_CSS: &str = include_str!("../static/css/main.css");
const MAIN_JS: &str = include_str!("../static/js/main.js");

// Shown above the form after a successful POST (post/redirect/get).
const SUCCESS_BANNER: &str = r#"<div class="form-success"><h2>Assessment received.</h2><p class="lead lead--sm" style="margin: 8px 0 0">Thank you — your request is in. We read every submission personally and reply from a real engineer, not an automated funnel.</p></div>"#;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let db_url = std::env::var("DATABASE_URL").map_err(|_| {
        Error::Internal(
            "DATABASE_URL is not set. Copy .env.example to .env and edit it before running.".into(),
        )
    })?;

    let db = Db::connect(&db_url).await?;
    auth::init_tables(&db).await?;
    migrations::apply(&db, "migrations").await?;
    background::spawn_housekeeping(db.clone());

    // `.app_name(...)` flows the brand into the admin tab title, login
    // header, and dashboard h1. Each `.model::<_>()` registers a CRUD
    // surface with auth, audit, search, filters and inlines for free.
    let admin = Admin::new()
        .app_name("SystemKraft")
        // rustio: models
        .model::<ServiceCategory>()
        .model::<Service>()
        .model::<Client>()
        .model::<Engagement>()
        .model::<CaseStudy>()
        .model::<Inquiry>();

    admin.seed_permissions(&db).await?;

    // Project template overrides in ./templates win over the framework's
    // embedded defaults.
    let template_dir = std::env::var("RUSTIO_TEMPLATE_DIR").unwrap_or_else(|_| "templates".into());
    let templates = Templates::new(Some(template_dir.into()))?;

    // Middleware order is locked: logger → correlation_id → security_headers
    // → csrf_protect, so every audit row under one request shares one
    // UUID v7 correlation id (surfaced at /admin/history).
    let dash_db = db.clone();
    let assess_db = db.clone();
    let router = Router::new()
        .middleware(middleware::logger)
        .middleware(middleware::correlation_id)
        .middleware(middleware::security_headers)
        .middleware(middleware::csrf_protect)
        // Static pages.
        .get("/", |_req| async move {
            Ok(Response::html(HOMEPAGE_HTML.to_string()))
        })
        .get("/investment", |_req| async move {
            Ok(Response::html(INVESTMENT_HTML.to_string()))
        })
        // Assessment: GET renders the form (CSRF token from context + an
        // optional success banner); POST stores the inquiry and redirects.
        .get("/assessment", |req| async move { Ok(render_assessment(&req)) })
        .post("/assessment", move |req| {
            let db = assess_db.clone();
            async move { handle_inquiry(req, &db).await }
        })
        // Dashboard preview: live Postgres connection + counts.
        .get("/dashboard-preview", move |_req| {
            let db = dash_db.clone();
            async move { Ok(Response::html(render_dashboard(&db).await)) }
        })
        // Shared assets (exact content-types for nosniff).
        .get("/assets/main.css", |_req| async move {
            Ok(Response::ok(MAIN_CSS).with_header("content-type", "text/css; charset=utf-8"))
        })
        .get("/assets/main.js", |_req| async move {
            Ok(Response::ok(MAIN_JS).with_header("content-type", "text/javascript; charset=utf-8"))
        });

    let router = register_admin_routes(router, admin, db, Arc::clone(&templates));

    let addr = "127.0.0.1:8000".parse().expect("valid listen address");
    log::info!("listening on http://{addr}/admin");
    Server::new(router, addr).run().await
}

/// Render the assessment page: inject the per-request CSRF token (placed in
/// the request context by `csrf_protect`) into the form's hidden field, and
/// show a success banner when arriving via the post/redirect/get `?sent=1`.
fn render_assessment(req: &Request) -> Response {
    let token = req
        .ctx()
        .get::<CsrfGuard>()
        .map(|g| g.token.clone())
        .unwrap_or_default();
    let result = if req.query().get("sent") == Some("1") {
        SUCCESS_BANNER
    } else {
        ""
    };
    Response::html(
        ASSESSMENT_HTML
            .replace("{{CSRF}}", &token)
            .replace("{{RESULT}}", result),
    )
}

/// Store an assessment submission as an `Inquiry`, then redirect back to the
/// form with a success flag (post/redirect/get, so a refresh won't resubmit).
/// CSRF has already been validated by `csrf_protect` before we get here.
async fn handle_inquiry(req: Request, db: &Db) -> Result<Response> {
    use rustio_admin::sqlx;

    let form = req.form().unwrap_or_default();
    let company = form.get("company").unwrap_or("").trim();
    let email = form.get("email").unwrap_or("").trim();
    let interest = form.get("service_interest").unwrap_or("").trim();
    let name = form.get("name").unwrap_or("").trim();
    let body = form.get("message").unwrap_or("").trim();
    let message = if name.is_empty() {
        body.to_string()
    } else {
        format!("Contact: {name}\n\n{body}")
    };

    if let Err(e) = sqlx::query(
        "INSERT INTO inquiries (company, email, service_interest, message) VALUES ($1, $2, $3, $4)",
    )
    .bind(company)
    .bind(email)
    .bind(interest)
    .bind(message)
    .execute(db.pool())
    .await
    {
        log::error!("assessment: failed to store inquiry: {e}");
    }

    Ok(Response::redirect("/assessment?sent=1"))
}

/// Render the dashboard preview with live figures pulled from Postgres at
/// request time. Every value falls back to a dash if the query fails, so the
/// page always renders even if the database hiccups.
async fn render_dashboard(db: &Db) -> String {
    use rustio_admin::sqlx::{self, Row};

    let mut db_name = "—".to_string();
    let mut pg_major = "—".to_string();
    let (mut eng_active, mut clients, mut services, mut models) = (0_i64, 0_i64, 0_i64, 0_i64);

    let query = "SELECT current_database() AS db, \
                 split_part(current_setting('server_version'), '.', 1) AS pg, \
                 (SELECT count(*) FROM engagements WHERE status = 'in_progress') AS eng_active, \
                 (SELECT count(*) FROM clients)  AS clients, \
                 (SELECT count(*) FROM services WHERE active) AS services, \
                 (SELECT count(*) FROM pg_class c \
                    JOIN pg_namespace n ON n.oid = c.relnamespace \
                  WHERE n.nspname = 'public' AND c.relkind = 'r' \
                    AND c.relname IN ('service_categories', 'services', 'clients', \
                                      'engagements', 'case_studies', 'inquiries')) AS models";

    if let Ok(row) = sqlx::query(query).fetch_one(db.pool()).await {
        db_name = row.try_get::<String, _>("db").unwrap_or(db_name);
        pg_major = row.try_get::<String, _>("pg").unwrap_or(pg_major);
        eng_active = row.try_get::<i64, _>("eng_active").unwrap_or(0);
        clients = row.try_get::<i64, _>("clients").unwrap_or(0);
        services = row.try_get::<i64, _>("services").unwrap_or(0);
        models = row.try_get::<i64, _>("models").unwrap_or(0);
    } else {
        log::warn!("dashboard: live stats query failed; rendering with placeholders");
    }

    // The "Recent engagements" table is rendered live from the database,
    // so the dashboard's rows always match the seeded operation — one
    // source of truth, which is exactly what the site sells.
    let mut rows = String::new();
    let rows_query = "SELECT c.company_name AS account, e.status AS status \
                      FROM engagements e JOIN clients c ON c.id = e.client_id \
                      ORDER BY e.started DESC, e.id DESC LIMIT 6";
    if let Ok(list) = sqlx::query(rows_query).fetch_all(db.pool()).await {
        for row in &list {
            let account: String = row.try_get("account").unwrap_or_default();
            let status: String = row.try_get("status").unwrap_or_default();
            let (label, cls) = match status.as_str() {
                "in_progress" => ("In delivery", "blue"),
                "delivered" => ("Delivered", "green"),
                "proposed" => ("Proposed", "amber"),
                other => (other, "blue"),
            };
            rows.push_str(&format!(
                "<tr><td>{}</td><td><span class=\"pill {}\">{}</span></td></tr>",
                html_escape(&account),
                cls,
                label
            ));
        }
    }
    if rows.is_empty() {
        rows.push_str("<tr><td colspan=\"2\">No engagements yet.</td></tr>");
    }

    DASHBOARD_HTML
        .replace("{{DB_NAME}}", &db_name)
        .replace("{{PG_MAJOR}}", &pg_major)
        .replace("{{ENG_ACTIVE}}", &eng_active.to_string())
        .replace("{{CLIENTS}}", &clients.to_string())
        .replace("{{SERVICES}}", &services.to_string())
        .replace("{{MODELS}}", &models.to_string())
        .replace("{{ENGAGEMENT_ROWS}}", &rows)
}

/// Minimal HTML-escape for values interpolated into page markup.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
