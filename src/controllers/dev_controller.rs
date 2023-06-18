use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use core_macro_lib::IHazAttributes;
use core_macro_lib::IModel;
use core_macro_lib::fake_property_attribute;
use core_macro_lib::reflect_attributes;
use core_macro_lib::reflect_methods;
use core_macro_lib::reflect_properties;
use core_macro_lib::nameof_member_fn;

use mvc_lib::action_results::iaction_result::IActionResult;
use mvc_lib::action_results::redirect_action_result::RedirectActionResult;
use mvc_lib::core::string_extensions::action_name_to_path;
use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::controller_actions::member_fn::ControllerActionMemberFn;
use mvc_lib::controllers::icontroller_extensions::IControllerExtensions;
use mvc_lib::diagnostics::logging::logging_service::ILoggingService;
use mvc_lib::diagnostics::logging::logging_service::LoggingService;
use mvc_lib::model_binder::imodel_attribute::IAttribute;
use mvc_lib::model_binder::ihaz_attributes::IHazAttributes;
use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::model_binder::imodel_method::IModelMethod;
use mvc_lib::model_binder::imodel_property::IModelProperty;
use mvc_lib::model_binder::reflected_attribute::ReflectedAttribute;
use mvc_lib::model_binder::reflected_property::ReflectedProperty;
use mvc_lib::model_binder::reflected_method::ReflectedMethod;
use mvc_lib::model_binder::model_validation_result::ModelValidationResult;
use mvc_lib::services::routemap_service::IRouteMapService;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_collection::ServiceCollectionExtensions;

use mvc_lib::contexts::controller_context::IControllerContext;

use mvc_lib::action_results::view_result::ViewResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::authorize::BypassOnLocalActionFilter;
use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controller_actions::controller_action::IControllerAction;

use mvc_lib::controller_action_features::local_host_only::LocalHostOnlyControllerActionFeature;
use mvc_lib::controller_action_features::authorize::AuthorizeControllerActionFeature;

use mvc_lib::view::view_renderer::IViewRenderer;
use mvc_macro_lib::rc_controller_action;
use mvc_macro_lib::rc_controller_action_validate_typed;

use crate::view_models::dev::controllers::ControllerDetailsViewModel;
use crate::view_models::dev::controllers::ControllersViewModel;
use crate::view_models::dev::log_add::LogAddInputModel;
use crate::view_models::dev::log_add::LogAddViewModel;
use crate::view_models::dev::log_clear::LogClearViewModel;
use crate::view_models::dev::log::LogViewModel;
use crate::view_models::dev::perf_log::PerfLogViewModel;
use crate::view_models::dev::index::IndexViewModel;
use crate::view_models::dev::views::ViewsViewModel;
use crate::view_models::dev::view_details::ViewDetailsViewModel;
use crate::view_models::dev::routes::RoutesViewModel;
use crate::view_models::dev::route_details::RouteDetailsViewModel;
use crate::view_models::dev::sys_info::SysInfoViewModel;


// this is the controller for the developer section of the site.
#[reflect_attributes]
#[reflect_properties]
#[derive(IHazAttributes, Clone, IModel)]
pub struct DevController {
    // this needs to be fixed such that reflect_properties will
    // extract attributes on data so they don't appear in final code.
    // Rust does not allow attributes on data properties by default.
    // another fact is that Rust does not support attributes on struct properties.
    // this is because attributes are only allowed on structs, enums, and functions.
    // these are valid AST nodes, but properties are not because they are separated by commas,
    // unless it is the last property. the only way to get around this is to use a regular macro function
    // instead of a proc macro attribute.
    // #[fake_property_attribute]
    pub log_service: Rc<dyn ILoggingService>,
}

#[reflect_methods]
impl DevController {
    // create a new instance of the controller.
    pub fn new(log_service: Rc<dyn ILoggingService>) -> Self {
        Self {
            log_service: log_service,
        }
    }

    // create a new instance of the controller as a service for a service collection.
    // services: the collection of available services.
    // returns: a new instance of the controller as a service in a vector.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn ILoggingService>(services)
        )) as Rc<dyn IController>)]
    }

    // this is the index action for the controller.
    pub fn index(&self, _controller_ctx: &dyn IControllerContext, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let view_model = Rc::new(IndexViewModel::new());
        Ok(Some(Rc::new(ViewResult::new("views/dev/index.rs".to_string(), view_model))))
    }

    // this action returns a view of a list of all the views in the application.
    pub fn views(&self, _controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let view_model = Rc::new(ViewsViewModel::new(view_renderer.get_all_views(services)));
        Ok(Some(Rc::new(ViewResult::new("views/dev/views.rs".to_string(), view_model))))
    }

    // this action returns a view of the details of a view in the application.
    pub fn view_details(&self, controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let request_context = controller_ctx.get_request_context();

        let path = &request_context.get_path()["/dev/views/".len()..];

        if path.len() == 0 {
            return Ok(Some(Rc::new(RedirectActionResult::new(false, Some(false), None, Some("views".to_string()), Some("Dev".to_string()), None, None))))
        }

        // println!("Viewing view at path: {:?}", path);
        let view_renderer = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let view_model = Rc::new(ViewDetailsViewModel::new(view_renderer.get_view(&path.to_string(), services)));
        return Ok(Some(Rc::new(ViewResult::new("views/dev/view_details.rs".to_string(), view_model))));
    }

    // this action returns a view of a list of all the controllers in the application.
    pub fn controllers(&self, _controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let route_map_service = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
        let view_model = Rc::new(ControllersViewModel::from_controllers(route_map_service.as_ref().get_mapper().as_ref().get_controllers()));
        Ok(Some(Rc::new(ViewResult::new("views/dev/controllers.rs".to_string(), view_model))))
    }

    #[fake_property_attribute]
    pub fn controller_details(&self, controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let request_context = controller_ctx.get_request_context();
        let path = &request_context.get_path()["/dev/controllers/".len()..];

        if path.len() == 0 {
            return Ok(Some(Rc::new(RedirectActionResult::new(false, Some(false), None, Some("controllers".to_string()), Some("Dev".to_string()), None, None))))
        }

        let route_map_service = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
        let controller = route_map_service.as_ref().get_mapper().get_controller(path.to_string());
        let view_model = Rc::new(ControllerDetailsViewModel::new(
            controller.get_type_name().into(),
            controller.get_actions().iter().map(|a| (a.get_name(), a.get_path().to_cow_str())).collect(),
            controller.get_features().iter().map(|f| f.get_name()).collect(),
            controller.get_attributes().iter().map(|a| a.to_string()).collect(),
            controller.get_properties().iter().map(|a| (a.0.clone(), a.1.get_return_type().map(|x| x.to_string()).unwrap_or("void".to_string()))).collect(),
            controller.get_methods().iter().map(|a| (
                a.1.get_attributes().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", "),
                format!("{} {}", a.1.get_visibility(), a.0.clone()), 
                a.1.get_arguments().iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "),
                a.1.get_return_type().map(|x| x.to_string()).unwrap_or("void".to_string()),
            )).collect(),
        ));
        return Ok(Some(Rc::new(ViewResult::new("views/dev/controller_details.rs".to_string(), view_model))));
    }

    // this action returns a view of a list of all the routes in the application.
    pub fn routes(&self, _controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let routes = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
        let view_model = Rc::new(RoutesViewModel::new(routes.as_ref().get_mapper().as_ref().get_all_actions()));
        Ok(Some(Rc::new(ViewResult::new("views/dev/routes.rs".to_string(), view_model))))
    }

    // this action returns a view of the details of a route in the application.
    pub fn route_details(&self, controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let request_context = controller_ctx.get_request_context();
        let path = &request_context.get_path()["/dev/routes/".len()..];

        if path.len() == 0 {
            return Ok(Some(Rc::new(RedirectActionResult::new(false, Some(false), None, Some("routes".to_string()), Some("Dev".to_string()), None, None))))
        }

        let routes = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
        let route = routes.as_ref().get_mapper().as_ref().get_action_at_area_controller_action_path(path.to_string());
        let controller = routes.as_ref().get_mapper().get_controller(route.get_controller_name().to_string());

        let view_model = Rc::new(RouteDetailsViewModel::new(route, Some(controller)));
        return Ok(Some(Rc::new(ViewResult::new("views/dev/route_details.rs".to_string(), view_model))));
    }

    // this action returns a view of the system information.
    pub fn sys_info(&self, _controller_ctx: &dyn IControllerContext, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let view_model = Rc::new(SysInfoViewModel::new());
        Ok(Some(Rc::new(ViewResult::new("views/dev/sysinfo.rs".to_string(), view_model))))
    }

    pub fn log(&self, _controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let logger = LoggingService::get_service(services).get_logger();
        let supports_read = logger.supports_read();
        let logs = if supports_read { logger.read_logs() } else { vec![] };
        let view_model = Rc::new(LogViewModel::new(supports_read, logs));
        Ok(Some(Rc::new(ViewResult::new("views/dev/log.rs".to_string(), view_model))))
    }

    pub fn log_add(&self, model_result: ModelValidationResult<LogAddInputModel>, controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let mut model_result = model_result; // remut
        let logger = LoggingService::get_service(services).get_logger();
        let supports_read = logger.supports_read();
        println!("model_result: {:?}", model_result);
        
        let model = match model_result {
            ModelValidationResult::Ok(model) => model.clone(),
            ModelValidationResult::OkNone |
            ModelValidationResult::ModelError(..) |
            ModelValidationResult::PropertyError(..) |
            ModelValidationResult::MultipleErrors(..) |
            ModelValidationResult::OtherError(..) => LogAddInputModel::default(),
        };
        let method = controller_ctx.get_request_context().get_method();
        println!("{} model: {}", method, model.to_string());

        if method == http::Method::GET || !model.is_valid() {
            if !model.is_valid() {
                model_result = model.get_validation_result();
                controller_ctx.get_request_context().set_model_validation_result(Some(model_result.as_anyimodel()));
            }

            let view_model = Rc::new(LogAddViewModel::new(supports_read, Rc::new(model)));
            Ok(Some(Rc::new(ViewResult::new("views/dev/log_add.rs".to_string(), view_model))))
        } else {
            self.log_service.log(model.parse_level(), model.message.as_str());            
            Ok(Some(Rc::new(RedirectActionResult::new(false, Some(false), None, Some("log".to_string()), Some("Dev".to_string()), None, None))))
        }
    }

    pub fn log_clear(&self, controller_ctx: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let logger = LoggingService::get_service(services).get_logger();
        let supports_clear = logger.supports_clear();

        if controller_ctx.get_request_context().get_method() == http::Method::GET {
            let view_model = Rc::new(LogClearViewModel::new(supports_clear));
            Ok(Some(Rc::new(ViewResult::new("views/dev/log_clear.rs".to_string(), view_model))))
        } else {
            logger.clear_logs();
            Ok(Some(Rc::new(RedirectActionResult::new(false, Some(false), None, Some("log".to_string()), Some("Dev".to_string()), None, None))))
        }
    }

    pub fn perf_log(&self, _controller_ctx: &dyn IControllerContext, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        let view_model = Rc::new(PerfLogViewModel::new());
        Ok(Some(Rc::new(ViewResult::new("views/dev/perf_log.rs".to_string(), view_model))))
    }

    pub fn error(&self, _controller_ctx: &dyn IControllerContext, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>> {
        Err(Rc::new(std::io::Error::new(std::io::ErrorKind::Other, "This is a test error.")))
    }
}

impl IController for DevController {
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(DevController)
    }

    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        let controller_name = IControllerExtensions::get_name(self);

        vec![
            rc_controller_action!(index),
            rc_controller_action!(controllers),
            rc_controller_action!(routes),
            rc_controller_action!(views),
            rc_controller_action!(sys_info),
            rc_controller_action!(log),
            rc_controller_action_validate_typed!(log_add),
            rc_controller_action!(log_clear),
            rc_controller_action!(perf_log),
            rc_controller_action!(error),
            
            Rc::new(ControllerActionMemberFn::new_not_validated(vec![], None, "/dev/controllers/..".into(), nameof_member_fn!(Self::controller_details).into(), controller_name.clone().into(), self.get_route_area(), Box::new(Self::controller_details))),
            Rc::new(ControllerActionMemberFn::new_not_validated(vec![], None, "/dev/routes/..".into(), nameof_member_fn!(Self::route_details).into(), controller_name.clone().into(), self.get_route_area(), Box::new(Self::route_details))),
            Rc::new(ControllerActionMemberFn::new_not_validated(vec![], None, "/dev/views/..".into(), nameof_member_fn!(Self::view_details).into(), controller_name.clone().into(), self.get_route_area(), Box::new(Self::view_details))),
        ]
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![
            AuthorizeControllerActionFeature::new_service_parse("admin,dev,owner".to_string(), None, Some(vec![
                Box::new(BypassOnLocalActionFilter::new())
            ])),
            LocalHostOnlyControllerActionFeature::new_service()
        ]
    }
}
