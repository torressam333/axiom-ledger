CREATE TABLE processed_requests (
    idempotency_key UUID PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'IN_PROGRESS',
    response_payload JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- This index allows the database to instantly identify the old records for deletion.
CREATE INDEX idx_processed_requests_created_at ON processed_requests(created_at);
