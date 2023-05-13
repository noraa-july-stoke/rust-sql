#[macro_export]
macro_rules! generate_schema {
    ($schema_name:expr, $($table:expr),*) => {
        {
            let mut sql = format!("CREATE SCHEMA IF NOT EXISTS {};\n", $schema_name);
            $(sql += &$table;)*
            sql
        }
    };
}

#[macro_export]
macro_rules! generate_table {
    ($table_name:expr, $($column:expr),*) => {
        {
            let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", $table_name);
            $(sql += &$column;)*
            sql.trim_end_matches(",\n").to_owned() + "\n);\n"
        }
    };
}


#[macro_export]
macro_rules! generate_column {
    ($column_name:expr, $data_type:expr, $($constraint:expr),*) => {
        {
            let mut sql = format!("\t{} {} ", $column_name, $data_type);
            $(sql += &format!("{} ", $constraint);)*
            sql.trim_end_matches(",\n").trim_end_matches(", ").to_owned() + ",\n"
        }
    };
}


#[macro_export]
macro_rules! generate_query {
    ($schema_name:expr, $table_name:expr) => {
        format!("SELECT * FROM {}.{};", $schema_name, $table_name)
    };
}

#[macro_export]
macro_rules! generate_insert {
    ($schema_name:expr, $table_name:expr, $($value:expr),*) => {
        format!("INSERT INTO {}.{} VALUES ({});", $schema_name, $table_name, $($value),*)
    };
}
