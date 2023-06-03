use std::any::Any;
use std::collections::HashMap;

use crate::core::type_info::TypeInfo;


pub trait IModel {
    fn get_properties(&self) -> HashMap<String, Box<dyn Any>>;
    fn get_property(&self, name: &str) -> Option<Box<dyn Any>>;

    fn get_attributes(&self) -> Vec<Box<dyn Any>>;
    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Box<dyn Any>>;

    fn get_type_info(&self) -> Box<TypeInfo>;

    fn get_underlying_value(&self) -> Box<dyn Any>;

    // string representation of the model, not used for binding or serialization / deserialization.
    fn to_string(&self) -> String;
}