-- Service categories: the four lines of work (Rust admin platforms,
-- security audits, robotics/systems, secure web apps).
-- Append-only: once applied, never edit this file — add a new numbered one.

CREATE TABLE service_categories (
    id   BIGSERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "slug" TEXT NOT NULL UNIQUE
);
