-- Add OAuth provider fields to users table
ALTER TABLE users 
ADD COLUMN provider_id VARCHAR NULL,
ADD COLUMN provider_name VARCHAR NULL,
ADD COLUMN avatar_url VARCHAR NULL,
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();

-- Add unique constraint for provider_id and provider_name combination
ALTER TABLE users 
ADD CONSTRAINT unique_provider_user UNIQUE (provider_id, provider_name);

-- Create index on email for faster OAuth user lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Update existing users with timestamps
UPDATE users 
SET created_at = NOW(), updated_at = NOW() 
WHERE created_at IS NULL;

-- Add comments
COMMENT ON COLUMN users.provider_id IS 'OAuth provider user ID (e.g., Google user ID)';
COMMENT ON COLUMN users.provider_name IS 'OAuth provider name (e.g., google, github)';
COMMENT ON COLUMN users.avatar_url IS 'User profile picture URL from OAuth provider';