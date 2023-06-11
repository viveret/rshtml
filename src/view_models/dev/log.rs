use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::model_binder::imodel::IModel;
use core_macro_lib::{IModel, IHazAttributes, reflect_attributes, reflect_properties, reflect_methods};
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;



#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct LogViewModel {
    pub supports_read: bool,
    pub logs: Vec<String>,
}

#[reflect_methods]
impl LogViewModel {
    pub fn new(supports_read: bool, logs: Vec<String>) -> Self {
        Self { supports_read: supports_read, logs: logs }
    }
}