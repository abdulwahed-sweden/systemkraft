-- Demo data for the `mansour` services admin.
-- Apply after migrations:
--   psql "$DATABASE_URL" -f seeds/mansour_demo.sql
-- Idempotent-ish: clears the project tables first, then reseeds. It does
-- NOT touch the framework's auth/audit tables.

BEGIN;

TRUNCATE engagements, case_studies, services, service_categories, clients, inquiries
    RESTART IDENTITY CASCADE;

-- ── Categories ────────────────────────────────────────────────
INSERT INTO service_categories (name, slug) VALUES
    ('Rust Admin & Internal Platforms', 'rust-admin-platforms'),
    ('Security Audits',                 'security-audits'),
    ('Systems & Robotics Engineering',  'systems-robotics'),
    ('Secure Web Applications',         'secure-web-apps');

-- ── Services ──────────────────────────────────────────────────
INSERT INTO services (category_id, name, slug, summary, problem_solved, tech_focus, engagement_model, active)
SELECT c.id, v.name, v.slug, v.summary, v.problem_solved, v.tech_focus, v.engagement_model, TRUE
FROM (VALUES
    ('rust-admin-platforms',
     'Rust Admin Platform Build', 'rust-admin-build',
     'A production back-office in Rust with auth, sessions, recovery and a full audit trail designed as one system.',
     'Slow, hard-to-audit internal tools and a sprawl of disconnected admin apps no one trusts.',
     'rustio-admin, Postgres, single-binary deployment', 'fixed'),
    ('rust-admin-platforms',
     'Internal Tooling Retainer', 'internal-tooling-retainer',
     'Ongoing development and operation of internal Rust tooling as the business grows.',
     'Internal tools that rot because no one owns them after launch.',
     'rustio-admin, RustIO, CI', 'retainer'),
    ('security-audits',
     'Smart-Contract Security Audit', 'smart-contract-audit',
     'Independent, doctrine-driven review of smart contracts and on-chain protocols with a written, actionable report.',
     'Capital and reputation at risk from vulnerabilities that surface only after launch.',
     'Solidity, DeFi protocols, threat modelling', 'fixed'),
    ('systems-robotics',
     'Robotics Control Software', 'robotics-control',
     'Reliable control software with one trait surface shared between simulation and hardware.',
     'Two divergent codebases for sim and hardware that drift apart and fail in the field.',
     'Rust, nalgebra, kinematics, motion planning', 'hourly'),
    ('secure-web-apps',
     'Secure Web Application', 'secure-web-app',
     'Web applications where login, recovery, sessions and audit logging are first-class from day one.',
     'Security features bolted on late, leaving the gaps that always creates.',
     'Rust, Django, audit-by-default', 'fixed')
) AS v(cat_slug, name, slug, summary, problem_solved, tech_focus, engagement_model)
JOIN service_categories c ON c.slug = v.cat_slug;

-- ── Clients (Swedish companies — illustrative) ────────────────
INSERT INTO clients (company_name, org_number, contact_name, email, city, industry, status) VALUES
    ('Nordkraft Systems AB', '556000-1111', 'Erik Lindqvist', 'erik@nordkraft.example', 'Stockholm', 'Energy',       'active'),
    ('Lagom Fintech AB',     '556000-2222', 'Sara Bergström', 'sara@lagomfintech.example', 'Göteborg',  'Fintech',      'lead'),
    ('Västkust Robotics AB', '556000-3333', 'Johan Persson',  'johan@vastkustrobotics.example', 'Malmö', 'Manufacturing', 'active'),
    ('Solvind Energi AB',    '556000-4444', 'Anna Holm',      'anna@solvind.example', 'Uppsala',   'Energy',       'past');

-- ── Engagements ───────────────────────────────────────────────
INSERT INTO engagements (client_id, service_id, title, status, started)
SELECT cl.id, s.id, v.title, v.status, now() - (v.age_days || ' days')::interval
FROM (VALUES
    ('Nordkraft Systems AB', 'rust-admin-build',  'Operations admin platform',        'in_progress', 40),
    ('Västkust Robotics AB', 'robotics-control',  'Pick-and-place control rewrite',    'delivered',   120),
    ('Lagom Fintech AB',     'smart-contract-audit', 'Pre-launch protocol audit',      'proposed',    5),
    ('Solvind Energi AB',    'secure-web-app',    'Customer portal with audit log',    'delivered',   300)
) AS v(company, svc_slug, title, status, age_days)
JOIN clients  cl ON cl.company_name = v.company
JOIN services s  ON s.slug = v.svc_slug;

-- ── Case studies ──────────────────────────────────────────────
INSERT INTO case_studies (service_id, title, problem, solution, outcome, published)
SELECT s.id, v.title, v.problem, v.solution, v.outcome, TRUE
FROM (VALUES
    ('rust-admin-build',
     'A back-office that auditors trust',
     'An energy operator ran critical operations through spreadsheets and three disconnected internal apps with no record of who changed what.',
     'Built a single Rust admin platform on rustio-admin: typed models, role-based access, and an audit trail with correlation ids across every action.',
     'One reviewable system; every authority change recorded; deployed as a single binary with no frontend build step.'),
    ('smart-contract-audit',
     'Catching the bug before mainnet',
     'A DeFi protocol needed independent assurance before putting user funds at risk on launch.',
     'Threat-modelled the protocol and reviewed it line by line against a written security doctrine, with each finding ranked by impact.',
     'Issues surfaced and fixed pre-launch; a clear report the team could act on and share with their community.'),
    ('robotics-control',
     'One codebase for sim and hardware',
     'A robotics team maintained separate code for simulation and the physical arm, and the two kept drifting apart.',
     'Designed one Rust trait surface shared by the simulator and the hardware backend, with pure kinematics and an explicit motion planner.',
     'Behaviour validated in simulation now matches hardware; new backends plug in without touching the planner.')
) AS v(svc_slug, title, problem, solution, outcome)
JOIN services s ON s.slug = v.svc_slug;

-- ── Inquiries (inbound) ───────────────────────────────────────
INSERT INTO inquiries (company, email, service_interest, message) VALUES
    ('Bergslagen Logistik AB', 'cto@bergslagen.example', 'Rust admin platform',
     'We want to replace our internal dispatch tool. Can we talk about a Rust admin build?'),
    ('Kiruna Mining Tech AB',  'ops@kirunatech.example', 'Robotics control',
     'Interested in control software for an automated handling arm.');

COMMIT;
