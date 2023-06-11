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


// this is the view model for the index view
#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
#[reflect_methods]
pub struct IndexViewModel {
}

impl IndexViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self { }
    }
}

// impl IModel for IndexViewModel {
//     fn get_properties(&self) -> HashMap<String, Box<dyn Any>> {
//         todo!()
//     }

//     fn get_property(&self, name: &str) -> Option<Box<dyn Any>> {
//         todo!()
//     }

//     fn get_attributes(&self) -> Vec<Box<dyn Any>> {
//         todo!()
//     }

//     fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Box<dyn Any>> {
//         todo!()
//     }

//     fn get_type_info(&self) -> Box<TypeInfo> {
//         todo!()
//     }

//     fn get_underlying_value(&self) -> &dyn Any {
//         todo!()
//     }

//     fn to_string(&self) -> String {
//         todo!()
//     }
// }