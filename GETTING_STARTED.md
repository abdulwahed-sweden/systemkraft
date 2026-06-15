# Getting started

A 5-minute orientation for opening this project for the first time. The
`README.md` covers install in depth — this is the cheat sheet.

## 1. Run it

```sh
createdb mansour_dev               # once
cp .env.example .env               # local Postgres defaults
rustio-admin migrate apply         # applies ./migrations
psql "$DATABASE_URL" -f seeds/mansour_demo.sql   # optional SystemKraft demo data
cargo run                          # http://127.0.0.1:8000
```

Create a login (prompts for the password twice):

```sh
rustio-admin user create --email you@systemkraft.dev --role administrator
```

## 2. The pages

| URL | Page |
|-----|------|
| `/` | Operational Overview — the pitch (four disciplines + the paradigm shift) |
| `/investment` | Business Case & ROI — the engagement model and capital equation |
| `/assessment` | Operational Assessment — the 3-question diagnostic + lead form |
| `/dashboard-preview` | Dashboard Preview — live Postgres counts + engagement table |
| `/admin` | The CRM behind it all — sign in here |

## 3. Where things live

| Path | What |
|------|------|
| `src/main.rs` | Boots the server, registers the six models, serves the public pages + `/assets/*`, and renders the assessment form (CSRF) and dashboard (live stats) |
| `src/*.rs` | One model per file (`#[derive(RustioAdmin)]` + `impl ModelAdmin`) |
| `migrations/*.sql` | Schema, applied in order; append-only |
| `seeds/mansour_demo.sql` | The SystemKraft dataset: disciplines, services, clients, engagements, case studies, inquiries |
| `templates/*.html` | The four public pages (home, investment, assessment, dashboard_preview) |
| `static/css/main.css`, `static/js/main.js` | Shared design system + theme toggle, served at `/assets/*` |
| `SERVICES.md` | The offering as standalone marketing copy |

> The public pages and `static/*` assets are embedded at compile time via
> `include_str!`, so edits there take effect after a `cargo run` (rebuild).
> Admin templates reload without a rebuild.

## 4. The model graph

```
ServiceCategory ──< Service ──< CaseStudy
        Client ──< Engagement >── Service
                       Inquiry
```

Foreign keys are wired into the admin as **inlines**: open a **Client** and its
**Engagements** show on the edit page; open a **Service** and its **Case
studies** show inline. The Operational Assessment form writes an **Inquiry**;
the Dashboard Preview reads counts and engagement rows from these tables live.

## 5. Add a model

Either add a row in `/admin`, or — for a new *kind* of data — scaffold a model:

```sh
rustio-admin startapp testimonial \
    --field client:fk:Client \
    --field quote:text \
    --field published:bool
```

Then register it in `src/main.rs` under `// rustio: models` and add a numbered
migration. Field types: `str`, `text`, `int`, `bigint`, `bool`, `timestamp`,
`json`, `fk:<Model>`.
