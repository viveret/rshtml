mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_route_details"
    @model crate::view_models::dev::RouteDetailsViewModel
    @{
        let route_path = model.route.get_path();
        let title = format!("Route details of {}", route_path);
        view_context.insert_str("Title", title.clone());

        let action_features = model.route.get_features();
        let controller_features = model.controller.get_features();
    }
    
    <h1>@title</h1>

    <h3>@format!("Features ({}):", action_features.len())</h3>
    <ol>
    @for f in action_features {
        <li>
            @f.to_string()
        </li>
    }
    </ol>

    <h3>@format!("Controller Features ({}):", controller_features.len())</h3>
    <ol>
    @for f in controller_features {
        <li>
            @f.to_string()
        </li>
    }
    </ol>
}