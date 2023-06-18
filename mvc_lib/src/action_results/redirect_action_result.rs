use std::collections::HashMap;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;
use crate::services::service_collection::IServiceCollection;

use super::iaction_result::IActionResult;


#[derive(Debug)]
pub struct RedirectActionResult {
    is_relative: bool,
    is_https: Option<bool>,
    protocol: Option<String>,
    action_name: Option<String>,
    controller_name: Option<String>,
    area_name: Option<String>,
    route_values: Option<HashMap<String, String>>,
}

impl RedirectActionResult {
    pub fn new(
        is_relative: bool,
        is_https: Option<bool>,
        protocol: Option<String>,
        action_name: Option<String>,
        controller_name: Option<String>,
        area_name: Option<String>,
        route_values: Option<&HashMap<String, String>>,
    ) -> Self {
        Self {
            is_relative: is_relative,
            is_https: is_https,
            protocol: protocol,
            action_name: action_name,
            controller_name: controller_name,
            area_name: area_name,
            route_values: route_values.cloned(),
        }
    }
}

impl IActionResult for RedirectActionResult {
    fn get_statuscode(self: &Self) -> http::StatusCode {
        todo!()
    }

    fn configure_response(self: &Self, _response_context: &dyn IResponseContext, request_context: &dyn IRequestContext, services: &dyn IServiceCollection) -> Result<(), std::rc::Rc<dyn std::error::Error>> {
        let _url = crate::routing::url_helpers::UrlHelpers::url_action_static(
            self.area_name.as_deref(),
            self.controller_name.as_deref(),
            self.action_name.as_deref(),
            self.is_relative,
            self.is_https,
            self.protocol.as_deref(),
            self.route_values.as_ref(),
            Some(request_context),
            services,
        );
        todo!()
    }
}