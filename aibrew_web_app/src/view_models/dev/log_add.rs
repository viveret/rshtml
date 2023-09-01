use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use core_macro_lib::IHazAttributes;
use core_macro_lib::IViewModel;
use core_macro_lib::fake_property_attribute;
use core_macro_lib::reflect_attributes;
use core_macro_lib::reflect_methods;
use core_macro_lib::reflect_properties;
use core_macro_lib::nameof_member_fn;
use mvc_lib::contexts::irequest_context::IRequestContext;

use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::model_binder::imodel::AnyIModel;
use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::model_binder::iviewmodel::IViewModel;
use core_macro_lib::IModel;
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;
use mvc_lib::model_binder::imodel_binder::IModelBinder;
use mvc_lib::model_binder::model_validation_result::ModelValidationResult;
use mvc_lib::model_binder::url_encoded_model::UrlEncodedModel;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_descriptor::ServiceDescriptor;
use mvc_lib::services::service_scope::ServiceScope;



#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
pub struct LogAddInputModel {
    // attribute macros are currently not possible on struct fields because of the way attribute macros are implemented.
    // might be able to do it with a proc macro instead of an attribute macro but that would be complicated.
    // another way to do it would be to have a separate attribute macro that is applied to the struct that
    // extracts the attributes from the struct fields and adds them to the struct AST but removes them from the struct fields
    // before the struct is returned to the compiler.
    // #[display_name(x: "Message: ")]
    pub message: Box<String>,
    
    // #[display_name(x: "Level: ")]
    pub level: Box<String>,
}

#[reflect_methods]
impl LogAddInputModel {
    pub fn new(message: String, level: String) -> Self {
        Self { message: Box::new(message), level: Box::new(level) }
    }

    pub fn default() -> Self {
        Self::new(String::default(), String::default())
    }

    #[fake_property_attribute]
    pub fn is_valid(&self) -> bool {
        if self.message.is_empty() {
            false
        } else if self.level.is_empty() {
            false
        } else {
            Self::is_valid_level(&self.level)
        }
    }

    pub fn is_valid_level(level: &String) -> bool {
        match level.to_lowercase().as_str() {
            "trace" |
            "debug" |
            "info" |
            "warn" |
            "error" |
            "fatal" => true,
            _ => false
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

    pub fn get_validation_result(&self) -> ModelValidationResult<LogAddInputModel> {
        let mut errors: Vec<(String, Rc<dyn std::error::Error>)> = vec![];
        if self.message.is_empty() {
            errors.push(("message".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Message is required."))));
        }
        
        if self.level.is_empty() {
            errors.push(("level".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Level is required."))));
        } else if !Self::is_valid_level(&self.level) {
            errors.push(("level".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Level is invalid (must be one of trace, debug, info, warn, error, fatal)."))));
        }

        match errors.len() {
            0 => ModelValidationResult::Ok(self.clone()),
            1 => ModelValidationResult::PropertyError(self.clone(), errors[0].0.clone(), errors[0].1.clone()),
            _ => ModelValidationResult::MultipleErrors(self.clone(), errors)
        }
    }
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

    fn matches(self: &Self, _request_context: &dyn IRequestContext) -> bool {
        true
    }

    fn bind_model(self: &Self, request_context: &dyn IRequestContext) -> ModelValidationResult<AnyIModel> {
        let mut model = LogAddInputModel::default();
        // if let Some(body) = request_context.get_body_content() {
            let content_type = request_context.get_content_type().unwrap();
            let form_encoded = UrlEncodedModel::new_from_body(content_type, request_context);
            let form = &form_encoded.0.entries;
            
            if let Some(message) = form.get("message") {
                model.message = Box::new(message.first().unwrap().to_string());
            } else {
                return ModelValidationResult::PropertyError(AnyIModel::new(Rc::new(model)), "message".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Message is required.".to_string())));
            }
    
            if let Some(level) = form.get("level") {
                model.level = Box::new(level.first().unwrap().to_string());
            } else {
                return ModelValidationResult::PropertyError(AnyIModel::new(Rc::new(model)), "level".to_string(), Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Level is required.".to_string())));
            }
            
            ModelValidationResult::Ok(AnyIModel::new(Rc::new(model)))
        // } else {
            // if no body then decode from request query string
            // let query_string = request_context.get_query_string();
            // let form = &query_string.entries;
            // request_context.decode_and_bind_body(services, &mut model, form)
            // ModelValidationResult::OtherError(Rc::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing request body.".to_string())))
        // }
    }
}


#[reflect_attributes]
#[reflect_properties]
#[derive(Clone, Debug, IHazAttributes, IModel, IViewModel)]
pub struct LogAddViewModel {
    pub supports_read: bool,
    pub input: LogAddInputModel,
}

#[reflect_methods]
impl LogAddViewModel {
    pub fn new(supports_read: bool, input: Rc<LogAddInputModel>) -> Self {
        Self { supports_read: supports_read, input: input.as_ref().clone() }
    }
}
