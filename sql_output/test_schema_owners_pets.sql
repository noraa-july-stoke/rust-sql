CREATE TABLE IF NOT EXISTS test_schema.owners_pets (
            owners_id INT NOT NULL,
            pets_id INT NOT NULL,
            PRIMARY KEY (owners_id, pets_id),
            FOREIGN KEY (owners_id) REFERENCES test_schema.owners (id),
            FOREIGN KEY (pets_id) REFERENCES test_schema.pets (id)
        );
