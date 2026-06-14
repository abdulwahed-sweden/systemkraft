-- Inquiries — inbound contact submissions from prospective clients.
-- Append-only: once applied, never edit this file — add a new numbered one.

CREATE TABLE inquiries (
    id              BIGSERIAL PRIMARY KEY,
    "company"         TEXT NOT NULL DEFAULT '',
    "email"           TEXT NOT NULL DEFAULT '',
    "service_interest" TEXT NOT NULL DEFAULT '',
    "message"         TEXT NOT NULL DEFAULT '',
    "created"         TIMESTAMPTZ NOT NULL DEFAULT now()
);
