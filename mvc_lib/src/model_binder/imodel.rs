use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;


pub trait IModel {
    fn get_properties(&self) -> HashMap<String, Box<dyn Any>>;
    fn get_property(&self, name: &str) -> Option<Box<dyn Any>>;

    fn get_attributes(&self) -> Vec<Box<dyn Any>>;
    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Box<dyn Any>>;

    fn get_type_info(&self) -> Box<TypeInfo>;

    // similar to as any, but returns the value contained in the model struct instead of the model container itself.
    fn get_underlying_value(&self) -> &dyn Any;

    // string representation of the model, not used for binding or serialization / deserialization.
    fn to_string(&self) -> String;
}

#[derive(Clone)]
pub struct AnyIModel {
    pub model: Rc<dyn IModel>,
}

impl AnyIModel {
    pub fn new(model: Rc<dyn IModel>) -> Self {
        Self {
            model: model,
        }
    }
}

impl IModel for AnyIModel {
    fn get_properties(&self) -> HashMap<String, Box<dyn Any>> {
        self.model.get_properties()
    }
    fn get_property(&self, name: &str) -> Option<Box<dyn Any>> {
        self.model.get_property(name)
    }

    fn get_attributes(&self) -> Vec<Box<dyn Any>> {
        self.model.get_attributes()
    }
    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Box<dyn Any>> {
        self.model.get_attribute(typeinfo)
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        self.model.get_type_info()
    }

    // similar to as any, but returns the value contained in the model struct instead of the model container itself.
    fn get_underlying_value(&self) -> &dyn Any {
        self.model.get_underlying_value()
    }

    // string representation of the model, not used for binding or serialization / deserialization.
    fn to_string(&self) -> String {
        self.model.to_string()
    }
}