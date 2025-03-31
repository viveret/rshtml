

pub trait SqlModel: Sized {
    /// Returns the SQL for creating the table
    fn create_table_sql() -> &'static str;
    
    /// Returns the SQL for inserting a new record
    fn insert_sql() -> &'static str;
    
    /// Returns the SQL for selecting a single record by primary key
    fn select_sql() -> &'static str;
    
    /// Returns the SQL for selecting all records
    fn select_all_sql() -> &'static str;
    
    /// Returns the SQL for updating a record
    fn update_sql() -> &'static str;
    
    /// Returns the SQL for deleting a record
    fn delete_sql() -> &'static str;
    
    /// Optional: Provides access to the primary key
    fn id(&self) -> i64;
    
    /// Optional: Sets the primary key
    fn set_id(&mut self, id: i64);
}