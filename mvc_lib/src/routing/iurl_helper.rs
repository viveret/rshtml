use std::collections::HashMap;




pub trait IUrlHelper {
    // get an url by the action identifiers and route values within the current context.
    // is_relative: whether the url is relative (true) or absolute (false).
    // is_https: whether the url is https (true) or http (false). This requires is_relative to be false.
    // protocol: the protocol of the url. This requires is_relative to be false.
    // action_name: the name of the action. If None, the current or default action is used.
    // controller_name: the name of the controller. If None, the current or default controller is used.
    // area_name: the name of the area. If None, the current or default area is used.
    // route_values: the route values for the url. If None, the no route values are used.
    fn url_action(self: &Self,
        is_relative: bool,
        is_https: Option<bool>,
        protocol: Option<&str>,
        action_name: Option<&str>,
        controller_name: Option<&str>,
        area_name: Option<&str>,
        route_values: Option<&HashMap<String, String>>
    ) -> String;
}