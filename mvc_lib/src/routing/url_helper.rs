use std::collections::HashMap;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::irequest_context::IRequestContext;
use crate::services::routemap_service::IRouteMapService;
use crate::services::service_collection::{IServiceCollection, ServiceCollectionExtensions};

use super::iurl_helper::IUrlHelper;


// this struct helps with url generation and implements IUrlHelper.
pub struct UrlHelper<'a> {
    // pub current_route_data: &'a super::route_data::RouteData,
    // action_context: &'a IActionContext,
    // controller_context: &'a dyn IControllerContext,
    // request_context: &'a dyn IRequestContext,
    services: &'a dyn IServiceCollection,
}

impl <'a> UrlHelper<'a> {
    pub fn new(services: &'a dyn IServiceCollection) -> Self {
        Self {
            services: services,
        }
    }
}

impl <'a> IUrlHelper for UrlHelper<'a> {
    fn url_action(self: &Self,
        is_relative: bool,
        is_https: Option<bool>,
        protocol: Option<&str>,
        action_name: Option<&str>,
        controller_name: Option<&str>,
        area_name: Option<&str>,
        route_values: Option<&HashMap<String, String>>
    ) -> String {
        let mut action_path = "/".to_string();
        if !area_name.unwrap_or_default().is_empty() {
            action_path = format!("/{}/{}", area_name.unwrap(), action_path);
        }

        if !controller_name.unwrap_or_default().is_empty() {
            action_path = format!("/{}/{}", controller_name.unwrap(), action_path);
        }

        if !action_name.unwrap_or_default().is_empty() {
            action_path = format!("/{}/{}", action_name.unwrap(), action_path);
        }

        let mapper = ServiceCollectionExtensions::get_required_single::<dyn IRouteMapService>(self.services);
        let action = mapper.as_ref().get_mapper().get_action_at_area_controller_action_path(action_path);

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