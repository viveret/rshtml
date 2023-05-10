use std::any::Any;
use std::borrow::Cow;
use std::error::Error;
use std::rc::Rc;

use http::Method;
use mvc_lib::action_results::iaction_result::IActionResult;
use mvc_lib::contexts::controller_context::ControllerContext;
use mvc_lib::contexts::controller_context::IControllerContext;
use mvc_lib::core::type_info::TypeInfo;
use mvc_lib::auth::auth_role_json_file_dbset::AuthRoleJsonFileDbSet;
use mvc_lib::auth::auth_role_json_file_dbset::JsonAuthRole;
use mvc_lib::auth::iauthroles_dbset_provider::IAuthRolesDbSetProvider;
use mvc_lib::entity::idbset::IDbSet;
use mvc_lib::entity::json_file_dbset::JsonFileDbSet;
use mvc_lib::services::authorization_service::IAuthorizationService;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::services::service_collection::ServiceCollectionExtensions;


use mvc_lib::action_results::view_result::ViewResult;

use mvc_lib::controllers::icontroller::IController;

use mvc_lib::controller_action_features::controller_action_feature::IControllerActionFeature;
use mvc_lib::controller_actions::builder::{ ControllerActionsBuilder, ControllerActionBuilder };
use mvc_lib::controller_actions::controller_action::IControllerAction;
use mvc_lib::controller_actions::closure::ControllerActionClosure;
use mvc_lib::controller_actions::member_fn::ControllerActionMemberFn;

use mvc_lib::controller_action_features::local_host_only::LocalHostOnlyControllerActionFeature;
use mvc_lib::controller_action_features::authorize::AuthorizeControllerActionFeature;

// use mvc_macro_lib::action_member;

use crate::view_models::authroles::{ IndexViewModel, AddViewModel };

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

    pub fn get_roles(self: &Self) -> Vec<JsonAuthRole> {
        self.authroles_dbset.get_authroles_dbset()
            .as_any(TypeInfo::of::<JsonFileDbSet<JsonAuthRole>>())
            .downcast_ref::<AuthRoleJsonFileDbSet>()
            .unwrap()
            .get_all()
            .iter().cloned().collect()
    }

    pub fn get_index(controller: &AuthRolesController, _controller_ctx: Rc<ControllerContext>, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
        let roles = controller.get_roles();
        let view_model = Box::new(Rc::new(IndexViewModel::new(roles)));
        Ok(Some(Rc::new(ViewResult::new("views/authroles/index.rs".to_string(), view_model))))
    }

    pub fn get_add(controller: &AuthRolesController, _controller_ctx: Rc<ControllerContext>, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
        let view_model = Box::new(Rc::new(AddViewModel::new(String::new(), None)));
        Ok(Some(Rc::new(ViewResult::new("views/authroles/add.rs".to_string(), view_model))))
    }

    pub fn post_add(controller: &AuthRolesController, controller_ctx: Rc<ControllerContext>, _services: &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
        let input_model = controller_ctx.get_request_context().get_model_validation_result();
        let new_role = controller_ctx.request_context.get_query().get("role"); // to do: this needs to use query parameter
        let view_model = Box::new(Rc::new(
            if let Some(new_role) = new_role {
                if new_role.is_empty() {
                    AddViewModel::new_error(new_role, "Role is blank")
                } else {
                    let role = JsonAuthRole::parse_str(&new_role);
                    let current_roles = controller.get_roles();
                    if current_roles.contains(&role) {
                        AddViewModel::new_error(new_role, "Role already exists")
                    } else {
                        AddViewModel::new_ok(new_role, "Successfully created role")
                    }
                }
            } else {
                AddViewModel::new_error(String::new(), "Role missing from query string")
            }
        ));
        Ok(Some(Rc::new(ViewResult::new("views/authroles/add.rs".to_string(), view_model))))
    }
}

impl IController for AuthRolesController {
    fn get_route_area(self: &Self) -> String {
        String::new()
    }

    fn get_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(AuthRolesController)
    }

    fn get_controller_name(self: &Self) -> Cow<'static, str> {
        Cow::Borrowed(nameof::name_of_type!(AuthRolesController))
    }
    
    fn get_actions(self: &Self) -> Vec<Rc<dyn IControllerAction>> {
        let actions_builder = ControllerActionsBuilder::new(self);
        
        actions_builder.add("/dev/auth-roles")
            .methods(&[Method::GET])
            .set_name("index")
            .set_controller_name(self.get_controller_name())
            .set_member_fn(Self::get_index);


        actions_builder.add("/dev/auth-roles/add")
            .methods(&[Method::GET])
            .set_name("add")
            .set_controller_name(self.get_controller_name())
            .set_member_fn(Self::get_add);

        actions_builder.add("/dev/auth-roles/add")
                .methods(&[Method::POST])
                .set_name("add_post")
                .set_controller_name(self.get_controller_name())
                .set_member_fn(Self::post_add);

        actions_builder.build()
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![
            AuthorizeControllerActionFeature::new_service_parse("admin,dev,owner".to_string(), None),
            LocalHostOnlyControllerActionFeature::new_service()
        ]
    }

    fn as_any(self: &Self) -> &dyn Any {
        self
    }
}