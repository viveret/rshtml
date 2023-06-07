use std::any::Any;
use std::collections::HashMap;

use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::auth::auth_role_json_file_dbset::JsonAuthRole;

// this is the view model for the index view
pub struct IndexViewModel {
    pub roles: Vec<JsonAuthRole>
}

impl IndexViewModel {
    pub fn new(roles: Vec<JsonAuthRole>) -> Self {
        Self { roles: roles }
    }
}

impl IModel for IndexViewModel {
    fn get_properties(&self) -> HashMap<String, Box<dyn Any>> {
        todo!()
    }

    fn get_property(&self, name: &str) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn get_attributes(&self) -> Vec<Box<dyn Any>> {
        todo!()
    }

    fn get_attribute(&self, typeinfo: &mvc_lib::core::type_info::TypeInfo) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn get_type_info(&self) -> Box<mvc_lib::core::type_info::TypeInfo> {
        todo!()
    }

    fn get_underlying_value(&self) -> &dyn Any {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}