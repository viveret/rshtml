use std::any::Any;
use std::error::Error;
use std::rc::Rc;

use mvc_lib::action_results::iaction_result::IActionResult;
use mvc_lib::contexts::controller_context::ControllerContext;
use mvc_lib::contexts::controller_context::IControllerContext;
use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::auth::auth_role_json_file_dbset::AuthRoleJsonFileDbSet;
use mvc_lib::auth::auth_role_json_file_dbset::JsonAuthRole;
use mvc_lib::auth::iauthroles_dbset_provider::IAuthRolesDbSetProvider;
use mvc_lib::entity::idbset::IDbSet;
use mvc_lib::entity::idbset::JsonFileDbSet;
use mvc_lib::services::authorization_service::IAuthorizationService;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_collection::ServiceCollectionExtensions;


use mvc_lib::action_results::view_result::ViewResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controller_actions::closure::ControllerActionClosure;
use mvc_lib::controller_actions::member_fn::ControllerActionMemberFn;

use mvc_lib::controller_action_features::local_host_only::LocalHostOnlyControllerActionFeature;
use mvc_lib::controller_action_features::authorize::AuthorizeControllerActionFeature;

use crate::view_models::authroles::{ IndexViewModel };

#[derive(Clone)]
pub struct AuthRolesController {
    authroles_dbset: Rc<dyn IAuthRolesDbSetProvider>,
    auth_service: Rc<dyn IAuthorizationService>,
}

impl AuthRolesController {
    pub fn new(
        authroles_dbset: Rc<dyn IAuthRolesDbSetProvider>,
        auth_service: Rc<dyn IAuthorizationService>,
    ) -> Self {
        Self {
            authroles_dbset: authroles_dbset,
            auth_service: auth_service,
        }
    }

    pub fn new_service(services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new(
            ServiceCollectionExtensions::get_required_single::<dyn IAuthRolesDbSetProvider>(services),
            ServiceCollectionExtensions::get_required_single::<dyn IAuthorizationService>(services),
        )) as Rc<dyn IController>)]
    }

    pub fn get_index(controller: Box<AuthRolesController>, _controller_ctx: Rc<ControllerContext>, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
        let roles = controller.authroles_dbset.get_authroles_dbset()
            .as_any(TypeInfo::of::<JsonFileDbSet<JsonAuthRole>>())
            .downcast_ref::<AuthRoleJsonFileDbSet>()
            .unwrap()
            .get_all()
            .iter().cloned().collect();
        let view_model = Box::new(Rc::new(IndexViewModel::new(roles)));
        Ok(Some(Rc::new(ViewResult::new("views/authroles/index.rs".to_string(), view_model))))
    }
}

impl IController for AuthRolesController {
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(AuthRolesController)
    }
    
    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        vec![
            Rc::new(ControllerActionMemberFn::<Box<AuthRolesController>>::new(vec![], None, "/auth-roles".to_string(), "Index".to_string(), self.get_type_name(), self.get_route_area(), Box::new(self.clone()), Self::get_index)),
        ]
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![
            AuthorizeControllerActionFeature::new_service_parse("admin,dev,owner".to_string(), None),
            LocalHostOnlyControllerActionFeature::new_service()
        ]
    }
}