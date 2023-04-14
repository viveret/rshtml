use std::any::Any;
use std::rc::Rc;


use crate::auth::iauth_role::IAuthRole;

use crate::core::type_info::TypeInfo;

use crate::entity::idbset::{ IDbSet, IDbSetAny, JsonFileDbSet };


#[derive(Clone)]
pub struct JsonAuthRole {
    pub name: String,
}

impl JsonAuthRole {
    pub fn new() -> Self {
        Self {
            name: "".to_string()
        }
    }

    pub fn parse_json(v: serde_json::Value) -> Self {
        Self {
            name: v.to_string()
        }
    }
}

impl IAuthRole for JsonAuthRole {
    fn get_name(self: &Self) -> String {
        self.name.clone()
    }
}

pub struct AuthRoleJsonFileDbSet {
    json_dbset: JsonFileDbSet<JsonAuthRole>,
}

impl AuthRoleJsonFileDbSet {
    pub fn new() -> Self {
        Self {
            json_dbset: JsonFileDbSet::new(JsonAuthRole::new, JsonAuthRole::parse_json)
        }
    }
}

impl IDbSetAny for AuthRoleJsonFileDbSet {
    fn add_any(self: &Self, item: Box<dyn Any>) {
        self.json_dbset.add_any(item)
    }

    fn add_range_any(self: &Self, items: Vec<Box<dyn Any>>) {
        self.json_dbset.add_range_any(items)
    }

    fn attach_any(self: &Self, item: Box<dyn Any>) {
        self.json_dbset.attach_any(item)
    }

    fn create_any(self: &Self) -> Box<dyn Any> {
        self.json_dbset.create_any()
    }

    fn find_any(self: &Self) -> Vec<Box<dyn Any>> {
        self.json_dbset.find_any()
    }

    fn get_all_any(self: &Self) -> Vec<Box<dyn Any>> {
        self.json_dbset.get_all_any()
    }

    fn remove_any(self: &Self, item: Box<dyn Any>) {
        self.json_dbset.remove_any(item)
    }

    fn remove_range_any(self: &Self, items: Vec<Box<dyn Any>>) {
        self.json_dbset.remove_range_any(items)
    }

    fn as_any(self: &Self, type_info: TypeInfo) -> &dyn Any {
        self
    }

    fn entity_type_info(self: &Self) -> TypeInfo {
        IDbSet::entity_type_info(&self.json_dbset)
    }

    fn entity_type_name(self: &Self) -> &'static str {
        IDbSet::entity_type_name(&self.json_dbset)
    }
}

impl IDbSet<JsonAuthRole> for AuthRoleJsonFileDbSet {
    fn add(self: &Self, item: &JsonAuthRole) {
        self.json_dbset.add(item);
    }
    fn add_range(self: &Self, items: Vec<JsonAuthRole>) {
        self.json_dbset.add_range(items);
    }

    fn attach(self: &Self, item: &JsonAuthRole) {
        self.json_dbset.attach(item);
    }

    fn create(self: &Self) -> JsonAuthRole {
        self.json_dbset.create()
    }

    fn find(self: &Self) -> Vec<JsonAuthRole> {
        self.json_dbset.find()
    }

    fn get_all(self: &Self) -> Vec<JsonAuthRole> {
        self.json_dbset.get_all()
    }

    fn remove(self: &Self, item: &JsonAuthRole) {
        self.json_dbset.remove(item);
    }

    fn remove_range(self: &Self, items: Vec<JsonAuthRole>) {
        self.json_dbset.remove_range(items);
    }

    fn entity_type_info(self: &Self) -> TypeInfo {
        IDbSet::entity_type_info(&self.json_dbset)
    }

    fn entity_type_name(self: &Self) -> &'static str {
        IDbSet::entity_type_name(&self.json_dbset)
    }

    fn upcast(self: &Self) -> &dyn IDbSetAny {
        self
    }
}