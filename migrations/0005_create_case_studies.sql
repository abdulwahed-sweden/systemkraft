-- Case studies — proof behind each service: problem, solution, outcome.
-- Append-only: once applied, never edit this file — add a new numbered one.

CREATE TABLE case_studies (
    id          BIGSERIAL PRIMARY KEY,
    "service_id"  BIGINT NOT NULL REFERENCES services(id),
    "title"       TEXT NOT NULL,
    "problem"     TEXT NOT NULL DEFAULT '',
    "solution"    TEXT NOT NULL DEFAULT '',
    "outcome"     TEXT NOT NULL DEFAULT '',
    "published"   BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX case_studies_service_idx ON case_studies (service_id);
