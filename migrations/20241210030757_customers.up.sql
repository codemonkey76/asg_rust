CREATE TABLE IF NOT EXISTS customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address VARCHAR,
    address_2 VARCHAR,
    suburb VARCHAR,
    state VARCHAR,
    postcode VARCHAR,
    preferred_contact_id INT, -- Default contact person
    terms INT NOT NULL DEFAULT 15, -- Terms in days
    credit_limit INT,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    archived_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Attach the global trigger function to the `customers` table
CREATE TRIGGER update_updated_at
BEFORE UPDATE ON customers
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
