CREATE SCHEMA IF NOT EXISTS test_schema;
CREATE TABLE IF NOT EXISTS test_schema.owners (
	id INT PRIMARY KEY ,
	name VARCHAR(255) NOT NULL ,
	email VARCHAR(255) NOT NULL
);
CREATE TABLE IF NOT EXISTS test_schema.pets (
	id INT PRIMARY KEY ,
	name VARCHAR(255) NOT NULL ,
	food_type VARCHAR(255) NOT NULL
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
INSERT INTO test_schema.owners VALUES (1, 'John', 'john@john.com');
INSERT INTO test_schema.owners VALUES (2, 'Jane', 'jane@jane.com');
INSERT INTO test_schema.pets VALUES (1, 'Max', 'purina');
INSERT INTO test_schema.pets VALUES (2, 'Bella', 'meow mix');
INSERT INTO test_schema.owners_pets VALUES (1, 1);
INSERT INTO test_schema.owners_pets VALUES (2, 2);
SELECT * FROM test_schema.owners WHERE id = 1;
SELECT * FROM test_schema.owners JOIN test_schema.owners_pets ON owners.id = owners_pets.owners_id;
SELECT * FROM test_schema.owners JOIN test_schema.owners_pets ON owners.id = owners_pets.owners_id JOIN test_schema.pets ON pets.id = owners_pets.pets_id;
