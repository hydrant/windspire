-- Create users table
CREATE TABLE users
(
  id         UUID DEFAULT gen_random_uuid() NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name  VARCHAR NOT NULL,
  email      VARCHAR NOT NULL,
  phone      VARCHAR NULL,
  country_id UUID NOT NULL,
  PRIMARY KEY (id)
);

COMMENT ON COLUMN users.email IS 'login name';

-- Add foreign constraint to countries table
ALTER TABLE users
  ADD CONSTRAINT FK_countries_TO_users
    FOREIGN KEY (country_id)
    REFERENCES countries (id);

-- add users
INSERT INTO users (first_name, last_name, email, phone, country_id)
VALUES
('Ove', 'Størholt', 'ovestoerholt@gmail.com', '93021759', 'ddba3f2c-3c65-47e6-a15b-ae9a246e8fad'),
('Jan', 'Christensen', 'janchris2000@gmail.com', '41521014', 'ddba3f2c-3c65-47e6-a15b-ae9a246e8fad'),
('Kalle', 'Anka', 'kalle.anka@disney.se', NULL, '75b559f5-4466-4548-a033-e9dbc8901a57')
