use std::collections::HashMap;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::IResponseContext;

use crate::action_results::iaction_result::IActionResult;

use crate::services::routemap_service::IRouteMapService;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

// this is a struct that holds a redirect target url.
#[derive(Debug)]
pub struct HttpRedirectResult {
    pub redirect_target: String,
}

impl HttpRedirectResult {
    pub fn new(redirect_target: String) -> Self {
        Self { redirect_target: redirect_target }
    }

    // this function configures the response to redirect to the redirect target.
    pub fn config_response(response_context: &dyn IResponseContext, redirect_target: String) {
        response_context.add_header_string("Location".to_string(), redirect_target);
    }
}

impl IActionResult for HttpRedirectResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::TEMPORARY_REDIRECT
    }

    fn configure_response(self: &Self, response_context: &dyn IResponseContext, _request_context: &dyn IRequestContext, _services: &dyn IServiceCollection) {
        Self::config_response(response_context, self.redirect_target.clone());
    }
}

// this is a struct that holds a redirect to action result.
#[derive(Debug)]
pub struct RedirectToActionResult {
    pub action_name: String,
    pub controller_name: String,
    pub area_name: String,
    pub route_values: Option<HashMap<String, String>>,
}

impl RedirectToActionResult {
    pub fn new(action_name: String, controller_name: String, area_name: String, route_values: Option<&HashMap<String, String>>) -> Self {
        Self { action_name: action_name, controller_name: controller_name, area_name: area_name, route_values: route_values.cloned() }
    }
}

impl IActionResult for RedirectToActionResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::TEMPORARY_REDIRECT
    }

    fn configure_response(self: &Self, response_context: &dyn IResponseContext, _request_context: &dyn IRequestContext, services: &dyn IServiceCollection) {
        // get the route map service and get the action from the route map.
        let route_map_service = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
        let action = route_map_service.get_mapper().get_action(self.action_name.as_str(), self.controller_name.as_str(), self.area_name.as_str());
        // generate the redirect url from the route values.
        let redirect_url = action.as_ref().get_route_pattern().gen_url(self.route_values.as_ref().unwrap());
        // configure the response to redirect to the redirect url.
        response_context.add_header_string("Location".to_string(), redirect_url);
    }
}

#[derive(Clone, Debug)]
pub struct InternalServerErrorResult {
    pub error: String,
}

impl InternalServerErrorResult {
    pub fn new(error: String) -> Self {
        Self { error: error }
    }

    pub fn default() -> Self {
        Self { error: "Internal Server Error".to_string() }
    }
}

impl IActionResult for InternalServerErrorResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn configure_response(self: &Self, response_context: &dyn IResponseContext, _request_context: &dyn IRequestContext, _services: &dyn IServiceCollection) {
        response_context.get_connection_context().write_str(format!("Error: {}", self.error).as_str()).unwrap();
    }
}