mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "dev/_view_start.rshtml"
    @name "dev_route_details"
    @model crate::view_models::dev::route_details::RouteDetailsViewModel
    @{
        let route_path = model.path;
        let title = format!("Route details of {}", route_path);
        view_context.insert_str("Title", title.clone());

        let action_features = model.features;
        let controller_features = model.controller_features;
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("routes"), Some("Dev"), None, None).as_str(), "< Back to routes list", None)

    <h1>@title</h1>

    <h3>@format!("Action Features ({}):", action_features.len())</h3>
    <ol>
    @for f in action_features {
        <li>
            @f
        </li>
    }
    </ol>

    <h3>@format!("Controller Features ({}):", controller_features.len())</h3>
    <ol>
    @for f in controller_features {
        <li>
            @f
        </li>
    }
    </ol>
}