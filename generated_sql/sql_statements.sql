CREATE SCHEMA IF NOT EXISTS test_schema;
CREATE TABLE IF NOT EXISTS test_schema.owners (
	id INT PRIMARY KEY,
	name VARCHAR(255)NOT NULL
);CREATE TABLE IF NOT EXISTS test_schema.pets (
	id INT PRIMARY KEY,
	name VARCHAR(255)NOT NULL
);CREATE TABLE IF NOT EXISTS test_schema.owners_pets (
	owners_id INTNOT NULL,
	pets_id INTNOT NULL,
PRIMARY KEY (owners_id, pets_id),
            FOREIGN KEY (owners_id) REFERENCES test_schema.owners (id),
            FOREIGN KEY (pets_id) REFERENCES test_schema.pets (id)
);
SELECT * FROM owners;
SELECT * FROM pets;
INSERT INTO owners VALUES (1, 'John');
INSERT INTO owners VALUES (2, 'Jane');
INSERT INTO pets VALUES (1, 'Max');
INSERT INTO pets VALUES (2, 'Bella');
