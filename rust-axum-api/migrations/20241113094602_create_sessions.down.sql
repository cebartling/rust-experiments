-- Drop triggers first
DROP TRIGGER IF EXISTS set_timestamp ON users;

-- Drop functions
DROP FUNCTION IF EXISTS trigger_set_timestamp;

-- Drop indexes
DROP INDEX IF EXISTS idx_sessions_token;
DROP INDEX IF EXISTS idx_sessions_user_id;

-- Drop tables
DROP TABLE IF EXISTS users;

