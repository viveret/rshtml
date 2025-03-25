use rusqlite::ToSql;



pub trait IHazSqlId {
    fn get_sql_id(&self) -> Box<dyn ToSql + '_>;
}