use std::collections::HashMap;

use crate::contexts::view_context::IViewContext;
use crate::services::routemap_service::IRouteMapService;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

use super::iurl_helpers::IUrlHelpers;
use super::path_builder::ActionPathBuilder;


// this struct helps with url generation and implements IUrlHelpers.
pub struct UrlHelpers<'a> {
    // pub current_route_data: &'a super::route_data::RouteData,
    // action_context: &'a IActionContext,
    // controller_context: &'a dyn IControllerContext,
    // request_context: &'a dyn IRequestContext,
    view_context: Option<&'a dyn IViewContext>,
    services: &'a dyn IServiceCollection,
}

impl <'a> UrlHelpers<'a> {
    pub fn new(view_context: &'a dyn IViewContext, services: &'a dyn IServiceCollection) -> Self {
        Self {
            view_context: Some(view_context),
            services: services,
        }
    }
}

impl <'a> IUrlHelpers for UrlHelpers<'a> {
    fn url_action(self: &Self,
        is_relative: bool,
        is_https: Option<bool>,
        protocol: Option<&str>,
        action_name: Option<&str>,
        controller_name: Option<&str>,
        area_name: Option<&str>,
        route_values: Option<&HashMap<String, String>>
    ) -> String {
        let mut path_builder = ActionPathBuilder::new();
        path_builder
            .add_optional(area_name)
            .add_optional(controller_name)
            .add_optional(action_name);

        let mapper = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(self.services);
        let action = mapper.as_ref().get_mapper().get_action_at_area_controller_action_path(path_builder.path);

        let mut url = String::new();
        if is_relative {
            url.push_str("/");
        } else {
            if let Some(is_https) = is_https {
                if is_https {
                    url.push_str("https://");
                } else {
                    url.push_str("http://");
                }
            } else {
                url.push_str("http://");
            }
            if let Some(protocol) = protocol {
                if let Some(is_https) = is_https {
                    if is_https {
                        panic!("The protocol cannot be specified when the url is https.");
                    }
                }

                url.push_str(protocol);
                url.push_str("://");
            }
        }

        url.push_str(&action.get_route_pattern().gen_url(route_values.unwrap_or(&HashMap::new())).as_str());

        url
    }
}