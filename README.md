# SystemKraft

**Enterprise Data Operations &amp; Business Systems Engineering — for Swedish enterprises.**

SystemKraft transforms fragmented digital infrastructure into a single,
centralized operating system. This repository is both the **public site** and
a **working demonstration of the stack it sells**: a multi-page marketing site
plus a fully-audited `/admin` control panel, all served from one Rust binary.

It is built on [`rustio-admin`](https://github.com/abdulwahed-sweden/rustio-admin)
— a Postgres-first administrative framework for Rust — so a visitor sees the
exact qualities on offer running live: a memory-safe Rust core, one PostgreSQL
source of truth, a complete audit trail, role-based access, and **single-binary
deployment with no build step**.

> The offering is described in [`SERVICES.md`](./SERVICES.md).

---

## What's inside

A multi-page public site and the admin panel behind it, all on one binary:

| Surface | URL | What it is |
|---------|-----|------------|
| Operational Overview | <http://127.0.0.1:8000/> | The pitch: four disciplines + the paradigm shift |
| Business Case &amp; ROI | <http://127.0.0.1:8000/investment> | The engagement model and the capital equation |
| Operational Assessment | <http://127.0.0.1:8000/assessment> | The 3-question diagnostic + a CSRF-protected lead form |
| Dashboard Preview | <http://127.0.0.1:8000/dashboard-preview> | A read-only control panel with **live Postgres** figures |
| Admin panel | <http://127.0.0.1:8000/admin> | The CRM behind it all — sign in here |

The public pages are plain HTML served from `templates/`; the shared design
system lives in `static/css/main.css` and `static/js/main.js`, served at
`/assets/*`. The dashboard and the assessment form read and write the database
at request time — the single source of truth the site is selling.

### Data model

Six related models, wired with foreign keys and admin inlines:

```
ServiceCategory ──< Service ──< CaseStudy
        Client ──< Engagement >── Service
                       Inquiry            (inbound assessment requests)
```

- **ServiceCategory** — the four disciplines (Enterprise Data Operations,
  Business Systems Engineering, Unified Control Panels, Audit &amp; Authority
  Architecture).
- **Service** — an offering: summary, the problem it solves, tech focus,
  engagement model (`fixed` / `retainer` / `hourly`), active flag.
- **Client** — a Swedish enterprise: contact, city, industry, status
  (`lead` / `active` / `past`).
- **Engagement** — work for a client under a service
  (`proposed` / `in_progress` / `delivered`).
- **CaseStudy** — problem / solution / outcome behind a service.
- **Inquiry** — inbound submissions from the Operational Assessment form.

---

## Run it

Prerequisites: **Rust** (1.88+), **PostgreSQL**, and the `rustio-admin` CLI
(`cargo install rustio-admin-cli`). The project depends on the published
`rustio-admin` crate, so a fresh clone builds standalone.

```sh
# 1. Configure the environment.
cp .env.example .env            # defaults target a local Postgres

# 2. Create the database (the one step the CLI cannot do for you).
createdb systemkraft_dev

# 3. Apply migrations and create your first operator.
rustio-admin migrate apply
rustio-admin user create --email you@systemkraft.dev --role administrator
#    (the CLI prompts for the password twice)

# 4. (Optional) Load the SystemKraft demo dataset so the panel and
#    dashboard are populated.
psql "$DATABASE_URL" -f seeds/systemkraft_demo.sql

# 5. Boot.
cargo run                       # http://127.0.0.1:8000
```

### Routes

Public pages: `/`, `/investment`, `/assessment` (GET form + POST submission),
`/dashboard-preview`, and the shared assets `/assets/main.css` &amp;
`/assets/main.js`.

Admin routes are **plural model name + verb**:

| Action | URL |
|--------|-----|
| List | `/admin/services` |
| Add | `/admin/services/new` |
| Edit | `/admin/services/<id>/edit` |
| Delete | `/admin/services/<id>/delete` |

Same shape for `clients`, `engagements`, `case_studies`, `service_categories`,
`inquiries`. The audit history is at `/admin/history`.

---

## Customising

- **Page copy &amp; markup** — `templates/*.html`. Each page is self-contained
  HTML that links the shared assets.
- **Design system** — `static/css/main.css` holds the RustIO design tokens
  (Cobalt / Rust accents, Inter + JetBrains Mono, light/dark). Editing it
  restyles every page at once. The theme toggle is `static/js/main.js`.
  > Note: the public pages and assets are embedded at compile time via
  > `include_str!`, so changes to `templates/*.html` or `static/*` take effect
  > after a `cargo run` (rebuild). Admin templates reload without a rebuild.
- **Content** — edit rows in `/admin`, or the `seeds/systemkraft_demo.sql` dataset.
- **A new model** — `rustio-admin startapp <name> --field ...`, then register
  it in `src/main.rs` under the `// rustio: models` marker.

## License

MIT © Abdulwahed Mansour. See [`LICENSE`](./LICENSE).
