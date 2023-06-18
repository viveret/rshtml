use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controllers::icontroller::IController;

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



// this is the view model for the route details view
#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
pub struct RouteDetailsViewModel {
    pub as_string: String,
    pub path: String,
    pub features: Vec<String>,
    pub controller_features: Vec<String>,
}

#[reflect_methods]
impl RouteDetailsViewModel {
    // create a new instance of the view model
    pub fn new(route: Rc<dyn IControllerAction>, controller: Option<Rc<dyn IController>>) -> Self {
        Self {
            as_string: route.to_string(),
            path: route.get_path().to_string(),
            features: route.get_features().iter().map(|f| f.to_string()).collect(),
            controller_features: controller.map(|x| x.get_features().iter().map(|f| f.to_string()).collect()).unwrap_or(vec![]),
        }
    }
}
