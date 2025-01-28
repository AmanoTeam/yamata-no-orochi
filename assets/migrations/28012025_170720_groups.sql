CREATE TABLE IF NOT EXISTS groups (
    id            INT8        PRIMARY KEY,
    language_code TEXT        NOT NULL DEFAULT 'pt',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON groups(created_at DESC,
                       updated_at DESC);
