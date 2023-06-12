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
use mvc_lib::model_binder::reflected_method::ReflectedMethod;


// this is the view model for the system info view
#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, IHazAttributes, IModel)]
pub struct SysInfoViewModel {
}

#[reflect_methods]
impl SysInfoViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self {  }
    }
}
