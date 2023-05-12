mod sql_generators;


fn main() {
    let schema_name = "test_schema";

    let owners_table = generate_table!(
        "owners",
        generate_column!("id", "INT", "PRIMARY KEY"),
        generate_column!("name", "VARCHAR(255)", "NOT NULL")
    );

    let pets_table = generate_table!(
        "pets",
        generate_column!("id", "INT", "PRIMARY KEY"),
        generate_column!("name", "VARCHAR(255)", "NOT NULL")
    );

    let join_table = generate_table!(
        "owners_pets",
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

    println!("Generated SQL:");
    println!("{}", schema_sql);

    let owners_query = generate_query!("owners");
    let pets_query = generate_query!("pets");

    println!("Owners Query: {}", owners_query);
    println!("Pets Query: {}", pets_query);

    let owner1_insert = generate_insert!("owners", "1, 'John'");
    let owner2_insert = generate_insert!("owners", "2, 'Jane'");
    let pet1_insert = generate_insert!("pets", "1, 'Max'");
    let pet2_insert = generate_insert!("pets", "2, 'Bella'");

    println!("Owner1 Insert: {}", owner1_insert);
    println!("Owner2 Insert: {}", owner2_insert);
    println!("Pet1 Insert: {}", pet1_insert);
    println!("Pet2 Insert: {}", pet2_insert);
}
