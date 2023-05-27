use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use crate::auth::auth_role_json_file_dbset::JsonAuthRole;
use crate::auth::iauthroles_dbset_provider::IAuthRolesDbSetProvider;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::type_info::TypeInfo;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controller_action_features::authorize::AuthorizeControllerActionFeature;
use crate::controllers::icontroller::IController;

use crate::services::service_collection::{ IServiceCollection, ServiceCollection, ServiceCollectionExtensions };
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;


// this enum is used to indicate the result of an authorization rejection
#[derive(Debug)]
pub enum AuthRejectionReason {
    // the user is not authenticated for an unknown reason
    Other(String)
}

// this enum is used to indicate the result of an authorization check
pub enum AuthResult {
    // the user is authorized
    Ok,
    // the user is not authorized
    Rejection(AuthRejectionReason)
}

// cookie, encrypted post form data token, plaintext username/password if logging in
pub struct AuthenticationToken {
    // the name of the token
    pub key: String,
    // the value of the token
    pub value: String,
}

// this trait is used to provide claims for authorization
pub trait IAuthClaim {
    // gets the name of the claim
    fn get_name(self: &Self) -> String;
    // gets the tokens for the claim
    fn get_tokens(self: &Self) -> HashMap<String, String>;

    // gets whether the claim is an identifier
    fn is_identifier(self: &Self) -> bool;
    // gets whether the claim is a secret
    fn is_secret(self: &Self) -> bool;

    // gets the type info for the claim
    fn get_type_info(self: &Self) -> TypeInfo;
    // gets the type name for the claim
    fn get_type_name(self: &Self) -> String;

    // gets the string representation of the claim
    fn to_string(self: &Self) -> String;
}

// Convert input claims and tokens to usable claims and tokens
pub trait IAuthClaimTransformer {
    // transform the claims from the request into usable claims for authorization.
    // claims: the claims from the request.
    // request_context: the request context.
    // returns: the usable claims for authorization.
    fn transform_claims(self: &Self, claims: Vec<Rc<dyn IAuthClaim>>, request_context: Rc<dyn IRequestContext>) -> Vec<Rc<dyn IAuthClaim>>;
    
    // transform the tokens from the request into usable tokens for authorization.
    // tokens: the tokens from the request.
    // request_context: the request context.
    // returns: the usable tokens for authorization.
    fn transform_tokens(self: &Self, tokens: Vec<Rc<AuthenticationToken>>, request_context: Rc<dyn IRequestContext>) -> Vec<Rc<AuthenticationToken>>;

    // gets the type info for the claim transformer
    fn get_type_info(self: &Self) -> TypeInfo;
    // gets the type name for the claim transformer
    fn get_type_name(self: &Self) -> String;

    // gets the string representation of the claim transformer
    fn to_string(self: &Self) -> String;
}

// for testing, allow changing role from cookie
pub struct CookieRoleClaim {
    // the role
    pub role: String,
}

impl CookieRoleClaim {
    pub fn new(role: String) -> Self {
        Self { role: role }
    }
    
    // create a new instance of the service for the service collection.
    pub fn new_service(role: String) -> Rc<dyn IAuthClaim> {
        Rc::new(Self::new(role))
    }
}

impl IAuthClaim for CookieRoleClaim {
    fn get_name(self: &Self) -> String {
        "Role".to_string()
    }

    fn get_tokens(self: &Self) -> HashMap<String, String> {
        let mut tokens = HashMap::new();
        tokens.insert("Role".to_string(), self.role.clone());
        tokens
    }

    fn is_identifier(self: &Self) -> bool {
        false
    }

    fn is_secret(self: &Self) -> bool {
        false
    }

    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<CookieRoleClaim>()
    }

    fn get_type_name(self: &Self) -> String {
        nameof::name_of_type!(CookieRoleClaim).to_string()
    }

    fn to_string(self: &Self) -> String {
        format!("{} (role: {})", self.get_type_name(), self.role)
    }
}

// this struct is used to transform the cookie into a claim
pub struct CookieRoleClaimTransformer {

}

impl CookieRoleClaimTransformer {
    pub fn new() -> Self {
        Self {

        }
    }

    // create a new instance of the service for the service collection.
    pub fn new_service() -> Rc<dyn IAuthClaimTransformer> {
        Rc::new(Self::new())
    }
}

impl IAuthClaimTransformer for CookieRoleClaimTransformer {
    fn transform_claims(self: &Self, claims: Vec<Rc<dyn IAuthClaim>>, request_context: Rc<dyn IRequestContext>) -> Vec<Rc<dyn IAuthClaim>> {
        if let Some(cookies) = request_context.get_cookies_parsed() {
            if let Some(role) = cookies.get("role") {
                return claims.iter().cloned().chain(vec![CookieRoleClaim::new_service(role.clone())]).collect();
            }
        }
        claims
    }

    fn transform_tokens(self: &Self, tokens: Vec<Rc<AuthenticationToken>>, request_context: Rc<dyn IRequestContext>) -> Vec<Rc<AuthenticationToken>> {
        tokens
    }

    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<CookieRoleClaimTransformer>()
    }

    fn get_type_name(self: &Self) -> String {
        nameof::name_of_type!(CookieRoleClaimTransformer).to_string()
    }

    fn to_string(self: &Self) -> String {
        self.get_type_name()
    }
}

// this trait is used to provide authorization requirements.
pub trait IAuthRequirement {
    // invoke the authorization requirement check.
    // auth_claims: the claims to check.
    // roles: the roles to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn invoke(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, roles: Vec<String>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // gets the name of the authorization requirement.
    fn get_name(self: &Self) -> String;

    // gets the type info for the authorization requirement.
    fn get_type_info(self: &Self) -> TypeInfo;
    // gets the type name for the authorization requirement.
    fn get_type_name(self: &Self) -> String;

    // gets the string representation of the authorization requirement.
    fn to_string(self: &Self) -> String;
}

// this struct is used to check if the user has a role.
pub struct RoleAuthRequirement {}

impl RoleAuthRequirement {
    pub fn new() -> Self {
        Self {

        }
    }

    // create a new instance of the service for the service collection.
    pub fn new_service() -> Rc<dyn IAuthRequirement> {
        Rc::new(Self::new())
    }
}

impl IAuthRequirement for RoleAuthRequirement {
    fn invoke(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, roles: Vec<String>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        if roles.len() == 0 {
            return Ok(AuthResult::Ok);
        }

        let found_roles = auth_claims
            .iter()
            .filter(|x| x.as_ref().get_name() == "Role")
            .map(|x| x.as_ref().get_tokens().values().map(|x| x.clone()).collect::<Vec<String>>())
            .flatten()
            .collect::<Vec<String>>();

        // println!("found_roles ({}): {:?}", found_roles.len(), found_roles);
        for role in found_roles.iter() {
            if roles.contains(&role) {
                return Ok(AuthResult::Ok);
            }
        }

        Ok(AuthResult::Rejection(AuthRejectionReason::Other(format!("Role(s) required {:?} not found in authed role(s) {:?}", roles, found_roles))))
    }

    fn get_name(self: &Self) -> String {
        "Role".to_string()
    }

    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<RoleAuthRequirement>()
    }

    fn get_type_name(self: &Self) -> String {
        nameof::name_of_type!(RoleAuthRequirement).to_string()
    }

    fn to_string(self: &Self) -> String {
        self.get_type_name()
    }
}

// this trait is used to provide and apply authorization policies.
// a policy is a set of authorization requirements.
// a policy can be applied to a set of claims to check if the user is authorized.
pub trait IAuthorizationService {
    // authenticate the user for a role.
    // auth_claims: the claims to check.
    // role: the role to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_role(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, role: String, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // authenticate the user for a set of roles.
    // auth_claims: the claims to check.
    // roles: the roles to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_roles(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, roles: Vec<String>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // authenticate the user for a set of requirements.
    // auth_claims: the claims to check.
    // requirements: the requirements to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_requirements(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, requirements: Vec<Rc<dyn IAuthRequirement>>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // authenticate the user for a set of requirements by name.
    // auth_claims: the claims to check.
    // requirements: the requirements to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_requirements_by_name(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, requirements: Vec<String>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // authenticate the user for a policy.
    // auth_claims: the claims to check.
    // policy: the policy to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_policy(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, policy: Rc<dyn IAuthRequirement>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // authenticate the user for a policy by name.
    // auth_claims: the claims to check.
    // policy: the policy to check.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_policy_by_name(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, policy: String, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;

    // authenticate the user for an HTTP request.
    // controller: the controller.
    // request_context: the request context.
    // returns: the result of the authorization check.
    fn authenticate_http_request(self: &Self, controller: Rc<dyn IController>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>>;
    
    // sign in the user.
    fn sign_in(self: &Self);
    // sign out the user.
    fn sign_out(self: &Self);

    // get the policies.
    fn get_policies(self: &Self) -> Vec<Rc<dyn IAuthRequirement>>;
    // get the roles.
    fn get_roles(self: &Self) -> Vec<String>;
    // get the claim providers.
    fn get_auth_claim_providers(self: &Self) -> Vec<String>;
    // get the claim transformers.
    fn get_claim_transformers(self: &Self) -> Vec<Rc<dyn IAuthClaimTransformer>>;
    
    // get the type info for the authorization service.
    fn get_type_info(self: &Self) -> TypeInfo;
    // get the type name for the authorization service.
    fn get_type_name(self: &Self) -> String;

    // get the string representation of the authorization service.
    fn to_string(self: &Self) -> String;
}

// this struct is used to provide authorization services.
pub struct AuthorizationService {
    // the authorization policies.
    pub policies: HashMap<String, Rc<dyn IAuthRequirement>>,
    // the claim transformers.
    pub claim_transformers: Vec<Rc<dyn IAuthClaimTransformer>>,
    // the auth roles dbset provider.
    pub authrole_dbset_provider: Rc<dyn IAuthRolesDbSetProvider>,
}

impl AuthorizationService {
    // creates a new instance of the service.
    // authrole_dbset_provider: the auth roles dbset provider.
    pub fn new(
        authrole_dbset_provider: Rc<dyn IAuthRolesDbSetProvider>
    ) -> Self {
        Self {
            authrole_dbset_provider: authrole_dbset_provider,
            policies: vec![
                RoleAuthRequirement::new_service()
            ].iter().map(|x| (x.get_name(), x.clone())).collect(),
            claim_transformers: vec![
                CookieRoleClaimTransformer::new_service()
            ],
        }
    }

    // creates a new instance of the service for the service collection.
    // services: the service collection.
    // returns: a vector containing the new instance of the service.
    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IAuthRolesDbSetProvider>(services)
        )) as Rc<dyn IAuthorizationService>)]
    }

    // adds the service to the service collection.
    // services: the service collection.
    // returns: nothing.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IAuthorizationService>(), Self::new_service, ServiceScope::Singleton));
    }
}

impl IAuthorizationService for AuthorizationService {
    fn authenticate_role(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, role: String, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        Ok(AuthResult::Ok)
    }

    fn authenticate_roles(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, roles: Vec<String>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        self.policies.get("Role").unwrap().invoke(auth_claims, roles, request_context)
    }

    fn authenticate_requirements(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, requirements: Vec<Rc<dyn IAuthRequirement>>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        Ok(AuthResult::Ok)
    }

    fn authenticate_requirements_by_name(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, requirements: Vec<String>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        Ok(AuthResult::Ok)
    }

    fn authenticate_policy(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, policy: Rc<dyn IAuthRequirement>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        Ok(AuthResult::Ok)
    }

    fn authenticate_policy_by_name(self: &Self, auth_claims: Vec<Rc<dyn IAuthClaim>>, policy: String, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        Ok(AuthResult::Ok)
    }

    fn get_policies(self: &Self) -> Vec<Rc<dyn IAuthRequirement>> {
        vec![]
    }

    fn get_roles(self: &Self) -> Vec<String> {
        self.authrole_dbset_provider.get_authroles_dbset().get_all_any().iter().map(|x| x.downcast_ref::<JsonAuthRole>().unwrap().name.clone()).collect()
    }

    fn get_auth_claim_providers(self: &Self) -> Vec<String> {
        vec![]
    }

    fn get_claim_transformers(self: &Self) -> Vec<Rc<dyn IAuthClaimTransformer>> {
        self.claim_transformers.clone()
    }

    fn authenticate_http_request(self: &Self, controller: Rc<dyn IController>, request_context: Rc<dyn IRequestContext>) -> Result<AuthResult, Box<dyn Error>> {
        let mut required_roles = vec![];
        let mut required_policies = vec![];

        let action_features = request_context.get_controller_action().get_features();
        let controller_features = controller.get_features();

        let find_my_feature: Vec<Rc<dyn IControllerActionFeature>> = controller_features
            .iter()
            .chain(
                action_features.iter()
            )
            //look for allow anonymous
            .filter(|x| x.get_name() == nameof::name_of_type!(AuthorizeControllerActionFeature).to_string())
            .cloned()
            .collect();

        // gather roles from requirements on action
        if find_my_feature.len() > 0 {
            // collect required roles and policies
            for it in find_my_feature.iter() {
                let req = it.as_any().downcast_ref::<AuthorizeControllerActionFeature>().unwrap();
                let mut apply_requirement = true;
                if let Some(req_filters) = &req.filters {
                    for req_filter in req_filters {
                        if !req_filter.use_requirement() {
                            apply_requirement = false;
                        }
                    }
                }
                if apply_requirement {
                    required_roles.extend_from_slice(&req.roles);
                    if let Some(policy) = &req.policy {
                        required_policies.push(policy);
                    }
                }
            }
        }

        // println!("Authenticating {:?} against roles: {:?}, policies {:?}", request_context.connection_context.get_remote_addr(), required_roles, required_policies);
        // let mut auth_result: Option<AuthResult> = None;
        let mut claims = request_context.as_ref().get_auth_claims();//.borrow().clone();
        let mut tokens = vec![];

        for it in self.claim_transformers.iter() {
            claims = it.transform_claims(claims, request_context.clone());
            tokens = it.transform_tokens(tokens, request_context.clone());
        }

        if required_roles.len() > 0 {
            match self.authenticate_roles(claims.clone(), required_roles.clone(), request_context.clone())? {
                AuthResult::Ok => {
                    // println!("authorized! :)");
                },
                AuthResult::Rejection(reason) => {
                    return Ok(AuthResult::Rejection(reason));
                }
            }
        }

        if required_policies.len() > 0 {
            for policy in required_policies {
                match self.authenticate_policy_by_name(claims.clone(), policy.clone(), request_context.clone())? {
                    AuthResult::Ok => {
                        // println!("authorized! :)");
                    },
                    AuthResult::Rejection(reason) => {
                        return Ok(AuthResult::Rejection(reason));
                    }
                }
            }
        }
        
        Ok(AuthResult::Ok)
    }
    
    fn sign_in(self: &Self) {
        
    }

    fn sign_out(self: &Self) {
        
    }

    fn get_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<AuthorizationService>()
    }

    fn get_type_name(self: &Self) -> String {
        nameof::name_of_type!(AuthorizationService).to_string()
    }

    fn to_string(self: &Self) -> String {
        self.get_type_name()
    }
}