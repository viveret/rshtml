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


// this is the view model for the index view
#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct IndexViewModel {
    // this is a list of all the learn docs
    pub learn_docs: Vec<String>,
}

#[reflect_methods]
impl IndexViewModel {
    // create a new instance of the view model
    pub fn new(learn_docs: Vec<String>) -> Self {
        Self { learn_docs: learn_docs }
    }
}


// this is the view model for the details view
#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct DetailsViewModel {
    // this is the path to the learn doc
    pub path: String,
}

#[reflect_methods]
impl DetailsViewModel {
    // create a new instance of the view model
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
}
