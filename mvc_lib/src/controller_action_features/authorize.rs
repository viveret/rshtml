use std::any::Any;
use std::error::Error;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use crate::contexts::response_context::ResponseContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;

use crate::services::request_middleware_service::MiddlewareResult;
use crate::services::service_collection::IServiceCollection;



pub trait IAuthRequirementFilter {
    fn use_requirement(self: &Self) -> bool;
}

pub struct BypassOnLocalActionFilter {}

impl BypassOnLocalActionFilter {
    pub fn new() -> Self {
        Self {}
    }
}

impl IAuthRequirementFilter for BypassOnLocalActionFilter {
    fn use_requirement(self: &Self) -> bool {
        false
    }
}


// this struct is used to indicate that a controller action can be called by anyone without authorization.
// this struct must be used in conjunction with the AllowAnonymousControllerActionFeatureMiddleware or else it will do nothing.
// this struct is useful for controller actions that are used for logging, metrics, or other non-sensitive data.
// this struct is also useful for controller actions that are used for authorization or authentication because otherwise the user would not be able to log in.
pub struct AllowAnonymous {

}

impl AllowAnonymous {
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

    fn invoke(self: &Self, request_context: Rc<dyn IRequestContext>, _response_context: Rc<ResponseContext>, _services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        println!("Allow Anonymous {:?}", request_context.get_connection_context().get_tcp_context().get_remote_addr());
        Ok(MiddlewareResult::OkContinue)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// https://learn.microsoft.com/en-us/aspnet/core/security/authorization/policies?view=aspnetcore-7.0
// https://learn.microsoft.com/en-us/aspnet/core/security/authorization/iauthorizationpolicyprovider?view=aspnetcore-7.0
// this struct is used to indicate that a controller action can only be called by users with a specific role or policy.
// this struct must be used in conjunction with the AuthorizeControllerActionFeatureMiddleware or else it will do nothing.
// this struct is useful for controller actions that are used for sensitive data or actions.
pub struct AuthorizeControllerActionFeature {
    // the roles that are allowed to call the controller action.
    pub roles: Vec<String>,
    // the policy that is allowed to call the controller action.
    pub policy: Option<String>,
    // what rules define when the requirement should be applied (default is always)
    pub filters: Option<Vec<Box<dyn IAuthRequirementFilter>>>
}

impl AuthorizeControllerActionFeature {
    // create a new instance of the feature.
    // roles: the roles that are allowed to call the controller action.
    // policy: the policy that is allowed to call the controller action.
    pub fn new(
        roles: Vec<String>,
        policy: Option<String>,
        filters: Option<Vec<Box<dyn IAuthRequirementFilter>>>,
    ) -> Self {
        Self {
            roles: roles,
            policy: policy,
            filters: filters,
        }
    }

    // create a new instance of the feature as a service for a service collection from a comma separated list of roles.
    // roles: the roles that are allowed to call the controller action.
    // policy: the policy that is allowed to call the controller action.
    pub fn new_parse(
        roles: String,
        policy: Option<String>,
        filters: Option<Vec<Box<dyn IAuthRequirementFilter>>>,
    ) -> Self {
        Self::new(roles.split(',').map(|s| s.to_string()).collect(), policy, filters)
    }

    // create a new instance of the feature as a service for a service collection.
    pub fn new_service(
        roles: Vec<String>,
        policy: Option<String>,
        filters: Option<Vec<Box<dyn IAuthRequirementFilter>>>,
    ) -> Rc<dyn IControllerActionFeature> {
        Rc::new(Self::new(roles, policy, filters))
    }

    // create a new instance of the feature as a service for a service collection from a comma separated list of roles.
    // roles: the roles that are allowed to call the controller action.
    // policy: the policy that is allowed to call the controller action.
    pub fn new_service_parse(
        roles: String,
        policy: Option<String>,
        filters: Option<Vec<Box<dyn IAuthRequirementFilter>>>,
    ) -> Rc<dyn IControllerActionFeature> {
        Rc::new(Self::new_parse(roles, policy, filters))
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

    fn invoke(self: &Self, _request_context: Rc<dyn IRequestContext>, _response_context: Rc<ResponseContext>, _services: &dyn IServiceCollection) -> Result<MiddlewareResult, Rc<dyn Error>> {
        Ok(MiddlewareResult::OkContinue)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
