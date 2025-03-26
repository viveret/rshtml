use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Meta, Lit};


/// Attribute to mark a struct as an entity and its fields as columns
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let table_name = if args.is_empty() {
        None
    } else {
        Some(format!("{:?}", parse_macro_input!(args as Lit)))
    };

    let mut input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Process fields to extract column metadata
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let mut column_info = Vec::new();
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Default column attributes
        let mut column_name = field_name.to_string();
        let mut sql_type = None;
        let mut is_primary_key = false;
        let mut is_nullable = false;
        let mut default_value = None;

        // Process field attributes
        for attr in &field.attrs {
            if attr.path().is_ident("column") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let lit: Lit = value.parse()?;
                        if let Lit::Str(s) = lit {
                            column_name = s.value();
                        }
                    } else if meta.path.is_ident("sql_type") {
                        let value = meta.value()?;
                        let lit: Lit = value.parse()?;
                        if let Lit::Str(s) = lit {
                            sql_type = Some(s.value());
                        }
                    } else if meta.path.is_ident("primary_key") {
                        is_primary_key = true;
                    } else if meta.path.is_ident("nullable") {
                        is_nullable = true;
                    } else if meta.path.is_ident("default") {
                        let value = meta.value()?;
                        let lit: Lit = value.parse()?;
                        if let Lit::Str(s) = lit {
                            default_value = Some(s.value());
                        }
                    }
                    Ok(())
                }).unwrap();
            }
        }

        // Infer SQL type if not specified
        let sql_type = sql_type.unwrap_or_else(|| infer_sql_type(field_type));

        column_info.push(quote! {
            EntityTypeInfoColumn {
                name: #column_name.to_string(),
                sql_type: #sql_type,
                rust_type: TypeInfo::of::<#field_type>(),
                is_primary_key: #is_primary_key,
                is_nullable: #is_nullable,
                default_value: #default_value,
            }
        });

        field_names.push(field_name);
        field_types.push(field_type);
    }

    // Generate the table name if not provided
    let table_name = table_name.unwrap_or_else(|| struct_name.to_string().to_lowercase());

    // Generate the impl block for the struct
    let expanded = quote! {
        #input

        impl #struct_name {
            /// Returns the EntityTypeInfo for this entity
            pub fn entity_type_info() -> EntityTypeInfo {
                EntityTypeInfo {
                    type_info: TypeInfo::of::<Self>(),
                    sql_table_name: #table_name.to_string(),
                    sql_columns: vec![#(#column_info),*],
                    indexes: vec![],
                }
            }

            /// Returns a map of field names to their type information
            pub fn field_info() -> std::collections::HashMap<&'static str, TypeInfo> {
                let mut map = std::collections::HashMap::new();
                #(
                    map.insert(stringify!(#field_names), TypeInfo::of::<#field_types>());
                )*
                map
            }
        }
    };

    TokenStream::from(expanded)
}

/// Infer SQL type from Rust type
fn infer_sql_type(ty: &Type) -> String {
    let type_str = quote!(#ty).to_string();
    match type_str.as_str() {
        "i32" | "i64" | "u32" | "u64" | "usize" | "isize" => "SqlType::Integer".to_string(),
        "f32" | "f64" => "SqlType::Real".to_string(),
        "bool" => "SqlType::Boolean".to_string(),
        "String" | "&str" | "str" => "SqlType::Text".to_string(),
        "Vec<u8>" => "SqlType::Blob".to_string(),
        _ => format!("SqlType::Custom(\"{}\")", type_str),
    }
}