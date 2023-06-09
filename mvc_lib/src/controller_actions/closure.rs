use std::borrow::Cow;
use std::borrow::Borrow;
use std::error::Error;
use std::rc::Rc;

use http::Method;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::irequest_context::IRequestContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
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


// this struct represents a controller action that is implemented by a closure.
// this struct is useful for creating controller actions that are not part of a controller and could be a part of an area.
// this struct is also useful for creating controller actions that just need to do something simple.
pub struct ControllerActionClosure {
    // the closure that implements the controller action.
    pub closure_fn: Rc<dyn Fn(ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>,
    // the name of the controller action.
    pub name: Cow<'static, str>,
    // the name of the controller.
    pub controller_name: Cow<'static, str>,
    // the name of the area.
    pub area_name: String,
    // the route pattern for the controller action.
    pub route_pattern: Rc<ControllerActionRoutePattern>,
    // the HTTP methods allowed for the controller action.
    pub http_methods_allowed: Vec<Method>,
    // the controller action features for the controller action.
    pub features: Vec<Rc<dyn IControllerActionFeature>>,
    // whether or not the model should be validated for the controller action.
    pub should_validate_model: bool,
}

impl ControllerActionClosure {
    // create a new instance of the action.
    // http_methods_allowed: the HTTP methods allowed for the controller action.
    // features: the controller action features for the controller action.
    // route_pattern: the route pattern for the controller action.
    // name: the name of the controller action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // should_validate_model: whether or not the model should be validated for the controller action.
    // closure_fn: the closure that implements the controller action.
    pub fn new(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        should_validate_model: bool,
        closure_fn: Rc<dyn Fn(ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: area_name,
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(route_pattern)),
            closure_fn: closure_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: should_validate_model,
        }
    }
    
    // create a new instance of the action that is validated.
    // http_methods_allowed: the HTTP methods allowed for the controller action.
    // features: the controller action features for the controller action.
    // route_pattern: the route pattern for the controller action.
    // name: the name of the controller action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // closure_fn: the closure that implements the controller action.
    pub fn new_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        closure_fn: Rc<dyn Fn(ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        Self::new(
            http_methods_allowed,
            features,
            route_pattern,
            name,
            controller_name,
            area_name,
            true,
            closure_fn,
        )
    }
    
    // create a new instance of the action that is not validated.
    // http_methods_allowed: the HTTP methods allowed for the controller action.
    // features: the controller action features for the controller action.
    // route_pattern: the route pattern for the controller action.
    // name: the name of the controller action.
    // controller_name: the name of the controller.
    // area_name: the name of the area.
    // closure_fn: the closure that implements the controller action.
    pub fn new_not_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        area_name: String,
        closure_fn: &'static dyn Fn(&dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>
    ) -> Self {
        Self::new(
            http_methods_allowed,
            features,
            route_pattern,
            name,
            controller_name,
            area_name,
            false,
            Rc::new(|_, a, b| (closure_fn)(a, b)),
        )
    }
    
    // create a new instance of the action that is not part of an area.
    // http_methods_allowed: the HTTP methods allowed for the controller action.
    // features: the controller action features for the controller action.
    // route_pattern: the route pattern for the controller action.
    // name: the name of the controller action.
    // controller_name: the name of the controller.
    // should_validate_model: whether or not the model should be validated for the controller action.
    // closure_fn: the closure that implements the controller action.
    pub fn new_default_area(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        should_validate_model: bool,
        closure_fn: Rc<dyn Fn(ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: String::new(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(route_pattern)),
            closure_fn: closure_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: should_validate_model,
        }
    }
    
    // create a new instance of the action that is validated and not part of an area.
    // http_methods_allowed: the HTTP methods allowed for the controller action.
    // features: the controller action features for the controller action.
    // route_pattern: the route pattern for the controller action.
    // name: the name of the controller action.
    // controller_name: the name of the controller.
    // closure_fn: the closure that implements the controller action.
    pub fn new_default_area_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        closure_fn: Rc<dyn Fn(ModelValidationResult<AnyIModel>, &dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>>
    ) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: String::new(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(route_pattern)),
            closure_fn: closure_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: true,
        }
    }
    
    // create a new instance of the action that is not validated and not part of an area.
    // http_methods_allowed: the HTTP methods allowed for the controller action.
    // features: the controller action features for the controller action.
    // route_pattern: the route pattern for the controller action.
    // name: the name of the controller action.
    // controller_name: the name of the controller.
    // closure_fn: the closure that implements the controller action.
    pub fn new_default_area_not_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: Cow<'static, str>,
        name: Cow<'static, str>,
        controller_name: Cow<'static, str>,
        closure_fn: &'static dyn Fn(&dyn IControllerContext, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Rc<dyn Error>>
    ) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: String::new(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(route_pattern)),
            closure_fn: Rc::new(|_, x, y| (closure_fn)(x, y)),
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: false,
        }
    }

}

impl IControllerAction for ControllerActionClosure {
    fn invoke(self: &Self, controller_context: &dyn IControllerContext, services: &dyn IServiceCollection) -> Result<(), Rc<dyn Error>> {
        let model = if self.should_validate_model {
            controller_context.get_request_context().get_model_validation_result()
        } else {
            None
        };

        let result_option = if let Some(model) = model {
            (self.closure_fn)(model, controller_context, services)
        } else {
            (self.closure_fn)(ModelValidationResult::OkNone, controller_context, services)
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
        Some(Box::new(TypeInfo::of::<dyn IModel>()))
    }
}
