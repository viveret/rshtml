use std::any::Any;
use std::collections::HashMap;

use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::model_binder::imodel::IModel;

// this is the view model for the index view
pub struct IndexViewModel {
}

impl IndexViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self { }
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

    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        todo!()
    }

    fn get_underlying_value(&self) -> &dyn Any {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}