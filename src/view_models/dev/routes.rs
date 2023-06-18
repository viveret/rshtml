use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::controller_actions::controller_action::IControllerAction;

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

use super::route_details::RouteDetailsViewModel;


// this is the view model for the routes view
#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
pub struct RoutesViewModel {
    pub routes: Vec<RouteDetailsViewModel>,
}

#[reflect_methods]
impl RoutesViewModel {
    // create a new instance of the view model
    pub fn new(routes: Vec<Rc<dyn IControllerAction>>) -> Self {
        Self {
            routes: routes.iter().map(|r| RouteDetailsViewModel::new(r.clone(), None)).collect()
        }
    }
}
