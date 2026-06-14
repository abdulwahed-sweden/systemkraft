-- Services offered to clients. Each belongs to a category and states the
-- problem it solves, the technology focus, and how it is engaged.
-- Append-only: once applied, never edit this file — add a new numbered one.

CREATE TABLE services (
    id              BIGSERIAL PRIMARY KEY,
    "category_id"     BIGINT NOT NULL REFERENCES service_categories(id),
    "name"            TEXT NOT NULL,
    "slug"            TEXT NOT NULL UNIQUE,
    "summary"         TEXT NOT NULL DEFAULT '',
    "problem_solved"  TEXT NOT NULL DEFAULT '',
    "tech_focus"      TEXT NOT NULL DEFAULT '',
    "engagement_model" TEXT NOT NULL CHECK ("engagement_model" IN ('fixed', 'retainer', 'hourly')),
    "active"          BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX services_category_idx ON services (category_id);
