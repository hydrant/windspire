-- Remove OAuth provider fields from users table
ALTER TABLE users 
DROP CONSTRAINT IF EXISTS unique_provider_user;

DROP INDEX IF EXISTS idx_users_email;

ALTER TABLE users 
DROP COLUMN IF EXISTS provider_id,
DROP COLUMN IF EXISTS provider_name,
DROP COLUMN IF EXISTS avatar_url,
DROP COLUMN IF EXISTS created_at,
DROP COLUMN IF EXISTS updated_at;