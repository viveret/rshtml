use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::model_binder::iviewmodel::IViewModel;
use core_macro_lib::{IModel, IViewModel, IHazAttributes, reflect_attributes, reflect_properties, reflect_methods};
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;
use mvc_lib::view::iview::IView;


// this is the view model for the view details view
#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
pub struct ViewDetailsViewModel {
    // pub view: Rc<dyn IView>,
    pub path: String,
    pub raw: String,
    pub model_type_name: Option<String>,
}

#[reflect_methods]
impl ViewDetailsViewModel {
    // create a new instance of the view model
    pub fn new(view: Rc<dyn IView>) -> Self {
        Self {
            // view: view
            path: view.get_path().to_string(),
            raw: view.get_raw().to_string(),
            model_type_name: view.get_model_type_name()
        }
    }
}
 