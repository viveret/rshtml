use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel::IModel;
use core_macro_lib::{IModel, IHazAttributes, reflect_attributes, reflect_properties, reflect_methods};
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;
use mvc_lib::auth::auth_role_json_file_dbset::JsonAuthRole;


// this is the view model for the index view
#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct IndexViewModel {
    pub roles: Vec<JsonAuthRole>
}

#[reflect_methods]
impl IndexViewModel {
    pub fn new(roles: Vec<JsonAuthRole>) -> Self {
        Self { roles: roles }
    }
}