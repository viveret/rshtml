use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use nameof::name_of_type;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::iresponse_context::IResponseContext;
use crate::core::type_info::TypeInfo;

use crate::contexts::response_context::ResponseContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;

use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::request_middleware_service::MiddlewareResult;

use crate::services::routemap_service::IRouteMapService;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;

// this struct is used to short circuit a controller action if it is not being called from localhost.
// this is useful for controller actions that are only used by the application itself, a reverse proxy, a load balancer, or a local user.
// this struct must be used in conjunction with the LocalHostOnlyControllerActionFeatureMiddleware or else it will do nothing.
pub struct LocalHostOnlyControllerActionFeature {

}

impl LocalHostOnlyControllerActionFeature {
    // create a new instance of the feature.
    pub fn new() -> Self {
        Self {

        }
    }

    // create a new instance of the feature as a service for a service collection.
    pub fn new_service() -> Rc<dyn IControllerActionFeature> {
        Rc::new(Self::new())
    }
}

impl IControllerActionFeature for LocalHostOnlyControllerActionFeature {
    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<LocalHostOnlyControllerActionFeature>()
    }

    fn get_name(self: &Self) -> String {
        name_of_type!(LocalHostOnlyControllerActionFeature).to_string()
    }

    fn to_string(self: &Self) -> String {
        format!("{}", self.get_name())
    }

    fn invoke(self: &Self, _request_context: Rc<dyn IRequestContext>, _response_context: Rc<ResponseContext>, _services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        Ok(MiddlewareResult::OkContinue)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// this middleware is used to authorize a controller action that has the LocalHostOnlyControllerActionFeature.
// this middleware will short circuit the controller action if it is not being called from localhost.
// this is useful for controller actions that are only used by the application itself, a reverse proxy, a load balancer, or a local user.
// this middleware must be used in conjunction with the LocalHostOnlyControllerActionFeature or else it will do nothing.
pub struct LocalHostOnlyControllerActionFeatureMiddleware {
    mapper_service: Rc<dyn IRouteMapService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl LocalHostOnlyControllerActionFeatureMiddleware {
    // create a new instance of the middleware.
    // mapper_service: the route map service. this is used to get the controller action.
    pub fn new(mapper_service: Rc<dyn IRouteMapService>) -> Self {
        Self { mapper_service: mapper_service, next: RefCell::new(None) }
    }

    // create a new instance of the middleware as a service for a service collection.
    // services: the service collection.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services),
        )) as Rc<dyn IRequestMiddlewareService>)]
    }
    
    // this is called by the application to add the middleware to the service collection.
    // services: the service collection to add the middleware to.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for LocalHostOnlyControllerActionFeatureMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        let controller_name = request_context.get_str("ControllerName");

        if controller_name.len() > 0 {
            let controller = self.mapper_service.get_mapper().get_controller(controller_name.clone());

            let action_features = request_context.get_controller_action().get_features();
            let controller_features = controller.get_features();

            let find_my_feature: Vec<Rc<dyn IControllerActionFeature>> = controller_features
                .iter()
                .chain(
                    action_features.iter()
                )
                .filter(|x| x.get_name() == name_of_type!(LocalHostOnlyControllerActionFeature).to_string())
                .take(1)
                .cloned()
                .collect();

            if find_my_feature.len() > 0 {
                // let my_feature = find_my_feature.first().unwrap();
                // let feature = my_feature.as_ref() as LocalHostOnlyControllerActionFeature;
                let remote_addr_str = format!("{:?}", request_context.get_connection_context().get_tcp_context().get_remote_addr());

                // println!("connected IP address: {}", remote_addr_str);
                if !remote_addr_str.starts_with("127.0.0.1:") {
                    // short circuit, this is a local host only action
                    return Ok(MiddlewareResult::OkBreak);
                }
            }
        }
        
        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(response_context, request_context, services)?;

            match next_response {
                MiddlewareResult::OkBreak => {
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
                _ => { }
            }
        }
        
        Ok(MiddlewareResult::OkContinue)
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<LocalHostOnlyControllerActionFeatureMiddleware>())
    }
}