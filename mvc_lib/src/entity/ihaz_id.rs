// use sqlx::ToSql;



pub trait IHazSqlId {
    fn get_sql_id(&self) -> Box<String>;
}