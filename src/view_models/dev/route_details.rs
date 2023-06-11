use std::collections::HashMap;
use std::rc::Rc;

use mvc_lib::core::type_info::TypeInfo;

use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controllers::icontroller::IController;

use mvc_lib::model_binder::imodel::IModel;
use core_macro_lib::{IModel, IHazAttributes, reflect_attributes, reflect_properties, reflect_methods};
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;


// this is the view model for the route details view
#[derive(Clone, IHazAttributes, IModel)]
#[reflect_attributes]
#[reflect_properties]
pub struct RouteDetailsViewModel {
    pub route: Rc<dyn IControllerAction>,
    pub controller: Rc<dyn IController>,
}

#[reflect_methods]
impl RouteDetailsViewModel {
    // create a new instance of the view model
    pub fn new(route: Rc<dyn IControllerAction>, controller: Rc<dyn IController>) -> Self {
        Self { route: route, controller: controller }
    }
}
