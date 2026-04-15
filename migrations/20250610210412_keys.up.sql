CREATE TABLE IF NOT EXISTS keys (
    id SERIAL PRIMARY KEY,
    user_id INT8 NOT NULL UNIQUE REFERENCES users (id),
    value TEXT NOT NULL UNIQUE,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ON keys (created_at DESC);

-- Always reset the created_at timestamp
CREATE OR REPLACE FUNCTION stop_change_on_created_at()
  RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at := OLD.created_at;
  RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER prevent_created_at_changes
  BEFORE UPDATE
  ON keys
  FOR EACH ROW
  EXECUTE PROCEDURE stop_change_on_created_at();
