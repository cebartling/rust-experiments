-- Sessions table for authentication
CREATE TABLE sessions
(
    id         UUID PRIMARY KEY         DEFAULT uuid_generate_v4(),
    user_id    UUID                     NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token      VARCHAR(255)             NOT NULL UNIQUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create updated_at trigger for users
CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON sessions
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

-- Add indexes
CREATE INDEX idx_sessions_token ON sessions (token);
CREATE INDEX idx_sessions_user_id ON sessions (user_id);