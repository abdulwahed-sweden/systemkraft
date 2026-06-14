-- Engagements — a piece of work delivered for a client under a service.
-- Append-only: once applied, never edit this file — add a new numbered one.

CREATE TABLE engagements (
    id          BIGSERIAL PRIMARY KEY,
    "client_id"   BIGINT NOT NULL REFERENCES clients(id),
    "service_id"  BIGINT NOT NULL REFERENCES services(id),
    "title"       TEXT NOT NULL,
    "status"      TEXT NOT NULL CHECK ("status" IN ('proposed', 'in_progress', 'delivered')),
    "started"     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX engagements_client_idx  ON engagements (client_id);
CREATE INDEX engagements_service_idx ON engagements (service_id);
