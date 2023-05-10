use std::borrow::Cow;
use std::error::Error;
use std::rc::Rc;

use as_any::Downcast;
use http::Method;

use crate::action_results::iaction_result::IActionResult;

use crate::action_results::iaction_result::IActionResultToAny;
use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::request_context::RequestContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controllers::icontroller::IController;
use crate::services::service_collection::IServiceCollection;

use crate::controller_actions::controller_action::IControllerAction;
use crate::controller_actions::controller_action::IControllerActionExtensions;
use crate::controller_actions::route_pattern::ControllerActionRoutePattern;


#[derive(Clone)]
pub struct ControllerActionMemberFn<T> {
    pub member_fn: fn(self_arg: &T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>,
    pub name: String,
    pub controller_name: Cow<'static, str>,
    pub area_name: String,
    pub route_pattern: Rc<ControllerActionRoutePattern>,
    pub http_methods_allowed: Vec<Method>,
    pub features: Vec<Rc<dyn IControllerActionFeature>>,
    pub should_validate_model: bool,
}

impl<T> ControllerActionMemberFn<T> {
    pub fn new(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: String,
        name: String,
        controller_name: Cow<'static, str>,
        area_name: String,
        should_validate_model: bool,
        member_fn: fn(self_arg: &T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>
    ) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: area_name,
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(&route_pattern)),
            member_fn: member_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: should_validate_model,
        }
    }
    
    pub fn new_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: String,
        name: String,
        controller_name: Cow<'static, str>,
        area_name: String,
        member_fn: fn(self_arg: &T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>
    ) -> Self {
        Self::new(
            http_methods_allowed,
            features,
            route_pattern,
            name,
            controller_name,
            area_name,
            true,
            member_fn,
        )
    }
    
    pub fn new_not_validated(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: String,
        name: String,
        controller_name: Cow<'static, str>,
        area_name: String,
        member_fn: fn(self_arg: &T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>
    ) -> Self {
        Self::new(
            http_methods_allowed,
            features,
            route_pattern,
            name,
            controller_name,
            area_name,
            false,
            member_fn,
        )
    }
    
    pub fn new_default_area(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: String,
        name: String,
        controller_name: Cow<'static, str>,
        should_validate_model: bool,
        member_fn: fn(self_arg: &T, Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>>
    ) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: String::new(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(&route_pattern)),
            member_fn: member_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
            should_validate_model: should_validate_model,
        }
    }
}

impl<T: 'static + IController> IControllerAction for ControllerActionMemberFn<T> {
    fn invoke(self: &Self, controller_context: Rc<ControllerContext>, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>> {
        let x = controller_context.get_controller();

        // for some reason the below line is not working as described in the solution
        // here on stack overflow: https://stackoverflow.com/a/40033391/11765486
        // and also described here: https://users.rust-lang.org/t/downcast-rc-any-to-rc-t/29230/13
        // A slight variation on that would be to add a method fn as_any(&self) -> &Any { self } 
        // to AstNode, and then you can call Expression methods (that take &self) by writing 
        // node.as_any().downcast_ref::<Expression>().method_on_expression(). But there is 
        // currently no way to (safely) upcast an Rc<dyn Trait> to an Rc<dyn Any> (this 
        // could change in the future, though).
        let typed_controller = x.as_any().downcast_ref::<T>().unwrap();
        let result_option = (self.member_fn)(typed_controller, controller_context.clone(), services)?;
        if let Some(result) = result_option {
            controller_context.set_action_result(Some(result));
        }
        Ok(())
    }

    fn is_route_match(self: &Self, request_context: Rc<dyn IRequestContext>) -> Result<bool, Box<dyn Error>> {
        if !IControllerActionExtensions::is_method_match(self, request_context.clone()) {
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

    fn get_name(self: &Self) -> String {
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

    fn get_path(self: &Self) -> String {
        format!("{}/{}/{}", self.area_name, self.controller_name, self.name)
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
}
