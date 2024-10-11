-- Create users table
CREATE TABLE users
(
  id         INT     NOT NULL,
  country_id INT     NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name  VARCHAR NOT NULL,
  email      VARCHAR NOT NULL,
  phone      VARCHAR NULL,
  PRIMARY KEY (id)
);

COMMENT ON COLUMN users.email IS 'login name';

-- Add foreign constraint to countries table
ALTER TABLE users
  ADD CONSTRAINT FK_countries_TO_users
    FOREIGN KEY (country_id)
    REFERENCES countries (id);

-- add users
INSERT INTO users (id, country_id, first_name, last_name, email, phone)
VALUES
(1, 1, 'Ove', 'Størholt', 'ovestoerholt@gmail.com', '93021759'),
(2, 1, 'Jan', 'Christensen', 'janchris2000@gmail.com', '41521014'),
(3, 2, 'Kalle', 'Anka', 'kalle.anka@disney.se', null)
