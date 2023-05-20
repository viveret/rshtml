use std::rc::Rc;

use http::StatusCode;

use crate::contexts::irequest_context::IRequestContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::controller_context::ControllerContext;

use crate::action_results::iaction_result::IActionResult;

use crate::controllers::controller_actions_map::IControllerActionsMap;
use crate::services::routemap_service::IRouteMapService;
use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

// this is a struct that holds a redirect target url.
pub struct HttpRedirectResult {
    pub redirect_target: String,
}

impl HttpRedirectResult {
    pub fn new(redirect_target: String) -> Self {
        Self { redirect_target: redirect_target }
    }

    // this function configures the response to redirect to the redirect target.
    pub fn config_response(response_ctx: Rc<ResponseContext>, redirect_target: String) {
        response_ctx.add_header_string("Location".to_string(), redirect_target);
    }
}

impl IActionResult for HttpRedirectResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::TEMPORARY_REDIRECT
    }

    fn configure_response(self: &Self, _controller_ctx: Rc<ControllerContext>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>, _services: &dyn IServiceCollection) {
        Self::config_response(response_ctx, self.redirect_target.clone());
    }
}

// this is a struct that holds a redirect to action result.
pub struct RedirectToActionResult {
    pub action_name: String,
    pub controller_name: String,
    pub area_name: String,
    pub route_values: Vec<(String, String)>,
}

impl RedirectToActionResult {
    pub fn new(action_name: String, controller_name: String, area_name: String, route_values: Option<Vec<(String, String)>>) -> Self {
        Self { action_name: action_name, controller_name: controller_name, area_name: area_name, route_values: route_values.or(Some(Vec::new())).unwrap() }
    }
}

impl IActionResult for RedirectToActionResult {
    fn get_statuscode(self: &Self) -> StatusCode {
        StatusCode::TEMPORARY_REDIRECT
    }

    fn configure_response(self: &Self, _controller_ctx: Rc<ControllerContext>, response_ctx: Rc<ResponseContext>, _request_ctx: Rc<dyn IRequestContext>, services: &dyn IServiceCollection) {
        // build the redirect action path from the area, controller and action name.
        let mut redirect_action_path = "/".to_string();
        if !self.area_name.is_empty() {
            redirect_action_path = format!("/{}/{}", self.area_name, redirect_action_path);
        }
        redirect_action_path = format!("{}/{}", redirect_action_path, self.controller_name);
        redirect_action_path = format!("{}/{}", redirect_action_path, self.action_name);

        // get the route map service and get the action at the area controller action path.
        let route_map_service = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(services);
        let action = route_map_service.get_mapper().get_action_at_area_controller_action_path(redirect_action_path);
        // generate the redirect url from the route values.
        let redirect_url = action.as_ref().gen_url(services, &self.route_values).unwrap();
        // configure the response to redirect to the redirect url.
        response_ctx.add_header_string("Location".to_string(), redirect_url);
    }
}