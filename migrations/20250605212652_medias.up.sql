CREATE TABLE IF NOT EXISTS medias (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    anilist_id INT4 NOT NULL,
    chat_id INT8 NOT NULL REFERENCES groups (id),
    message_id INT4 NOT NULL,
    title TEXT,
    season INT2,
    episode TEXT NOT NULL,
    language_code TEXT NOT NULL,
    subtitled BOOLEAN NOT NULL DEFAULT FALSE,
    added_by INT8 NOT NULL REFERENCES users (id),
    notes TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ON medias (created_at DESC, updated_at DESC);

-- Always reset the created_at timestamp
CREATE OR REPLACE FUNCTION stop_change_on_created_at()
  RETURNS TRIGGER AS $$
BEGIN
  NEW.created_at := OLD.created_at;
  RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER prevent_created_at_changes
  BEFORE UPDATE
  ON medias
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

CREATE TRIGGER update_updated_at_trigger
  BEFORE UPDATE
  ON medias
  FOR EACH ROW
  EXECUTE PROCEDURE update_updated_at();
