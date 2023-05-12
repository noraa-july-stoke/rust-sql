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

fn generate_schema_table_column_sql(schema_name: &str, table_name: &str, columns: &[Column]) -> String {
    let mut sql = format!("CREATE SCHEMA IF NOT EXISTS {};\n", schema_name);
    sql += &format!("CREATE TABLE IF NOT EXISTS {}.{} (\n", schema_name, table_name);

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

fn main() {
    let schema_name = "my_schema";
    let table_name = "my_table";
    let columns = vec![
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
            name: "email".to_string(),
            data_type: "VARCHAR(255)".to_string(),
            constraints: Some("UNIQUE".to_string()),
        },
        Column {
            name: "created_at".to_string(),
            data_type: "TIMESTAMP".to_string(),
            constraints: Some("DEFAULT CURRENT_TIMESTAMP".to_string()),
        },
        Column {
            name: "updated_at".to_string(),
            data_type: "TIMESTAMP".to_string(),
            constraints: Some("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_string()),
        },
    ];

    let table = Table {
        schema_name: schema_name.to_string(),
        table_name: table_name.to_string(),
        columns,
    };

    let sql = generate_schema_table_column_sql(&table.schema_name, &table.table_name, &table.columns);

    println!("The following SQL was generated: {}", sql);

    let output_file = "output.sql";
    let mut file = File::create(output_file).expect("Failed to create output file");
    file.write_all(sql.as_bytes()).expect("Failed to write SQL to file");

    println!("SQL saved to {}", output_file);
}
