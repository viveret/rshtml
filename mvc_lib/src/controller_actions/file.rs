use std::borrow::Cow;
use std::error::Error;

use std::rc::Rc;

use http::Method;

use crate::action_results::file_result::FileResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::request_context::RequestContext;

use crate::controller_action_features::controller_action_feature::IControllerActionFeature;
use crate::controller_actions::route_pattern::ControllerActionRoutePattern;
use crate::controller_actions::controller_action::IControllerAction;
use crate::controller_actions::controller_action::IControllerActionExtensions;

use crate::services::service_collection::IServiceCollection;


pub struct ControllerActionFileResult {
    pub file_path: String,
    pub name: String,
    pub controller_name: Cow<'static, str>,
    pub area_name: String,
    pub route_pattern: Rc<ControllerActionRoutePattern>,
}

impl ControllerActionFileResult {
    pub fn new(
        file_path: String,
        route_pattern: String,
        name: String,
        controller_name: Cow<'static, str>,
        area_name: String,
        ) -> Self {
        Self {
            file_path: file_path,
            name: name,
            controller_name: controller_name,
            area_name: area_name,
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(&route_pattern)),
        }
    }
    
    pub fn new_default_area(
        file_path: String,
        route_pattern: String,
        name: String,
        controller_name: Cow<'static, str>,
        ) -> Self {
        Self {
            file_path: file_path,
            name: name,
            controller_name: controller_name,
            area_name: String::new(),
            route_pattern: Rc::new(ControllerActionRoutePattern::parse(&route_pattern)),
        }
    }
}

impl IControllerAction for ControllerActionFileResult {
    fn invoke(self: &Self, controller_context: Rc<ControllerContext>, services: &dyn IServiceCollection) -> Result<(), Box<dyn Error>> {
        let result_option = Some(Rc::new(FileResult::new(self.file_path.clone(), None)));
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
        format!("[{:?}] {} mapped to {} (which is mapped to file {})", self.get_http_methods_allowed(), self.get_path(), self.route_pattern.raw, self.file_path)
    }

    fn get_path(self: &Self) -> String {
        format!("{}/{}/{}", self.area_name, self.controller_name, self.name)
    }

    fn get_http_methods_allowed(self: &Self) -> Vec<Method> {
        vec![Method::GET, Method::HEAD]
    }

    fn get_features(self: &Self) -> Vec<Rc<dyn IControllerActionFeature>> {
        vec![]
    }

    fn get_should_validate_model(self: &Self) -> bool {
        false
    }
}