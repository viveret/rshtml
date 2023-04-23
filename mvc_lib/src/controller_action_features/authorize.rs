use std::any::Any;
use std::error::Error;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use crate::contexts::response_context::ResponseContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;

use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::service_collection::IServiceCollection;

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

    fn invoke(self: &Self, request_context: Rc<dyn IRequestContext>, _response_ctx: Rc<ResponseContext>, _services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        println!("Allow Anonymous {:?}", request_context.get_connection_context().get_remote_addr());
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

    fn invoke(self: &Self, request_context: Rc<dyn IRequestContext>, _response_ctx: Rc<ResponseContext>, services: &dyn IServiceCollection) -> Result<MiddlewareResult, Box<dyn Error>> {
        Ok(MiddlewareResult::OkContinue)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
