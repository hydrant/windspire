CREATE TABLE countries
(
  id          INT     NOT NULL,
  iso_name    VARCHAR NOT NULL,
  iso_alpha_2 VARCHAR NOT NULL,
  iso_alpha_3 VARCHAR NOT NULL,
  PRIMARY KEY (id)
);

INSERT INTO countries (id, iso_name, iso_alpha_2, iso_alpha_3)
VALUES
(1, 'Norway', 'NO', 'NOR'),
(2, 'Sweden', 'SE', 'SWE'),
(3, 'Denmark', 'DK', 'DNK'),
(4, 'Germany', 'DE', 'DEU'),
(5, 'United Kingdom', 'GB', 'GBR');
