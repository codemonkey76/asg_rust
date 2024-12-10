-- sqlx-up

-- Create a global function for updating `updated_at`
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- sqlx-down

-- Drop the global function
DROP FUNCTION IF EXISTS set_updated_at;
