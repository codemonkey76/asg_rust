CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    email_verified_at TIMESTAMP,
    password VARCHAR NOT NULL,
    customer_id INT,
    customer_name VARCHAR,
    remember_token VARCHAR,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    CONSTRAINT fk_customer FOREIGN KEY (customer_id) REFERENCES customers (id) ON DELETE SET NULL
);
