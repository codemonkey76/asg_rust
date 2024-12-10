-- sqlx-up
ALTER TABLE customers
ADD CONSTRAINT fk_preferred_contact FOREIGN KEY (preferred_contact_id) REFERENCES contacts (id) ON DELETE SET NULL;



-- sqlx-down
ALTER TABLE customers
DROP CONSTRAINT IF EXISTS fk_preferred_contact;
