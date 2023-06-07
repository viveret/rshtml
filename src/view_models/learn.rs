use std::any::Any;
use std::collections::HashMap;

// this file contains the view models for the learn controller

use mvc_lib::model_binder::imodel::IModel;

// this is the view model for the index view
pub struct IndexViewModel {
    // this is a list of all the learn docs
    pub learn_docs: Vec<String>,
}

impl IndexViewModel {
    // create a new instance of the view model
    pub fn new(learn_docs: Vec<String>) -> Self {
        Self { learn_docs: learn_docs }
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

// this is the view model for the details view
pub struct DetailsViewModel {
    // this is the path to the learn doc
    pub path: String,
}

impl DetailsViewModel {
    // create a new instance of the view model
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
}

impl IModel for DetailsViewModel {
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