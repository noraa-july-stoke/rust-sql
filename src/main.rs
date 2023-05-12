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
}
