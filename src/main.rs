use std::fmt;
use std::fs::File;
use std::io::Write;

struct Column {
    name: String,
    data_type: String,
    constraints: Option<String>,
}

struct Table {
    schema_name: String,
    table_name: String,
    columns: Vec<Column>,
}

fn generate_schema_table_column_sql(
    schema_name: &str,
    table_name: &str,
    columns: &[Column],
) -> String {
    let mut sql = format!("CREATE SCHEMA IF NOT EXISTS {};\n", schema_name);
    sql += &format!(
        "CREATE TABLE IF NOT EXISTS {}.{} (\n",
        schema_name, table_name
    );

    for column in columns {
        sql += &format!("\t{} {}", column.name, column.data_type);

        if let Some(constraints) = &column.constraints {
            sql += &format!(" {}", constraints);
        }

        sql += ",\n";
    }

    sql = sql.trim_end_matches(",\n").to_owned();
    sql += "\n);";

    sql
}

fn generate_many_to_many_relation_sql(
    schema_name: &str,
    table_name1: &str,
    table_name2: &str,
    join_table_name: &str,
) -> String {
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {}.{} (
            {}_id INT NOT NULL,
            {}_id INT NOT NULL,
            PRIMARY KEY ({}_id, {}_id),
            FOREIGN KEY ({}_id) REFERENCES {}.{} (id),
            FOREIGN KEY ({}_id) REFERENCES {}.{} (id)
        );",
        schema_name,
        join_table_name,
        table_name1,
        table_name2,
        table_name1,
        table_name2,
        table_name1,
        schema_name,
        table_name1,
        table_name2,
        schema_name,
        table_name2
    );

    sql
}

struct Owner {
    id: i32,
    name: String
}

struct Pet {
    id: i32,
    name: String,
    owner_id: i32,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Owner [id={}, name={}]",
            self.id, self.name
        )
    }
}

impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pet [id={}, name={}, owner_id={}]",
            self.id, self.name, self.owner_id
        )
    }
}

fn insert_owner(owner: &Owner) -> String {
    format!(
        "INSERT INTO test_schema.owners (id, name) VALUES ({}, '{}');",
        owner.id, owner.name
    )
}

fn insert_pet(pet: &Pet) -> String {
    format!(
        "INSERT INTO test_schema.pets (id, name, owner_id) VALUES ({}, '{}', {});",
        pet.id, pet.name, pet.owner_id
    )
}

fn generate_owners_query() -> String {
    "SELECT * FROM test_schema.owners;".to_string()
}

fn generate_pets_query() -> String {
    "SELECT * FROM test_schema.pets;".to_string()
}

fn main() {
    let schema_name = "test_schema";

    let owners_table_name = "owners";
    let owners_columns = vec![
        Column {
            name: "id".to_string(),
            data_type: "INT".to_string(),
            constraints: Some("PRIMARY KEY".to_string()),
        },
        Column {
            name: "name".to_string(),
            data_type: "VARCHAR(255)".to_string(),
            constraints: Some("NOT NULL".to_string()),
        },
    ];
    let owners_table = Table {
        schema_name: schema_name.to_string(),
        table_name: owners_table_name.to_string(),
        columns: owners_columns,
    };

    let pets_table_name = "pets";
    let pets_columns = vec![
        Column {
            name: "id".to_string(),
            data_type: "INT".to_string(),
            constraints: Some("PRIMARY KEY".to_string()),
        },
        Column {
            name: "name".to_string(),
            data_type: "VARCHAR(255)".to_string(),
            constraints: Some("NOT NULL".to_string()),
        },
        Column {
            name: "owner_id".to_string(),
            data_type: "INT".to_string(),
            constraints: Some("NOT NULL".to_string()),
        }
    ];
    let pets_table = Table {
        schema_name: schema_name.to_string(),
        table_name: pets_table_name.to_string(),
        columns: pets_columns,
    };

    let owners_pets_join_table_name = "owners_pets";
    let join_table_sql = generate_many_to_many_relation_sql(
        schema_name,
        owners_table_name,
        pets_table_name,
        owners_pets_join_table_name,
    );

    let owners_table_sql = generate_schema_table_column_sql(
        &owners_table.schema_name,
        &owners_table.table_name,
        &owners_table.columns,
    );
    let pets_table_sql = generate_schema_table_column_sql(
        &pets_table.schema_name,
        &pets_table.table_name,
        &pets_table.columns,
    );

    // Save the SQL statements to files
    let output_dir = "sql_output";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let owners_table_file = format!("{}/{}_{}.sql", output_dir, schema_name, owners_table_name);

    let mut owners_table_file =
        File::create(&owners_table_file).expect("Failed to create owners table file");
    owners_table_file
        .write_all(owners_table_sql.as_bytes())
        .expect("Failed to write owners table SQL to file");

    let pets_table_file = format!("{}/{}_{}.sql", output_dir, schema_name, pets_table_name);
    let mut pets_table_file =
        File::create(&pets_table_file).expect("Failed to create pets table file");
    pets_table_file
        .write_all(pets_table_sql.as_bytes())
        .expect("Failed to write pets table SQL to file");

    let join_table_file = format!(
        "{}/{}_{}.sql",
        output_dir, schema_name, owners_pets_join_table_name
    );
    let mut join_table_file =
        File::create(&join_table_file).expect("Failed to create join table file");
    join_table_file
        .write_all(join_table_sql.as_bytes())
        .expect("Failed to write join table SQL to file");

    println!("SQL files saved in {}", output_dir);
    // Example data for owners and pets
    let owners = vec![
        Owner {
            id: 1,
            name: "John".to_string(),
        },
        Owner {
            id: 2,
            name: "Jane".to_string(),
        },
    ];

    let pets = vec![
        Pet {
            id: 1,
            name: "Max".to_string(),
            owner_id: 1,
        },
        Pet {
            id: 2,
            name: "Bella".to_string(),
            owner_id: 2,
        },
    ];

    // Insert owners
    for owner in &owners {
        let insert_query = insert_owner(owner);
        println!("Insert Query: {}", insert_query);
        // Execute the insert query
    }

    println!();

    // Insert pets
    for pet in &pets {
        let insert_query = insert_pet(pet);
        println!("Insert Query: {}", insert_query);
        // Execute the insert query
    }

    println!();


    // Insert owners-pets relationship into the join table
    for pet in &pets {
        for owner in &owners {
            if pet.owner_id == owner.id {
                let insert_query = format!(
                    "INSERT INTO test_schema.owners_pets (owners_id, pets_id) VALUES ({}, {});",
                    owner.id, pet.id
                );
                println!("Insert Query (Owners-Pets): {}", insert_query);
                // Execute the insert query
            }
        }
    }

    println!();

    // Generate owners query
    let owners_query = generate_owners_query();
    println!("Owners Query: {}", owners_query);

    println!();

    // Generate pets query
    let pets_query = generate_pets_query();
    println!("Pets Query: {}", pets_query);
        // Save the queries to a file
    let queries_output_file = "sql_output/queries.sql";
    let mut queries_file = File::create(queries_output_file).expect("Failed to create queries file");

    queries_file.write_all(owners_query.as_bytes()).expect("Failed to write owners query to file");
    queries_file.write_all(pets_query.as_bytes()).expect("Failed to write pets query to file");

    // Save the inserts to a file
    let inserts_output_file = "sql_output/inserts.sql";
    let mut inserts_file = File::create(inserts_output_file).expect("Failed to create inserts file");

    for owner in &owners {
        let insert_query = insert_owner(owner);
        inserts_file.write_all(insert_query.as_bytes()).expect("Failed to write owner insert query to file");
    }

    for pet in &pets {
        let insert_query = insert_pet(pet);
        inserts_file.write_all(insert_query.as_bytes()).expect("Failed to write pet insert query to file");
    }

    println!("Queries saved in the 'output/queries.sql' file");
    println!("Inserts saved in the 'output/inserts.sql' file");
}
