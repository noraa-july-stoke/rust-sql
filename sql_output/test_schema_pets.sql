CREATE SCHEMA IF NOT EXISTS test_schema;
CREATE TABLE IF NOT EXISTS test_schema.pets (
	id INT PRIMARY KEY,
	name VARCHAR(255) NOT NULL
);