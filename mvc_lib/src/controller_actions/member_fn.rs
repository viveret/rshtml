use std::borrow::Cow;
use std::borrow::Borrow;
use std::error::Error;
use std::rc::Rc;

use http::Method;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::irequest_context::IRequestContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controllers::icontroller::IController;
use crate::core::type_info::TypeInfo;
use crate::model_binder::imodel::AnyIModel;
use crate::model_binder::imodel::IModel;
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::routing::action_path::ActionPath;
use crate::routing::path_builder::ActionPathBuilder;
use crate::services::service_collection::IServiceCollection;

use crate::controller_actions::controller_action::IControllerAction;
use crate::controller_actions::controller_action::IControllerActionExtensions;
use crate::controller_actions::route_pattern::ControllerActionRoutePattern;


// this struct represents a controller action that is a member function of a controller.
// T - the type of the controller.
#[derive(Clone)]
pub struct ControllerActionMemberFn<T: 'static + IController> {
    // the member function to invoke. this is a function pointer.
    // the first argument is the controller instance, the second argument is the controller context, and the third argument is the service collection.
    // the return value is an optional action result.
    // the function pointer is wrapped in an Rc so that it can be cloned.
    pub member_fn_validated: Option<Box<fn(self_arg: &T, model: ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>>,
    pub member_fn_validated_typed: Option<Rc<dyn Fn(&T, ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>>,
    pub member_fn_not_validated: Option<Box<fn(self_arg: &T, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>>,
    // the name of the action (the name of the member function).
    pub name: Cow<'static, str>,
    // the name of the controller.
    pub controller_name: Cow<'static, str>,
    // the name of the area.
    pub area_name: String,
    // the route pattern for the action.
    pub route_pattern: Rc<ControllerActionRoutePattern>,
    // the http methods allowed for the action.
    pub http_methods_allowed: Vec<Method>,
    // the features for the action.
    pub features: Vec<Rc<dyn IControllerActionFeature>>,
    // whether or not the model should be validated.
    pub should_validate_model: bool,
    // the type of the model.
    pub model_type: Option<Box<TypeInfo>>,
}

impl<T: IController> ControllerActionMemberFn<T> {
    // create a new instance of the action.
    // http_methods_allowed: the http methods allowed for the action.
    // features: the features for the action.
    // route_pattern: the route pattern for the action.
    // name: the name of the action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // should_validate_model: whether or not the model should be validated.
    // member_fn: the member function to invoke. this is a function pointer.
    pub fn new(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        should_validate_model: bool,
        member_fn_validated: Option<Box<fn(self_arg: &T, model: ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>>,
        member_fn_not_validated: Option<Box<fn(self_arg: &T, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>>,
    ) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: area_name,
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(route_pattern)),
            member_fn_validated: member_fn_validated,
            member_fn_not_validated: member_fn_not_validated,
            member_fn_validated_typed: None,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: should_validate_model,
            model_type: None,
        }
    }
    
    // create a new instance of the action that is validated.
    // http_methods_allowed: the http methods allowed for the action.
    // features: the features for the action.
    // route_pattern: the route pattern for the action.
    // name: the name of the action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // member_fn: the member function to invoke. this is a function pointer.
    pub fn new_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        member_fn: Box<fn(self_arg: &T, model: ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        Self::new(
            http_methods_allowed,
            features,
            route_pattern,
            name,
            controller_name,
            area_name,
            true,
            Some(member_fn),
            None
        )
    }
    
    // create a new instance of the action that is not validated.
    // http_methods_allowed: the http methods allowed for the action.
    // features: the features for the action.
    // route_pattern: the route pattern for the action.
    // name: the name of the action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // member_fn: the member function to invoke. this is a function pointer.
    pub fn new_not_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        member_fn: Box<fn(self_arg: &T, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        Self::new(
            http_methods_allowed,
            features,
            route_pattern,
            name,
            controller_name,
            area_name,
            false,
            None,
            Some(member_fn),
        )
    }

    pub fn new_validated_typed<TModel: 'static + IModel + Clone>(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        member_fn: Box<fn(&T, ModelValidationResult<TModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        // let x = move |self_arg, model: ModelValidationResult<AnyIModel>, a, services| {
        //     // match model
        //     (member_fn.as_ref())(self_arg, model.downcast(), a, services)
        // };

        Self {
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or_default(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(route_pattern)),
            name: name,
            controller_name: controller_name,
            area_name: area_name,
            should_validate_model: true,
            member_fn_validated: None,
            member_fn_validated_typed: Some(Rc::new(move |self_arg, model: ModelValidationResult<AnyIModel>, a, services| {
                // match model
                (member_fn.as_ref())(self_arg, model.downcast(), a, services)
            })),
            member_fn_not_validated: None,
            model_type: Some(Box::new(TypeInfo::of::<TModel>())), // might need to be rc_of
        }
    }
    
    // create a new instance of the action that is validated and has no area.
    // http_methods_allowed: the http methods allowed for the action.
    // features: the features for the action.
    // route_pattern: the route pattern for the action.
    // name: the name of the action.
    // controller_name: the name of the controller.
    // should_validate_model: whether or not the model should be validated.
    // member_fn: the member function to invoke. this is a function pointer.
    pub fn new_default_area(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        should_validate_model: bool,
        member_fn: Box<fn(self_arg: &T, model: ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        Self::new(http_methods_allowed, features, route_pattern, name, controller_name, String::new(), should_validate_model, Some(member_fn), None)
        //     name: name,
        //     controller_name: controller_name,
        //     area_name: String::new(),
        //     route_pattern: ,
        //     member_fn: member_fn,
        //     http_methods_allowed: http_methods_allowed,
        //     features: features.unwrap_or(vec![]),
        //     should_validate_model: should_validate_model,
        //     model_type: None,
        // }
    }
}

impl<T: 'static + IController> IControllerAction for ControllerActionMemberFn<T> {
    fn invoke(self: &Self, controller_context: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>> {
        let base_controller = controller_context.get_controller();
        let controller = base_controller.as_ref().as_any().downcast_ref::<T>().expect("Could not downcast base_controller to T where T: IController.");

        let result_option = if self.should_validate_model {
            if let Some(member_fn_validated) = &self.member_fn_validated {
                if let Some(model) = controller_context.get_request_context().get_model_validation_result() {
                    (member_fn_validated)(controller, model, controller_context, services)
                } else {
                    (member_fn_validated)(controller, ModelValidationResult::OkNone, controller_context, services)
                }
            } else if let Some(member_fn_validated_typed) = &self.member_fn_validated_typed {
                if let Some(model) = controller_context.get_request_context().get_model_validation_result() {
                    (member_fn_validated_typed)(controller, model, controller_context, services)
                } else {
                    (member_fn_validated_typed)(controller, ModelValidationResult::OkNone, controller_context, services)
                }
            } else {
                Err(Rc::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Could not find member_fn_validated for action {}", self.name))) as Rc<dyn Error>)
            }
        } else {
            (self.member_fn_not_validated.as_ref().expect("member_fn_not_validated not set"))(controller, controller_context, services)
        }?;

        if let Some(result) = result_option {
            controller_context.get_response_context().set_action_result(Some(result));
        }
        Ok(())
    }

    fn is_route_match(self: &Self, request_context: &dyn IRequestContext) -> Result<bool, Rc<dyn Error>> {
        if !IControllerActionExtensions::is_method_match(self, request_context) {
            return Ok(false);
        }

        let path = request_context.get_path().as_str().trim();
        let route_pattern = self.get_route_pattern();

        // println!("Testing path {} against pattern {}", path, route_pattern.raw);

        if route_pattern.raw.ends_with("..") {
            Ok(path.starts_with(&route_pattern.raw[..route_pattern.raw.len() - 2]))
        } else {
            let r = path == route_pattern.raw;
            // println!("path == route_pattern.raw -> {}", r);
            Ok(r)
        }
    }

    fn get_name(self: &Self) -> Cow<'static, str> {
        self.name.clone()
    }

    fn get_controller_name(self: &Self) -> Cow<'static, str> {
        self.controller_name.clone()
    }

    fn get_area_name(self: &Self) -> String {
        self.area_name.clone()
    }

    fn get_route_pattern(self: &Self) -> Rc<ControllerActionRoutePattern> {
        self.route_pattern.clone()
    }

    fn to_string(self: &Self) -> String {
        let methods = self.get_http_methods_allowed();
        let methods_str = if methods.len() > 0 { format!("{:?}", methods) } else { "[[*]]".to_string() };
        format!("{} {} mapped to {}", methods_str, self.get_path(), self.route_pattern.raw)
    }

    fn get_path(self: &Self) -> ActionPath {
        let mut path_builder = ActionPathBuilder::new();
        path_builder
            .add(&self.area_name, false)
            .add(self.controller_name.borrow(), true)
            .add(&self.name, false)
            .as_action_path()
    }

    fn get_http_methods_allowed(self: &Self) -> Vec<Method> {
        self.http_methods_allowed.clone()
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        self.features.iter().cloned().collect()
    }

    fn get_should_validate_model(self: &Self) -> bool {
        self.should_validate_model
    }

    fn get_model_type(self: &Self) -> Option<Box<crate::core::type_info::TypeInfo>> {
        self.model_type.clone()
    }
}
