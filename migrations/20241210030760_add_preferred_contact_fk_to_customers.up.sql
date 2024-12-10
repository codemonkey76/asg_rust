ALTER TABLE customers
ADD CONSTRAINT fk_preferred_contact FOREIGN KEY (preferred_contact_id) REFERENCES contacts (id) ON DELETE SET NULL;
