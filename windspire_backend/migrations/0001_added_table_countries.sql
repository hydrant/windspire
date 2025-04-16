CREATE TABLE countries
(
  id          UUID NOT NULL,
  iso_name    VARCHAR NOT NULL,
  iso_alpha_2 VARCHAR NOT NULL,
  iso_alpha_3 VARCHAR NOT NULL,
  PRIMARY KEY (id)
);

INSERT INTO countries (id, iso_name, iso_alpha_2, iso_alpha_3)
VALUES
('0196407f-574a-7061-a353-03f612af0766', 'Norway', 'NO', 'NOR'),
('0196407f-85db-7ece-bab2-681b8c5320e6', 'Sweden', 'SE', 'SWE'),
('0196407f-c2b8-7adf-903a-4054353bc9d8', 'Denmark', 'DK', 'DNK'),
('0196407f-ed7e-7007-8409-daf224f801bf', 'Germany', 'DE', 'DEU'),
('01964080-1cad-7c68-9ee5-410e421f69e7', 'United Kingdom', 'GB', 'GBR');
