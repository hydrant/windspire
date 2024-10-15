CREATE TABLE countries
(
  id          UUID DEFAULT gen_random_uuid() NOT NULL,
  iso_name    VARCHAR NOT NULL,
  iso_alpha_2 VARCHAR NOT NULL,
  iso_alpha_3 VARCHAR NOT NULL,
  PRIMARY KEY (id)
);

INSERT INTO countries (id, iso_name, iso_alpha_2, iso_alpha_3)
VALUES
('ddba3f2c-3c65-47e6-a15b-ae9a246e8fad', 'Norway', 'NO', 'NOR'),
('75b559f5-4466-4548-a033-e9dbc8901a57', 'Sweden', 'SE', 'SWE'),
('d575310d-490c-4cdf-838f-f71140c497ab', 'Denmark', 'DK', 'DNK'),
('5c450ad0-c783-47a8-a76b-03c24e317194', 'Germany', 'DE', 'DEU'),
('44d6a316-eb51-4438-b0b8-2dcd66a24d0f', 'United Kingdom', 'GB', 'GBR');
