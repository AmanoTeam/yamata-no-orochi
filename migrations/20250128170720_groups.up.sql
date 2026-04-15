CREATE TABLE IF NOT EXISTS groups (
    id INT8 PRIMARY KEY,
    language_code TEXT NOT NULL DEFAULT 'pt',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ON groups (created_at DESC, updated_at DESC);

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
  ON groups
  FOR EACH ROW
  EXECUTE PROCEDURE stop_change_on_created_at();

-- Update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at()
  RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at := CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER update_updated_at_trigger
  BEFORE UPDATE
  ON groups
  FOR EACH ROW
  EXECUTE PROCEDURE update_updated_at();
