use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::imodel_attribute::IAttribute;



pub trait IHazAttributes {
    // only contains runtime attributes, compile time attributes are not saved / reflected.
    fn get_attributes(&self) -> Vec<Rc<dyn IAttribute>>;
    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Rc<dyn IAttribute>>;
}