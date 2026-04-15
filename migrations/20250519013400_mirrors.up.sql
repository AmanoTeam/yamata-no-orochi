CREATE TABLE IF NOT EXISTS mirrors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    owner_id INT8 NOT NULL,
    token TEXT NOT NULL,
    state TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ON mirrors (created_at DESC, updated_at DESC);

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
  ON mirrors
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
  ON mirrors
  FOR EACH ROW
  EXECUTE PROCEDURE update_updated_at();
