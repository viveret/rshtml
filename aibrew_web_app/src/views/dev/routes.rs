mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "dev/_view_start.rshtml"
    @name "dev_routes"
    @model crate::view_models::dev::routes::RoutesViewModel
    @{
        view_context.insert_str("Title", "Routes - Dev".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)

    <h1>@view_context.get_str("Title")</h1>
    
    <p>@format!("In total there are {} routes:", model.routes.len())</p>
    <ul>
    @for route in model.routes.iter() {
        let link_text = &route.as_string;
        let link_href = url.url_action(false, Some(false), None, Some("route_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(route.path.as_str())));
        <li>
            @html.link(&link_href, link_text.as_str(), None)
        </li>
    }
    </ul>
}