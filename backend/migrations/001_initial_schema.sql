-- Migration: 001_initial_schema
CREATE TABLE IF NOT EXISTS inquiries (
    id           TEXT PRIMARY KEY,
    company_name TEXT NOT NULL,
    contact_name TEXT NOT NULL,
    email        TEXT NOT NULL,
    phone        TEXT,
    country      TEXT NOT NULL,
    product_ids  TEXT NOT NULL DEFAULT '[]',
    volume_mt    REAL,
    message      TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'new',
    created_at   TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS newsletter_subscribers (
    email         TEXT PRIMARY KEY,
    name          TEXT,
    subscribed_at TEXT NOT NULL,
    active        INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_inquiries_created ON inquiries(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_inquiries_status  ON inquiries(status);
