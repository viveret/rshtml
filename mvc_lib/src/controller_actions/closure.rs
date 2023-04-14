use std::error::Error;
use std::rc::Rc;

use http::Method;

use crate::action_results::iaction_result::IActionResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::request_context::RequestContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::services::service_collection::IServiceCollection;

use crate::controller_actions::controller_action::IControllerAction;
use crate::controller_actions::route_pattern::ControllerActionRoutePattern;


pub struct ControllerActionClosure<T> where T: Fn(Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
    pub closure_fn: T,
    pub name: String,
    pub controller_name: &'static str,
    pub area_name: String,
    pub route_pattern: Rc<ControllerActionRoutePattern>,
    pub http_methods_allowed: Vec<Method>,
    pub features: Vec<Rc<dyn IControllerActionFeature>>,
}

impl<T> ControllerActionClosure<T> where T: Fn(Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
    pub fn new(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: String,
        name: String,
        controller_name: &'static str,
        area_name: String,
        closure_fn: T) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: area_name,
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(&route_pattern)),
            closure_fn: closure_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
        }
    }
    
    pub fn new_default_area(
        http_methods_allowed: Vec<Method>,
        features: Option<Vec<Rc<dyn IControllerActionFeature>>>,
        route_pattern: String,
        name: String,
        controller_name: &'static str,
        closure_fn: T) -> Self {
        Self {
            name: name,
            controller_name: controller_name,
            area_name: String::new(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(&route_pattern)),
            closure_fn: closure_fn,
            http_methods_allowed: http_methods_allowed,
            features: features.unwrap_or(vec![]),
        }
    }
}

impl<T> IControllerAction for ControllerActionClosure<T> where T: Fn(Rc<ControllerContext>, &dyn IServiceCollection) -> Result<Option<Rc<dyn IActionResult>>, Box<dyn Error>> {
    fn invoke(self: &Self, controller_context: Rc<ControllerContext>, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>> {
        let result_option = (self.closure_fn)(controller_context.clone(), services)?;
        if let Some(result) = result_option {
            controller_context.set_action_result(Some(result));
        }

        Ok(())
    }

    fn is_route_match(self: &Self, request_context: Rc<RequestContext>) -> Result<bool, Box<dyn Error>> {
        let path = request_context.path.as_str().trim();
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

    fn get_controller_name(self: &Self) -> &'static str {
        self.controller_name
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
}
