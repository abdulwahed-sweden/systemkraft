use std::sync::Arc;

use rustio_admin::admin::Admin;
use rustio_admin::middleware;
use rustio_admin::templates::Templates;
use rustio_admin::{
    auth, background, migrations, register_admin_routes, Db, Error, Response, Result, Router,
    Server,
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

// The public homepage at `/`. Baked at compile time so the binary stays
// single-file. Replace by editing templates/home.html and re-running.
const HOMEPAGE_HTML: &str = include_str!("../templates/home.html");

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
        .app_name("Mansour")
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
    let home_db = db.clone();
    let router = Router::new()
        .middleware(middleware::logger)
        .middleware(middleware::correlation_id)
        .middleware(middleware::security_headers)
        .middleware(middleware::csrf_protect)
        .get("/", move |_req| {
            let db = home_db.clone();
            async move { Ok(Response::html(render_home(&db).await)) }
        });

    let router = register_admin_routes(router, admin, db, Arc::clone(&templates));

    let addr = "127.0.0.1:8000".parse().expect("valid listen address");
    log::info!("listening on http://{addr}/admin");
    Server::new(router, addr).run().await
}

/// Render the public homepage with live figures pulled from Postgres at
/// request time. Every value falls back to a dash if the query fails, so
/// the page always renders even if the database hiccups.
async fn render_home(db: &Db) -> String {
    use rustio_admin::sqlx::{self, Row};

    let mut db_name = "—".to_string();
    let mut pg_major = "—".to_string();
    let (mut services, mut case_studies, mut clients, mut engagements) = (0_i64, 0_i64, 0_i64, 0_i64);
    let mut models = 0_i64;

    let query = "SELECT current_database() AS db, \
                 split_part(current_setting('server_version'), '.', 1) AS pg, \
                 (SELECT count(*) FROM services   WHERE active)    AS services, \
                 (SELECT count(*) FROM case_studies WHERE published) AS case_studies, \
                 (SELECT count(*) FROM clients)                    AS clients, \
                 (SELECT count(*) FROM engagements)                AS engagements, \
                 (SELECT count(*) FROM pg_class c \
                    JOIN pg_namespace n ON n.oid = c.relnamespace \
                  WHERE n.nspname = 'public' AND c.relkind = 'r' \
                    AND c.relname IN ('service_categories', 'services', 'clients', \
                                      'engagements', 'case_studies', 'inquiries')) AS models";

    if let Ok(row) = sqlx::query(query).fetch_one(db.pool()).await {
        db_name = row.try_get::<String, _>("db").unwrap_or(db_name);
        pg_major = row.try_get::<String, _>("pg").unwrap_or(pg_major);
        services = row.try_get::<i64, _>("services").unwrap_or(0);
        case_studies = row.try_get::<i64, _>("case_studies").unwrap_or(0);
        clients = row.try_get::<i64, _>("clients").unwrap_or(0);
        engagements = row.try_get::<i64, _>("engagements").unwrap_or(0);
        models = row.try_get::<i64, _>("models").unwrap_or(0);
    } else {
        log::warn!("homepage: live stats query failed; rendering with placeholders");
    }

    HOMEPAGE_HTML
        .replace("{{DB_NAME}}", &db_name)
        .replace("{{PG_MAJOR}}", &pg_major)
        .replace("{{SERVICES}}", &services.to_string())
        .replace("{{CASE_STUDIES}}", &case_studies.to_string())
        .replace("{{CLIENTS}}", &clients.to_string())
        .replace("{{ENGAGEMENTS}}", &engagements.to_string())
        .replace("{{MODELS}}", &models.to_string())
}
