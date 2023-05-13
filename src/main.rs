mod sql_generators;
use std::fs::File;
use std::io::Write;

fn main() {
    let schema_name = "test_schema";

    let owners_table = generate_table!(
        format!("{}.owners", schema_name),
        generate_column!("id", "INT", "PRIMARY KEY"),
        generate_column!("name", "VARCHAR(255)", "NOT NULL"),
        generate_column!("email", "VARCHAR(255)", "NOT NULL")
    );

    let pets_table = generate_table!(
        format!( "{}.pets", schema_name),
        generate_column!("id", "INT", "PRIMARY KEY"),
        generate_column!("name", "VARCHAR(255)", "NOT NULL"),
        generate_column!("food_type", "VARCHAR(255)", "NOT NULL")
    );

    let join_table = generate_table!(
        format!("{}.owners_pets", schema_name),
        generate_column!("owners_id", "INT", "NOT NULL"),
        generate_column!("pets_id", "INT", "NOT NULL"),
        format!(
            "PRIMARY KEY (owners_id, pets_id),
            FOREIGN KEY (owners_id) REFERENCES {}.{} (id),
            FOREIGN KEY (pets_id) REFERENCES {}.{} (id)",
            schema_name, "owners", schema_name, "pets"
        )
    );

    let schema_sql = generate_schema!(schema_name, owners_table, pets_table, join_table);




    let owner1_insert = generate_insert!(schema_name, "owners", "1, 'John', 'john@john.com'");
    let owner2_insert = generate_insert!(schema_name, "owners", "2, 'Jane', 'jane@jane.com'");
    let pet1_insert = generate_insert!(schema_name, "pets", "1, 'Max', 'purina'");
    let pet2_insert = generate_insert!(schema_name, "pets", "2, 'Bella', 'meow mix'");
    let owner1_pet1_insert = generate_insert!(schema_name, "owners_pets", "1, 1");
    let owner2_pet2_insert = generate_insert!(schema_name, "owners_pets", "2, 2");


    let owners_query = generate_query!(schema_name, "owners");
    let pets_query = generate_query!(schema_name, "pets");
    let owners_query_where = generate_query!(schema_name, "owners", "id = 1");
    let join_query_1 = generate_query!(schema_name, "owners", "owners_pets", "owners.id = owners_pets.owners_id");
    let join_query_2 = generate_query!(schema_name, "owners", "owners_pets", "pets", "owners.id = owners_pets.owners_id", "pets.id = owners_pets.pets_id");



    let sql_statements = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        schema_sql,
        owners_query,
        pets_query,
        owner1_insert,
        owner2_insert,
        pet1_insert,
        pet2_insert,
        owner1_pet1_insert,
        owner2_pet2_insert,
        owners_query_where,
        join_query_1,
        join_query_2
    );

    // Create the 'generated_sql' directory if it doesn't exist
    std::fs::create_dir_all("generated_sql").expect("Failed to create 'generated_sql' directory");

    // Write the SQL statements to a file
    let sql_file_path = "generated_sql/sql_statements.sql";
    let mut sql_file = File::create(sql_file_path).expect("Failed to create SQL file");
    sql_file.write_all(sql_statements.as_bytes()).expect("Failed to write SQL statements to file");

    println!("SQL statements saved in '{}'", sql_file_path);
}
