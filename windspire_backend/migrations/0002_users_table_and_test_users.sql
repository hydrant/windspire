-- Create users table
CREATE TABLE users
(
  id         UUID NOT NULL,
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
INSERT INTO users (id, first_name, last_name, email, phone, country_id)
VALUES
('01964081-4fbf-747a-ae64-d17030fc3dcc', 'Ove', 'Størholt', 'ovestoerholt@gmail.com', '93021759', '0196407f-574a-7061-a353-03f612af0766'),
('01964081-7e14-72dd-9039-4d9201218a92', 'Jan', 'Christensen', 'janchris2000@gmail.com', '41521014', '0196407f-574a-7061-a353-03f612af0766'),
('01964081-9dda-76cf-bffc-abb547244bd4', 'Kalle', 'Anka', 'kalle.anka@disney.se', NULL, '0196407f-85db-7ece-bab2-681b8c5320e6')
