use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use as_any::Downcast;
use core_macro_lib::nameof_member_fn;
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controllers::icontroller::IController;

use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::model_binder::imodel_binder::IModelBinder;
use mvc_lib::model_binder::model_validation_result::ModelValidationResult;
use mvc_lib::model_binder::url_encoded_model::UrlEncodedModel;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_descriptor::ServiceDescriptor;
use mvc_lib::services::service_scope::ServiceScope;
use mvc_lib::view::iview::IView;

// this is the view model for the index view
pub struct IndexViewModel {
}

impl IndexViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self { }
    }
}

// this is the view model for the views view
pub struct ViewsViewModel {
    pub views: Vec<Rc<dyn IView>>,
}

impl ViewsViewModel {
    // create a new instance of the view model
    pub fn new(views: Vec<Rc<dyn IView>>) -> Self {
        Self { views: views }
    }
}

// this is the view model for the view details view
pub struct ViewDetailsViewModel {
    pub view: Rc<dyn IView>,
}

impl ViewDetailsViewModel {
    // create a new instance of the view model
    pub fn new(view: Rc<dyn IView>) -> Self {
        Self { view: view }
    }
}

// this is the view model for the routes view
pub struct RoutesViewModel {
    pub routes: Vec<Rc<dyn IControllerAction>>,
}

impl RoutesViewModel {
    // create a new instance of the view model
    pub fn new(routes: Vec<Rc<dyn IControllerAction>>) -> Self {
        Self { routes: routes }
    }
}

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

// this is the view model for the system info view
pub struct SysInfoViewModel {
}

impl SysInfoViewModel {
    // create a new instance of the view model
    pub fn new() -> Self {
        Self {  }
    }
}

pub struct LogViewModel {
    pub supports_read: bool,
    pub logs: Vec<String>,
}

impl LogViewModel {
    pub fn new(supports_read: bool, logs: Vec<String>) -> Self {
        Self { supports_read: supports_read, logs: logs }
    }
}

#[derive(Clone, Debug)]
pub struct LogAddInputModel {
    pub message: String,
    pub level: String,
}

impl LogAddInputModel {
    pub fn new(message: String, level: String) -> Self {
        Self { message: message, level: level }
    }

    pub fn default() -> Self {
        Self { message: String::default(), level: String::default() }
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    pub fn is_valid(&self) -> bool {
        if self.message.is_empty() {
            false
        } else if self.level.is_empty() {
            false
        } else {
            match self.level.to_lowercase().as_str() {
                "trace" |
                "debug" |
                "info" |
                "warn" |
                "error" |
                "fatal" => true,
                _ => false
            }
        }
    }

    pub fn parse_level(&self) -> log::Level {
        match self.level.as_str() {
            nameof_member_fn!(log::Level::Trace) => log::Level::Trace,
            "debug" => log::Level::Debug,
            "info" => log::Level::Info,
            "warn" => log::Level::Warn,
            "error" => log::Level::Error,
            "fatal" => log::Level::Error,
            _ => log::Level::Info
        }
    }
}

impl IModel for LogAddInputModel {
    fn get_properties(&self) -> std::collections::HashMap<String, Box<dyn std::any::Any>> {
        vec![
            ("message".to_string(), Box::new(self.message.clone()) as Box<dyn std::any::Any>),
            ("level".to_string(), Box::new(self.level.clone()) as Box<dyn std::any::Any>),
        ]
        .into_iter()
        .collect::<HashMap<String, Box<dyn std::any::Any>>>()
    }

    fn get_property(&self, name: &str) -> Option<Box<dyn std::any::Any>> {
        match name {
            "message" => Some(Box::new(self.message.clone()) as Box<dyn std::any::Any>),
            "level" => Some(Box::new(self.level.clone()) as Box<dyn std::any::Any>),
            _ => None,
        }
    }

    fn get_attributes(&self) -> Vec<Box<dyn std::any::Any>> {
        vec![]
    }

    fn get_attribute(&self, typeinfo: &mvc_lib::core::type_info::TypeInfo) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn get_type_info(&self) -> Box<mvc_lib::core::type_info::TypeInfo> {
        Box::new(TypeInfo::of::<LogAddInputModel>())
    }

    fn get_underlying_value(&self) -> Box<dyn std::any::Any> {
        self.as_any()
    }

    fn to_string(&self) -> String {
        format!("LogAddInputModel {{ message: {}, level: {} }}", self.message, self.level)
    }
    // fn validate(&self) -> ModelValidationResult<()> {
    //     let mut result = ModelValidationResult::new();
    //     if self.message.is_empty() {
    //         result.add_error("message", "Message is required.");
    //     }
    //     if self.level.is_empty() {
    //         result.add_error("level", "Level is required.");
    //     }
    //     result
    // }
}

pub struct LogAddInputModelBinder {

}

impl LogAddInputModelBinder {
    pub fn new() -> Self {
        Self { }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IModelBinder>)]
    }

    pub fn add_to_services(services: &mut mvc_lib::services::service_collection::ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IModelBinder>(), LogAddInputModelBinder::new_service, ServiceScope::Singleton));
    }
}

impl IModelBinder for LogAddInputModelBinder {
    fn type_info(self: &Self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<LogAddInputModel>())
    }

    fn matches(self: &Self, request_context: &dyn mvc_lib::contexts::irequest_context::IRequestContext) -> bool {
        true
    }

    fn bind_model(self: &Self, request_context: &dyn mvc_lib::contexts::irequest_context::IRequestContext) -> ModelValidationResult<Rc<dyn IModel>> {
        let mut model = LogAddInputModel::default();
        if let Some(body) = request_context.get_body_content() {
            let content_type = request_context.get_content_type().unwrap();
            let form_encoded = UrlEncodedModel::new_from_body(content_type, body);
            let form = &form_encoded.0.entries;
            
            if let Some(message) = form.get("message") {
                model.message = message.first().unwrap().to_string();
            } else {
                return ModelValidationResult::PropertyError(Rc::new(model), "message".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Message is required.".to_string())));
            }
    
            if let Some(level) = form.get("level") {
                model.level = level.first().unwrap().to_string();
            } else {
                return ModelValidationResult::PropertyError(Rc::new(model), "level".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Level is required.".to_string())));
            }
            
            ModelValidationResult::Ok(Rc::new(model))
        } else {
            // if no body then decode from request query string
            // let query_string = request_context.get_query_string();
            // let form = &query_string.entries;
            // request_context.decode_and_bind_body(services, &mut model, form)
            ModelValidationResult::OtherError(Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing request body.".to_string())))
        }
    }
}

pub struct LogAddViewModel {
    pub supports_read: bool,
    pub input: Rc<LogAddInputModel>,
}

impl LogAddViewModel {
    pub fn new(supports_read: bool, input: Rc<LogAddInputModel>) -> Self {
        Self { supports_read: supports_read, input: input }
    }
}

pub struct LogClearViewModel {
    pub supports_clear: bool,
}

impl LogClearViewModel {
    pub fn new(supports_clear: bool) -> Self {
        Self { supports_clear: supports_clear }
    }
}

pub struct PerfLogViewModel {

}

impl PerfLogViewModel {
    pub fn new() -> Self {
        Self { }
    }
}
