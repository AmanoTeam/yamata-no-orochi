CREATE TABLE IF NOT EXISTS users (
    id            INT8        PRIMARY KEY,
    anilist_id    INT8        UNIQUE,
    anilist_token TEXT,
    language_code TEXT        NOT NULL DEFAULT 'pt',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON users(created_at DESC,
                      updated_at DESC);
