# mansour

**Rust systems, security &amp; admin platforms — services for Swedish companies.**

This repository is both a **services site** and a **working demonstration of
the stack it sells**. The public homepage markets four service lines in
English; the `/admin` panel is a small, fully-audited CRM that manages the
services, the clients (Swedish companies), the engagements, and the case
studies behind them.

It is built on [`rustio-admin`](https://github.com/abdulwahed-sweden/rustio-admin)
— a Postgres-first administrative framework for Rust — so a visitor sees the
exact qualities on offer running live: memory-safe Rust, a complete audit
trail, role-based access, and **single-binary deployment with no build step**.

> The services themselves are described in [`SERVICES.md`](./SERVICES.md).

---

## What's inside

| Surface | URL | What it is |
|---------|-----|------------|
| Homepage | <http://127.0.0.1:8000/> | English services landing page with **live stats** from Postgres (`templates/home.html`) |
| Admin panel | <http://127.0.0.1:8000/admin> | The CRM — sign in here |

### Data model

Six related models, wired with foreign keys and admin inlines:

```
ServiceCategory ──< Service ──< CaseStudy
        Client ──< Engagement >── Service
                       Inquiry            (inbound contact)
```

- **ServiceCategory** — the four lines of work.
- **Service** — an offering: summary, the problem it solves, tech focus, engagement model, active flag.
- **Client** — a Swedish company: contact, city, industry, status (`lead` / `active` / `past`).
- **Engagement** — work for a client under a service (`proposed` / `in_progress` / `delivered`).
- **CaseStudy** — problem / solution / outcome behind a service; `published` gates the homepage count.
- **Inquiry** — inbound contact submissions.

---

## Run it

Prerequisites: **Rust** (1.88+), **PostgreSQL**, and the `rustio-admin` CLI
(`cargo install rustio-admin-cli`). This project path-deps the in-repo
framework at `../rustio-admin`, so keep the two side by side.

```sh
# 1. Configure the environment.
cp .env.example .env            # defaults target a local Postgres

# 2. Create the database (the one step the CLI cannot do for you).
createdb mansour_dev

# 3. Apply migrations and create your first operator.
rustio-admin migrate apply
rustio-admin user create --email you@mansour.dev --role administrator
#    (the CLI prompts for the password twice)

# 4. (Optional) Load demo data so the panel and homepage look alive.
psql "$DATABASE_URL" -f seeds/mansour_demo.sql

# 5. Boot.
cargo run                       # http://127.0.0.1:8000
```

### Admin URLs

Routes are **plural model name + verb**:

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

- **Homepage copy & styling** — `templates/home.html`. The colour tokens live
  in the `:root` block at the top.
- **Services content** — edit rows in the admin, or `seeds/mansour_demo.sql`.
- **A new model** — `rustio-admin startapp <name> --field ...`, then register
  it in `src/main.rs` under the `// rustio: models` marker.

## License

MIT © Abdulwahed Mansour. See [`LICENSE`](./LICENSE).
