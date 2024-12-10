-- sqlx-up
CREATE TABLE IF NOT EXISTS contacts (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR,
    last_name VARCHAR,
    position VARCHAR,
    phone VARCHAR,
    email VARCHAR,
    customer_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_customer FOREIGN KEY (customer_id) REFERENCES customers (id) ON DELETE CASCADE,
    CONSTRAINT check_at_least_one_not_null CHECK (
        first_name IS NOT NULL OR
        last_name IS NOT NULL OR
        phone IS NOT NULL OR
        email IS NOT NULL 
    )
);

-- Create a trigger function to update `updated_at`
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach the trigger to the `contacts` table
CREATE TRIGGER update_updated_at
BEFORE UPDATE on contacts
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

-- sqlx-down
DROP TABLE IF EXISTS contacts;
