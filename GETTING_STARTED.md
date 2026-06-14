# Getting started

A 5-minute orientation for opening this project for the first time. The
`README.md` covers install in depth — this is the cheat sheet.

## 1. Run it

```sh
createdb mansour_dev               # once
cp .env.example .env               # local Postgres defaults
rustio-admin migrate apply         # applies ./migrations
psql "$DATABASE_URL" -f seeds/mansour_demo.sql   # optional demo data
cargo run                          # http://127.0.0.1:8000
```

Create a login (prompts for the password twice):

```sh
rustio-admin user create --email you@mansour.dev --role administrator
```

## 2. Where things live

| Path | What |
|------|------|
| `src/main.rs` | Boots the server, registers the six models, renders the homepage with live stats |
| `src/*.rs` | One model per file (`#[derive(RustioAdmin)]` + `impl ModelAdmin`) |
| `migrations/*.sql` | Schema, applied in order; append-only |
| `seeds/mansour_demo.sql` | Demo categories, services, clients, engagements, case studies |
| `templates/home.html` | The public English landing page |
| `SERVICES.md` | The service catalog as standalone marketing copy |

## 3. The model graph

```
ServiceCategory ──< Service ──< CaseStudy
        Client ──< Engagement >── Service
                       Inquiry
```

Foreign keys are wired into the admin as **inlines**: open a **Client** and its
**Engagements** show on the edit page; open a **Service** and its **Case
studies** show inline.

## 4. Add a service line

Either add a `Service` row in the admin, or — for a new *kind* of data —
scaffold a model:

```sh
rustio-admin startapp testimonial \
    --field client:fk:Client \
    --field quote:text \
    --field published:bool
```

Then register it in `src/main.rs` under `// rustio: models` and add a numbered
migration. Field types: `str`, `text`, `int`, `bigint`, `bool`, `timestamp`,
`json`, `fk:<Model>`.
