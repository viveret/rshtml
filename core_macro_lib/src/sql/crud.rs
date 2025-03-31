use std::matches;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type, Ident, Attribute};

pub fn create_table_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Get table name from attribute or default to struct name in snake_case
    let table_name = get_table_name(&input);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("GenerateTable only works with structs with named fields"),
        },
        _ => panic!("GenerateTable only works with structs"),
    };

    let columns: Vec<_> = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let name_str = name.to_string();
        
        // Get SQL type
        let sql_type = map_type_to_sql(&field.ty);
        
        // Check for primary key attribute
        let is_primary = field.attrs.iter().any(|attr| {
            attr.path().is_ident("primary_key")
        });
        
        // Check for nullable (Option<T>)
        let is_nullable = matches!(&field.ty, Type::Path(type_path) if 
            type_path.path.segments.last().unwrap().ident == "Option");
        
        // Build column definition
        let mut column_def = format!("{} {}", name_str, sql_type);
        
        if is_primary {
            column_def.push_str(" PRIMARY KEY");
        } else if !is_nullable {
            column_def.push_str(" NOT NULL");
        }
        
        column_def
    }).collect();

    let columns_sql = columns.join(",\n    ");

    let create_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
        table_name,
        columns_sql
    );

    let expanded = quote! {
        impl #struct_name {
            pub fn create_table_sql() -> &'static str {
                #create_table_sql
            }
        }
    };

    TokenStream::from(expanded)
}

// fn get_table_name(input: &DeriveInput) -> String {
//     input.attrs.iter()
//         .find(|attr| attr.path().is_ident("table"))
//         .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
//         .map(|lit| lit.value())
//         .unwrap_or_else(|| {
//             // Convert PascalCase to snake_case
//             let name = input.ident.to_string();
//             let mut snake = String::new();
//             for (i, c) in name.char_indices() {
//                 if i > 0 && c.is_uppercase() {
//                     snake.push('_');
//                 }
//                 snake.push(c.to_ascii_lowercase());
//             }
//             snake
//         })
// }

// fn map_type_to_sql(ty: &Type) -> String {
//     match ty {
//         Type::Path(type_path) => {
//             let type_name = type_path.path.segments.last().unwrap().ident.to_string();
//             match type_name.as_str() {
//                 "i32" => "INTEGER".to_string(),
//                 "i64" => "BIGINT".to_string(),
//                 "f32" | "f64" => "REAL".to_string(),
//                 "bool" => "BOOLEAN".to_string(),
//                 "String" => "TEXT".to_string(),
//                 "Option" => {
//                     if let syn::PathArguments::AngleBracketed(args) = &type_path.path.segments.last().unwrap().arguments {
//                         if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
//                             return map_type_to_sql(inner_ty);
//                         }
//                     }
//                     "TEXT".to_string()
//                 }
//                 _ => "TEXT".to_string(),
//             }
//         }
//         _ => "TEXT".to_string(),
//     }
// }











pub fn sql_crud_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    
    // Get table metadata
    let table_meta = TableMeta::from_derive_input(&input);
    
    // Generate all SQL operations
    let create_table = generate_create_table(&table_meta);
    let insert = generate_insert(&table_meta);
    let select = generate_select(&table_meta);
    let update = generate_update(&table_meta);
    let delete = generate_delete(&table_meta);
    
    // Combine all generated code
    let expanded = quote! {
        impl #struct_name {
            #create_table
            #insert
            #select
            #update
            #delete
        }
    };
    
    TokenStream::from(expanded)
}

// Helper structs and functions
struct TableMeta {
    name: String,
    columns: Vec<ColumnMeta>,
    primary_keys: Vec<String>,
}

struct ColumnMeta {
    name: String,
    rust_name: Ident,
    sql_type: String,
    is_primary: bool,
    is_nullable: bool,
}

impl TableMeta {
    fn from_derive_input(input: &DeriveInput) -> Self {
        let name = get_table_name(input);
        let mut columns = Vec::new();
        let mut primary_keys = Vec::new();
        
        if let Data::Struct(data) = &input.data {
            if let Fields::Named(fields) = &data.fields {
                for field in &fields.named {
                    let column = ColumnMeta::from_field(field);
                    if column.is_primary {
                        primary_keys.push(column.name.clone());
                    }
                    columns.push(column);
                }
            }
        }
        
        Self { name, columns, primary_keys }
    }
}

impl ColumnMeta {
    fn from_field(field: &syn::Field) -> Self {
        let rust_name = field.ident.as_ref().unwrap().clone();
        let name = get_column_name(field);
        let sql_type = map_type_to_sql(&field.ty);
        let is_primary = field.attrs.iter().any(|a| a.path().is_ident("primary_key"));
        let is_nullable = is_option_type(&field.ty);
        
        Self {
            name,
            rust_name,
            sql_type,
            is_primary,
            is_nullable,
        }
    }
}

// Generation functions
fn generate_create_table(table: &TableMeta) -> proc_macro2::TokenStream {
    let columns: Vec<_> = table.columns.iter().map(|col| {
        let mut def = format!("{} {}", col.name, col.sql_type);
        if col.is_primary {
            def.push_str(" PRIMARY KEY");
        } else if !col.is_nullable {
            def.push_str(" NOT NULL");
        }
        def
    }).collect();
    
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
        table.name,
        columns.join(",\n    ")
    );
    
    quote! {
        pub fn create_table_sql() -> &'static str {
            #sql
        }
    }
}

fn generate_insert(table: &TableMeta) -> proc_macro2::TokenStream {
    let columns: Vec<_> = table.columns.iter()
        .filter(|c| !c.is_primary || !is_auto_increment(&c.sql_type))
        .map(|c| c.name.as_str())
        .collect();
    
    let placeholders = vec!["?"; columns.len()].join(", ");
    
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table.name,
        columns.join(", "),
        placeholders
    );
    
    quote! {
        pub fn insert_sql() -> &'static str {
            #sql
        }
    }
}

fn generate_select(table: &TableMeta) -> proc_macro2::TokenStream {
    let table_name = &table.name;
    let columns = table.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ");
    let where_clause = if !table.primary_keys.is_empty() {
        let conditions = table.primary_keys.iter().map(|pk| format!("{} = ?", pk)).collect::<Vec<_>>();
        format!(" WHERE {}", conditions.join(" AND "))
    } else {
        String::new()
    };
    
    let sql = format!(
        "SELECT {} FROM {}{};",
        columns,
        table_name,
        where_clause
    );
    
    quote! {
        pub fn select_sql() -> &'static str {
            #sql
        }
        
        pub fn select_all_sql() -> &'static str {
            concat!("SELECT ", #columns, " FROM ", #table_name, ";")
        }
    }
}

fn generate_update(table: &TableMeta) -> proc_macro2::TokenStream {
    let set_clause = table.columns.iter()
        .filter(|c| !c.is_primary)
        .map(|c| format!("{} = ?", c.name))
        .collect::<Vec<_>>()
        .join(", ");
    
    let where_clause = if !table.primary_keys.is_empty() {
        let conditions = table.primary_keys.iter().map(|pk| format!("{} = ?", pk)).collect::<Vec<_>>();
        format!(" WHERE {}", conditions.join(" AND "))
    } else {
        String::new()
    };
    
    let sql = format!(
        "UPDATE {} SET {}{};",
        table.name,
        set_clause,
        where_clause
    );
    
    quote! {
        pub fn update_sql() -> &'static str {
            #sql
        }
    }
}

fn generate_delete(table: &TableMeta) -> proc_macro2::TokenStream {
    let where_clause = if !table.primary_keys.is_empty() {
        let conditions = table.primary_keys.iter().map(|pk| format!("{} = ?", pk)).collect::<Vec<_>>();
        format!(" WHERE {}", conditions.join(" AND "))
    } else {
        String::new()
    };
    
    let sql = format!(
        "DELETE FROM {}{};",
        table.name,
        where_clause
    );
    
    quote! {
        pub fn delete_sql() -> &'static str {
            #sql
        }
    }
}

// Helper functions
fn get_table_name(input: &DeriveInput) -> String {
    input.attrs.iter()
        .find(|attr| attr.path().is_ident("table"))
        .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
        .map(|lit| lit.value())
        .unwrap_or_else(|| to_snake_case(&input.ident.to_string()))
}

fn get_column_name(field: &syn::Field) -> String {
    field.attrs.iter()
        .find(|attr| attr.path().is_ident("column"))
        .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
        .map(|lit| lit.value())
        .unwrap_or_else(|| to_snake_case(&field.ident.as_ref().unwrap().to_string()))
}

fn map_type_to_sql(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();
            match type_name.as_str() {
                "i32" => "INTEGER",
                "i64" => "BIGINT",
                "f32" => "REAL",
                "f64" => "DOUBLE",
                "bool" => "BOOLEAN",
                "String" => "TEXT",
                "Vec" => "BLOB",
                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) = &type_path.path.segments.last().unwrap().arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            return map_type_to_sql(inner_ty);
                        }
                    }
                    "TEXT"
                }
                _ => "TEXT",
            }.to_string()
        }
        _ => "TEXT".to_string(),
    }
}

fn is_option_type(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) 
        if type_path.path.segments.last().unwrap().ident == "Option")
}

fn is_auto_increment(sql_type: &str) -> bool {
    sql_type.contains("SERIAL") || sql_type.contains("AUTOINCREMENT")
}

fn to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, c) in s.char_indices() {
        if i > 0 && c.is_uppercase() {
            snake.push('_');
        }
        snake.push(c.to_ascii_lowercase());
    }
    snake
}



// #[proc_macro_derive(SqlModel, attributes(table, column))]
pub fn sql_model_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let table_meta = TableMeta::from_derive_input(&input);

    let create_table = generate_create_table(&table_meta);
    let insert = generate_insert(&table_meta);
    let select = generate_select(&table_meta);
    let update = generate_update(&table_meta);
    let delete = generate_delete(&table_meta);

    // Generate field names for FromRow
    let field_names = table_meta.columns.iter()
        .map(|c| &c.rust_name)
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[derive(sqlx::FromRow)]
        #input

        impl #struct_name {
            #create_table
            #insert
            #select
            #update
            #delete

            pub async fn insert(&self, pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
                sqlx::query_as(Self::insert_sql())
                    .bind(#(self.#field_names),*)
                    .fetch_one(pool)
                    .await
            }

            pub async fn find_by_id(id: i64, pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
                sqlx::query_as(Self::select_sql())
                    .bind(id)
                    .fetch_one(pool)
                    .await
            }

            pub async fn update(&self, pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
                sqlx::query_as(Self::update_sql())
                    .bind(#(self.#field_names),*)
                    .bind(self.id) // Assuming primary key is named 'id'
                    .fetch_one(pool)
                    .await
            }

            pub async fn delete(id: i64, pool: &sqlx::PgPool) -> Result<u64, sqlx::Error> {
                sqlx::query(Self::delete_sql())
                    .bind(id)
                    .execute(pool)
                    .await
                    .map(|r| r.rows_affected())
            }
        }
    };

    TokenStream::from(expanded)
}