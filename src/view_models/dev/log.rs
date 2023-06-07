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



pub struct LogViewModel {
    pub supports_read: bool,
    pub logs: Vec<String>,
}

impl LogViewModel {
    pub fn new(supports_read: bool, logs: Vec<String>) -> Self {
        Self { supports_read: supports_read, logs: logs }
    }
}

impl IModel for LogViewModel {
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