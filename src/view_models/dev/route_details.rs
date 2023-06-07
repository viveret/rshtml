use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use as_any::Downcast;
use core_macro_lib::nameof_member_fn;
use mvc_lib::contexts::irequest_context::IRequestContext;
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controllers::icontroller::IController;

use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::model_binder::imodel::{IModel, AnyIModel};
use mvc_lib::model_binder::imodel_binder::IModelBinder;
use mvc_lib::model_binder::model_validation_result::ModelValidationResult;
use mvc_lib::model_binder::url_encoded_model::UrlEncodedModel;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_descriptor::ServiceDescriptor;
use mvc_lib::services::service_scope::ServiceScope;
use mvc_lib::view::iview::IView;



// this is the view model for the route details view
pub struct RouteDetailsViewModel {
    pub route: Rc<dyn IControllerAction>,
    pub controller: Rc<dyn IController>,
}

impl RouteDetailsViewModel {
    // create a new instance of the view model
    pub fn new(route: Rc<dyn IControllerAction>, controller: Rc<dyn IController>) -> Self {
        Self { route: route, controller: controller }
    }
}

impl IModel for RouteDetailsViewModel {
    fn get_properties(&self) -> HashMap<String, Box<dyn Any>> {
        todo!()
    }

    fn get_property(&self, name: &str) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn get_attributes(&self) -> Vec<Box<dyn Any>> {
        todo!()
    }

    fn get_attribute(&self, typeinfo: &TypeInfo) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        todo!()
    }

    fn get_underlying_value(&self) -> &dyn Any {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}