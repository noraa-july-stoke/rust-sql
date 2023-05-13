CREATE SCHEMA IF NOT EXISTS test_schema;
CREATE TABLE IF NOT EXISTS test_schema.owners (
	id INT PRIMARY KEY ,
	name VARCHAR(255) NOT NULL
);
CREATE TABLE IF NOT EXISTS test_schema.pets (
	id INT PRIMARY KEY ,
	name VARCHAR(255) NOT NULL
);
CREATE TABLE IF NOT EXISTS test_schema.owners_pets (
	owners_id INT NOT NULL ,
	pets_id INT NOT NULL ,
PRIMARY KEY (owners_id, pets_id),
            FOREIGN KEY (owners_id) REFERENCES test_schema.owners (id),
            FOREIGN KEY (pets_id) REFERENCES test_schema.pets (id)
);

SELECT * FROM test_schema.owners;
SELECT * FROM test_schema.pets;
INSERT INTO test_schema.owners VALUES (1, 'John');
INSERT INTO test_schema.owners VALUES (2, 'Jane');
INSERT INTO test_schema.pets VALUES (1, 'Max');
INSERT INTO test_schema.pets VALUES (2, 'Bella');
