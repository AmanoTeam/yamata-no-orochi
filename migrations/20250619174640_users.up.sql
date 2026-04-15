-- Add password column
ALTER TABLE users
ADD COLUMN password TEXT;

-- Add permission level column
ALTER TABLE users
ADD COLUMN perm_level INT4 NOT NULL DEFAULT 1;

-- Add avatar_url column
ALTER TABLE users
ADD COLUMN avatar_url TEXT;
