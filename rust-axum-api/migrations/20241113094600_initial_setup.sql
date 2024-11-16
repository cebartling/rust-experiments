-- Enable UUID extension
CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create updated_at function
CREATE
OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at
= CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$
LANGUAGE plpgsql;

