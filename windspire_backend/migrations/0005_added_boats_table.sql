-- Create boats table
CREATE TABLE boats
(
  id          UUID NOT NULL,
  name        VARCHAR NOT NULL,
  brand       VARCHAR NULL,
  model       VARCHAR NULL,
  sail_number VARCHAR NULL,
  country_id UUID NOT NULL,
  PRIMARY KEY (id)
);

-- Add foreign constraint to countries table
ALTER TABLE boats
  ADD CONSTRAINT FK_countries_TO_boats
    FOREIGN KEY (country_id)
    REFERENCES countries (id);

