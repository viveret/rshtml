use std::any::Any;

use core_macro_lib::SqlCrud;
use sqlx::prelude::FromRow;

use crate::auth::iauth_role::IAuthRole;

use crate::core::type_info::TypeInfo;

use crate::entity::idbset::{ IDbSet, IDbSetAny };
use crate::entity::ihaz_id::IHazSqlId;
use crate::entity::sql_dbset::SqlFileDbSet;

// this struct is used to store a single role in the authrole_dbset.json file
#[derive(Clone, Debug, FromRow, SqlCrud)]
pub struct SqlAuthRole {
    // the name of the role.
    pub name: String,
}

impl SqlAuthRole {
    // this is used to create a new SqlAuthRole struct.
    // returns a SqlAuthRole struct.
    pub fn new() -> Self {
        Self {
            name: "".to_string()
        }
    }

    // this is used to parse a string from the sql row.
    // returns a SqlAuthRole struct
    // pub fn parse_sql(v: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
    //     Ok(Self {
    //         name: v.get::<usize, String>(0)?.to_string()
    //     })
    // }

    // this is used to convert a SqlAuthRole struct to a sql::Row.
    // v: the SqlAuthRole struct to convert.
    // returns a vec of sql::ToSql.
    // pub fn to_sql(&self) -> Vec<Box<dyn rusqlite::ToSql + '_>> {
    //     vec![Box::new(self.name.to_sql().unwrap())]
    // }

    // For conversion to SQL parameters, sqlx uses query macros with bind parameters
    // pub async fn insert(&self, pool: &sqlx::AnyPool) -> Result<(), sqlx::Error> {
    //     sqlx::query!("INSERT INTO auth_roles (name) VALUES (?)", self.name)
    //         .execute(pool)
    //         .await?;
    //     Ok(())
    // }
}

impl IHazSqlId for SqlAuthRole {
    fn get_sql_id(&self) -> Box<String> {
        Box::new(self.name.clone())
    }
}

impl IAuthRole for SqlAuthRole {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq for SqlAuthRole {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// this struct is used to store the authrole_dbset.json file
pub struct AuthRoleSqlDbSet {
    json_dbset: SqlFileDbSet<SqlAuthRole>,
}

impl AuthRoleSqlDbSet {
    // this is used to create a new AuthRoleSqlDbSet struct.
    // file_path: the path to the authrole_dbset.json file.
    // returns a AuthRoleSqlDbSet struct.
    pub fn open(file_path: String) -> std::io::Result<Self> {
        match SqlFileDbSet::open(file_path, "auth_roles".to_string(), SqlAuthRole::new, SqlAuthRole::select_sql, SqlAuthRole::insert_sql) {
            Ok(r) => Ok(Self {
                json_dbset: r
            }),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}

impl IDbSetAny for AuthRoleSqlDbSet {
    fn add_any(&self, item: Box<dyn Any>) {
        self.json_dbset.add_any(item)
    }

    fn add_range_any(&self, items: Vec<Box<dyn Any>>) {
        self.json_dbset.add_range_any(items)
    }

    fn attach_any(&self, item: Box<dyn Any>) {
        self.json_dbset.attach_any(item)
    }

    fn create_any(&self) -> Box<dyn Any> {
        self.json_dbset.create_any()
    }

    fn find_any(&self) -> Vec<Box<dyn Any>> {
        self.json_dbset.find_any()
    }

    fn get_all_any(&self) -> Vec<Box<dyn Any>> {
        self.json_dbset.get_all_any()
    }

    fn remove_any(&self, item: Box<dyn Any>) {
        self.json_dbset.remove_any(item)
    }

    fn remove_range_any(&self, items: Vec<Box<dyn Any>>) {
        self.json_dbset.remove_range_any(items)
    }

    fn as_any(&self, _type_info: TypeInfo) -> &dyn Any {
        self
    }

    fn entity_type_info(&self) -> TypeInfo {
        IDbSet::entity_type_info(&self.json_dbset)
    }

    fn entity_type_name(&self) -> &'static str {
        IDbSet::entity_type_name(&self.json_dbset)
    }

    fn save_changes(&self) {
        self.json_dbset.save_changes();
    }
}

impl IDbSet<SqlAuthRole> for AuthRoleSqlDbSet {
    fn add(&self, item: &SqlAuthRole) {
        self.json_dbset.add(item);
    }
    fn add_range(&self, items: Vec<SqlAuthRole>) {
        self.json_dbset.add_range(items);
    }

    fn attach(&self, item: &SqlAuthRole) {
        self.json_dbset.attach(item);
    }

    fn create(&self) -> SqlAuthRole {
        self.json_dbset.create()
    }

    fn find(&self) -> Vec<SqlAuthRole> {
        self.json_dbset.find()
    }

    fn get_all(&self) -> Vec<SqlAuthRole> {
        self.json_dbset.get_all()
    }

    fn remove(&self, item: &SqlAuthRole) {
        self.json_dbset.remove(item);
    }

    fn remove_range(&self, items: Vec<SqlAuthRole>) {
        self.json_dbset.remove_range(items);
    }

    fn entity_type_info(&self) -> TypeInfo {
        IDbSet::entity_type_info(&self.json_dbset)
    }

    fn entity_type_name(&self) -> &'static str {
        IDbSet::entity_type_name(&self.json_dbset)
    }

    fn upcast(&self) -> &dyn IDbSetAny {
        self
    }
}