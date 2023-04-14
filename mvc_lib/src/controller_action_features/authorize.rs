use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use crate::contexts::request_context::RequestContext;
use crate::contexts::response_context::ResponseContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controllers::controller_actions_map::IControllerActionsMap;

use crate::services::authorization_service::AuthResult;
use crate::services::authorization_service::IAuthorizationService;

use crate::services::request_middleware_service::IRequestMiddlewareService;
use crate::services::request_middleware_service::MiddlewareResult;

use crate::services::routemap_service::IRouteMapService;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;

pub struct AllowAnonymous {

}

impl AllowAnonymous {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn new_service() -> Rc<dyn IControllerActionFeature> {
        Rc::new(Self::new())
    }
}

impl IControllerActionFeature for AllowAnonymous {
    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<AllowAnonymous>()
    }

    fn get_name(self: &Self) -> String {
        nameof::name_of_type!(AllowAnonymous).to_string()
    }

    fn to_string(self: &Self) -> String {
        format!("{}", self.get_name())
    }

    fn invoke(self: &Self, request_context: Rc<RequestContext>, _response_ctx: Rc<ResponseContext>, _services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        println!("Allow Anonymous {:?}", request_context.connection_context.get_remote_addr());
        Ok(MiddlewareResult::OkContinue)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// https://learn.microsoft.com/en-us/aspnet/core/security/authorization/policies?view=aspnetcore-7.0
// https://learn.microsoft.com/en-us/aspnet/core/security/authorization/iauthorizationpolicyprovider?view=aspnetcore-7.0
pub struct AuthorizeControllerActionFeature {
    pub roles: Vec<String>,
    pub policy: Option<String>,
}

impl AuthorizeControllerActionFeature {
    pub fn new(
        roles: Vec<String>,
        policy: Option<String>,
    ) -> Self {
        Self {
            roles: roles,
            policy: policy,
        }
    }

    pub fn new_parse(
        roles: String,
        policy: Option<String>,
    ) -> Self {
        Self::new(roles.split(',').map(|s| s.to_string()).collect(), policy)
    }

    pub fn new_service(
        roles: Vec<String>,
        policy: Option<String>,
    ) -> Rc<dyn IControllerActionFeature> {
        Rc::new(Self::new(roles, policy))
    }

    pub fn new_service_parse(
        roles: String,
        policy: Option<String>,
    ) -> Rc<dyn IControllerActionFeature> {
        Rc::new(Self::new_parse(roles, policy))
    }
}

impl IControllerActionFeature for AuthorizeControllerActionFeature {
    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<AuthorizeControllerActionFeature>()
    }

    fn get_name(self: &Self) -> String {
        nameof::name_of_type!(AuthorizeControllerActionFeature).to_string()
    }

    fn to_string(self: &Self) -> String {
        format!("{} (roles: {:?}, policy: {:?})", self.get_name(), self.roles, self.policy)
    }

    fn invoke(self: &Self, request_context: Rc<RequestContext>, _response_ctx: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        Ok(MiddlewareResult::OkContinue)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AuthorizeControllerActionFeatureMiddleware {
    mapper_service: Rc<dyn IRouteMapService>,
    next: RefCell<Option<Rc<dyn IRequestMiddlewareService>>>
}

impl AuthorizeControllerActionFeatureMiddleware {
    pub fn new(mapper_service: Rc<dyn IRouteMapService>) -> Self {
        Self { mapper_service: mapper_service, next: RefCell::new(None) }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services)
        )) as Rc<dyn IRequestMiddlewareService>)]
    }
    
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IRequestMiddlewareService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IRequestMiddlewareService for AuthorizeControllerActionFeatureMiddleware {
    fn set_next(self: &Self, next: Option<Rc<dyn IRequestMiddlewareService>>) {
        self.next.replace(next);
    }

    fn handle_request(self: &Self, request_context: Rc<RequestContext>, response_context: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        let auth_service = ServiceCollectionExtensions::get_required_single::<dyn IAuthorizationService>(services);
        let controller_name = request_context.get_str("ControllerName");

        if controller_name.len() > 0 {
            let controller = self.mapper_service.get_mapper().get_controller(controller_name.clone());
            match auth_service.authenticate_http_request(controller, request_context.clone())? {
                AuthResult::Ok => {
                    // should make note of authorization somewhere
                },
                AuthResult::Rejection(reason) => {
                    println!("Request denied, unauthorized: {:?}", reason);
                    response_context.as_ref().status_code.replace(http::StatusCode::NOT_FOUND);
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
            }
        }
        
        if let Some(next) = self.next.borrow().as_ref() {
            let next_response = next.handle_request(request_context.clone(), response_context.clone(), services)?;

            match next_response {
                MiddlewareResult::OkBreak => {
                    return Ok(MiddlewareResult::OkBreak); // short circuit middleware
                },
                _ => { }
            }
        }

        Ok(MiddlewareResult::OkContinue)
    }
}