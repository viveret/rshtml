use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::imodel_attribute::IAttribute;
use super::imodel_property::IModelProperty;




pub trait IModelMethod {
    fn get_name(&self) -> String;
    fn get_visibility(&self) -> String;
    fn get_return_type(&self) -> Box<TypeInfo>;
    fn get_arguments(&self) -> Vec<Rc<dyn IModelProperty>>;
    fn get_argument(&self, name: &str) -> Option<Rc<dyn IModelProperty>>;
    
    fn to_string(&self) -> String;
    fn get_type_info(&self) -> Box<TypeInfo>;

    // only contains runtime attributes, compile time attributes are not saved / reflected.
    fn get_attributes(&self) -> Vec<Rc<dyn IAttribute>>;
    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Rc<dyn IAttribute>>;
}

