-- Clients — Swedish companies, tracked from first contact through delivery.
-- Append-only: once applied, never edit this file — add a new numbered one.

CREATE TABLE clients (
    id            BIGSERIAL PRIMARY KEY,
    "company_name"  TEXT NOT NULL,
    "org_number"    TEXT NOT NULL DEFAULT '',
    "contact_name"  TEXT NOT NULL DEFAULT '',
    "email"         TEXT NOT NULL DEFAULT '',
    "city"          TEXT NOT NULL DEFAULT '',
    "industry"      TEXT NOT NULL DEFAULT '',
    "status"        TEXT NOT NULL CHECK ("status" IN ('lead', 'active', 'past'))
);
