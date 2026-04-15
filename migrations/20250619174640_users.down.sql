-- Remove password column
ALTER TABLE users
DROP COLUMN password;

-- Remove permission level column
ALTER TABLE users
DROP COLUMN perm_level;

-- Remove avatar URL column
ALTER TABLE users
DROP COLUMN avatar_url;
