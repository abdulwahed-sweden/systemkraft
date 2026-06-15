-- ============================================================
-- SystemKraft — demo / reference dataset
-- Aligned with the public site: four disciplines, the engagement
-- model, real Swedish clients, on-brand engagements and case studies.
-- Apply after migrations:
--   psql "$DATABASE_URL" -f seeds/mansour_demo.sql
-- Clears the project tables first, then reseeds. Does NOT touch the
-- framework's auth/audit tables.
-- ============================================================

BEGIN;

TRUNCATE engagements, case_studies, services, service_categories, clients, inquiries
    RESTART IDENTITY CASCADE;

-- ── Disciplines (the four service categories on the site) ─────
INSERT INTO service_categories (name, slug) VALUES
    ('Enterprise Data Operations',     'enterprise-data-operations'),
    ('Business Systems Engineering',   'business-systems-engineering'),
    ('Unified Control Panels',         'unified-control-panels'),
    ('Audit & Authority Architecture', 'audit-authority-architecture');

-- ── Services (the offerings, mapped to disciplines) ───────────
INSERT INTO services (category_id, name, slug, summary, problem_solved, tech_focus, engagement_model, active)
SELECT c.id, v.name, v.slug, v.summary, v.problem_solved, v.tech_focus, v.engagement_model, v.active
FROM (VALUES
    ('enterprise-data-operations',
     'Data Consolidation & Single Source of Truth', 'data-consolidation',
     'We unify disparate data sources into one high-performance PostgreSQL core, so every department reads and writes the same numbers.',
     'Data silos and the absence of a single source of truth across departments.',
     'PostgreSQL, schema design, ETL pipelines', 'fixed', TRUE),
    ('enterprise-data-operations',
     'Legacy Migration & Decommissioning', 'legacy-migration',
     'We extract data trapped in legacy ERPs, CSVs and spreadsheets and migrate it into the operational core — then retire the old tools.',
     'Critical business data held hostage in third-party tools and brittle exports.',
     'Migration pipelines, data validation', 'fixed', TRUE),
    ('business-systems-engineering',
     'Core Engine Build — Rust & PostgreSQL', 'core-engine-build',
     'We engineer the central, memory-safe back end and data core that the whole operation runs on, deployed as a single binary.',
     'Fragile manual workflows and disconnected SaaS stitched together with webhooks.',
     'Rust, PostgreSQL, single-binary deployment', 'fixed', TRUE),
    ('unified-control-panels',
     'Operational Control Panel', 'operational-control-panel',
     'A minimalist, keyboard-first admin panel giving decision-makers instant, unified visibility into the entire operation.',
     'Cluttered, slow interfaces that delay critical business decisions.',
     'Keyboard-first UI, low-latency rendering', 'fixed', TRUE),
    ('audit-authority-architecture',
     'RBAC & Audit Architecture', 'rbac-audit-architecture',
     'Strict role-based access control and immutable audit trails engineered into the foundation — not bolted on afterward.',
     'Unmanaged authority and the inability to answer "who did what, and when?"',
     'RBAC, immutable audit logs, correlation ids', 'fixed', TRUE),
    ('business-systems-engineering',
     'Operational Assurance & SLA', 'operational-assurance',
     'We keep the engine running flawlessly around the clock: hosting, backups, patches and a guaranteed uptime SLA.',
     'A delivered system with no one accountable for keeping it running.',
     'GDPR-compliant hosting, backups, monitoring', 'retainer', TRUE)
) AS v(cat_slug, name, slug, summary, problem_solved, tech_focus, engagement_model, active)
JOIN service_categories c ON c.slug = v.cat_slug;

-- ── Clients (Swedish enterprises — illustrative) ──────────────
INSERT INTO clients (company_name, org_number, contact_name, email, city, industry, status) VALUES
    ('Nordkraft Systems AB',   '556421-1180', 'Erik Lindqvist',  'erik.lindqvist@nordkraft.example',   'Stockholm', 'Energy',         'active'),
    ('Västkust Robotics AB',   '556733-2204', 'Johan Persson',   'johan.persson@vastkustrobotics.example', 'Malmö',   'Manufacturing',  'active'),
    ('Bergslagen Logistik AB', '556902-7741', 'Anna Holm',       'anna.holm@bergslagen.example',       'Örebro',    'Logistics',      'active'),
    ('Lagom Fintech AB',       '559014-3382', 'Sara Bergström',  'sara.bergstrom@lagomfintech.example', 'Göteborg', 'Fintech',        'lead'),
    ('Kiruna Mining Tech AB',  '556188-9925', 'Lars Eklund',     'lars.eklund@kirunatech.example',     'Kiruna',    'Mining',         'lead'),
    ('Solvind Energi AB',      '556650-5512', 'Ingrid Falk',     'ingrid.falk@solvind.example',        'Uppsala',   'Energy',         'past');

-- ── Engagements (delivered work, mapped client × service) ─────
INSERT INTO engagements (client_id, service_id, title, status, started)
SELECT cl.id, s.id, v.title, v.status, now() - (v.age_days || ' days')::interval
FROM (VALUES
    ('Nordkraft Systems AB',   'core-engine-build',         'Operations data core',                'in_progress', 38),
    ('Nordkraft Systems AB',   'rbac-audit-architecture',   'RBAC & audit rollout',                'in_progress', 21),
    ('Västkust Robotics AB',   'operational-control-panel', 'Production dispatch control panel',    'in_progress', 16),
    ('Västkust Robotics AB',   'legacy-migration',          'Legacy ERP migration',                'delivered',   150),
    ('Bergslagen Logistik AB', 'core-engine-build',         'Dispatch system replacement',          'in_progress', 9),
    ('Bergslagen Logistik AB', 'operational-assurance',     'Operational assurance retainer',       'delivered',   95),
    ('Lagom Fintech AB',       'data-consolidation',        'Single source of truth diagnosis',     'proposed',    4),
    ('Kiruna Mining Tech AB',  'data-consolidation',        'Reporting unification assessment',     'proposed',    2),
    ('Solvind Energi AB',      'core-engine-build',         'Customer portal data core',            'delivered',   320),
    ('Solvind Energi AB',      'rbac-audit-architecture',   'Audit trail implementation',           'delivered',   300)
) AS v(company, svc_slug, title, status, age_days)
JOIN clients  cl ON cl.company_name = v.company
JOIN services s  ON s.slug = v.svc_slug;

-- ── Case studies (proof — problem / solution / outcome) ───────
INSERT INTO case_studies (service_id, title, problem, solution, outcome, published)
SELECT s.id, v.title, v.problem, v.solution, v.outcome, TRUE
FROM (VALUES
    ('data-consolidation',
     'From four tools to one source of truth',
     'An energy operator ran critical operations across spreadsheets and three disconnected internal apps, with no department ever looking at the same numbers.',
     'We consolidated every source into one PostgreSQL core, defined the single source of truth, and gave each team a unified view of the same live data.',
     'Roughly 160 hours of manual reconciliation eliminated per week; management reporting moved from a five-day lag to real time.'),
    ('core-engine-build',
     'Replacing a fragile dispatch with a single binary',
     'A manufacturer''s dispatch ran on a web of SaaS tools glued together with webhooks; when one link broke, the line stopped.',
     'We engineered a single-binary Rust engine on one PostgreSQL core, ingesting every input once and structurally preventing invalid data entry.',
     'Human data-entry errors fell to zero; infrastructure cost stayed flat as headcount grew, with no per-seat tax.'),
    ('rbac-audit-architecture',
     'Audit-by-default for a regulated operation',
     'A logistics operator could not answer "who changed this, and when?" — permissions and security had been bolted on late.',
     'We engineered strict role-based access control and an immutable audit trail into the foundation, with a correlation id on every action.',
     'The operation became compliance-ready, with complete traceability of every authority change across the system.'),
    ('operational-control-panel',
     'A control panel decision-makers actually use',
     'Leadership waited on analysts to assemble reports from cluttered, slow dashboards, delaying every operational decision.',
     'We shipped a minimalist, keyboard-first control panel reading directly from the operational core, built for heavy daily use.',
     'Decision velocity went from days to instant; the leadership team self-serves live figures without touching a spreadsheet.')
) AS v(svc_slug, title, problem, solution, outcome)
JOIN services s ON s.slug = v.svc_slug;

-- ── Inquiries (inbound assessment requests) ───────────────────
INSERT INTO inquiries (company, email, service_interest, message) VALUES
    ('Mälardalen Industri AB', 'cfo@malardalen.example', 'Data silos / no single source of truth',
     'We run production, sales and finance on three systems that never agree. We need one source of truth before our next board report.'),
    ('Skåne Transport Group',  'coo@skanetransport.example', 'Manual data entry between tools',
     'Our dispatchers re-key every order into four tools. Can SystemKraft replace that with one engine?'),
    ('Norrland Verkstad AB',   'it@norrlandverkstad.example', 'SaaS sprawl / per-seat costs',
     'Per-seat subscriptions are scaling faster than our headcount. We want to own the infrastructure instead of renting it.');

COMMIT;
