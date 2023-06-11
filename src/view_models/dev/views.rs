use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::model_binder::imodel::IModel;
use core_macro_lib::{IModel, IHazAttributes, reflect_properties, reflect_attributes, reflect_methods};
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;
use mvc_lib::view::iview::IView;

// this is the view model for the views view


#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct ViewsViewModel {
    pub views: Vec<Rc<dyn IView>>,
}

#[reflect_methods]
impl ViewsViewModel {
    // create a new instance of the view model
    pub fn new(views: Vec<Rc<dyn IView>>) -> Self {
        Self { views: views }
    }
}
