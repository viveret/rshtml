use crate::core::type_info::TypeInfo;
use std::fmt;

#[derive(Clone, Debug)]
pub struct EntityTypeInfoColumn {
    pub name: String,
    pub sql_type: SqlType,
    pub rust_type: TypeInfo,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SqlType {
    Integer,
    Real,
    Text,
    Blob,
    Boolean,
    // Custom types can be added here
    Custom(String),
}

impl fmt::Display for SqlType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SqlType::Integer => write!(f, "INTEGER"),
            SqlType::Real => write!(f, "REAL"),
            SqlType::Text => write!(f, "TEXT"),
            SqlType::Blob => write!(f, "BLOB"),
            SqlType::Boolean => write!(f, "BOOLEAN"),
            SqlType::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EntityTypeInfo {
    pub type_info: TypeInfo,
    pub sql_table_name: String,
    pub sql_columns: Vec<EntityTypeInfoColumn>,
    pub indexes: Vec<EntityIndexInfo>,
}

#[derive(Clone, Debug)]
pub struct EntityIndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
}

impl EntityTypeInfo {
    /// Creates a new EntityTypeInfo with the given type information
    pub fn new<T: 'static>(table_name: &str) -> Self {
        Self {
            type_info: TypeInfo::of::<T>(),
            sql_table_name: table_name.to_string(),
            sql_columns: Vec::new(),
            indexes: Vec::new(),
        }
    }

    /// Adds a column to the entity definition
    pub fn add_column(
        &mut self,
        name: &str,
        sql_type: SqlType,
        rust_type: TypeInfo,
        is_primary_key: bool,
        is_nullable: bool,
        default_value: Option<&str>,
    ) -> &mut Self {
        self.sql_columns.push(EntityTypeInfoColumn {
            name: name.to_string(),
            sql_type,
            rust_type,
            is_primary_key,
            is_nullable,
            default_value: default_value.map(|s| s.to_string()),
        });
        self
    }

    /// Adds an index to the entity definition
    pub fn add_index(&mut self, name: &str, columns: &[&str], is_unique: bool) -> &mut Self {
        self.indexes.push(EntityIndexInfo {
            name: name.to_string(),
            columns: columns.iter().map(|s| s.to_string()).collect(),
            is_unique,
        });
        self
    }

    /// Generates the SQL CREATE TABLE statement
    pub fn generate_create_table_sql(&self) -> String {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", self.sql_table_name);

        let columns_sql: Vec<String> = self.sql_columns.iter().map(|col| {
            let mut col_sql = format!("    {} {}", col.name, col.sql_type);

            if col.is_primary_key {
                col_sql.push_str(" PRIMARY KEY");
                if matches!(col.sql_type, SqlType::Integer) {
                    col_sql.push_str(" AUTOINCREMENT");
                }
            }

            if !col.is_nullable && !col.is_primary_key {
                col_sql.push_str(" NOT NULL");
            }

            if let Some(default) = &col.default_value {
                col_sql.push_str(&format!(" DEFAULT {}", default));
            }

            col_sql
        }).collect();

        sql.push_str(&columns_sql.join(",\n"));
        sql.push_str("\n);");

        // Add indexes
        for index in &self.indexes {
            sql.push_str(&self.generate_index_sql(index));
        }

        sql
    }

    /// Generates SQL for an index
    pub fn generate_index_sql(&self, index: &EntityIndexInfo) -> String {
        let unique = if index.is_unique { "UNIQUE " } else { "" };
        let columns = index.columns.join(", ");
        format!(
            "\nCREATE {}INDEX IF NOT EXISTS {} ON {} ({});",
            unique,
            index.name,
            self.sql_table_name,
            columns
        )
    }

    /// Generates the SQL INSERT statement
    pub fn generate_insert_sql(&self) -> String {
        let columns: Vec<String> = self.sql_columns
            .iter()
            .filter(|c| !c.is_primary_key || !matches!(c.sql_type, SqlType::Integer))
            .map(|c| c.name.clone())
            .collect();

        let placeholders = vec!["?"; columns.len()].join(", ");

        format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.sql_table_name,
            columns.join(", "),
            placeholders
        )
    }

    /// Generates the SQL SELECT statement for all columns
    pub fn generate_select_all_sql(&self) -> String {
        let columns: Vec<String> = self.sql_columns
            .iter()
            .map(|c| c.name.clone())
            .collect();

        format!(
            "SELECT {} FROM {};",
            columns.join(", "),
            self.sql_table_name
        )
    }

    /// Generates the SQL UPDATE statement
    pub fn generate_update_sql(&self) -> String {
        let pk_column = self.sql_columns.iter()
            .find(|c| c.is_primary_key)
            .expect("Primary key column not found");

        let set_clauses: Vec<String> = self.sql_columns
            .iter()
            .filter(|c| !c.is_primary_key)
            .map(|c| format!("{} = ?", c.name))
            .collect();

        format!(
            "UPDATE {} SET {} WHERE {} = ?;",
            self.sql_table_name,
            set_clauses.join(", "),
            pk_column.name
        )
    }

    /// Generates the SQL DELETE statement
    pub fn generate_delete_sql(&self) -> String {
        let pk_column = self.sql_columns.iter()
            .find(|c| c.is_primary_key)
            .expect("Primary key column not found");

        format!(
            "DELETE FROM {} WHERE {} = ?;",
            self.sql_table_name,
            pk_column.name
        )
    }

    /// Finds a column by name
    pub fn find_column(&self, name: &str) -> Option<&EntityTypeInfoColumn> {
        self.sql_columns.iter().find(|c| c.name == name)
    }

    /// Gets the primary key column
    pub fn get_primary_key_column(&self) -> Option<&EntityTypeInfoColumn> {
        self.sql_columns.iter().find(|c| c.is_primary_key)
    }
}