mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_controller_details"
    @model crate::view_models::dev::controllers::ControllerDetailsViewModel
    @{
        let route_path = model.name;
        let title = format!("Controller details of {}", route_path);
        view_context.insert_str("Title", title.clone());

        let actions = model.actions;
        let controller_features = model.features;
        let controller_attributes = model.attributes;
        let controller_properties = model.properties;
        let controller_methods = model.methods;
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("controllers"), Some("Dev"), None, None).as_str(), "< Back to controllers list", None)

    <h1>@title</h1>

    <h3>@format!("Controller Features ({}):", controller_features.len())</h3>
    <ol>
    @for f in controller_features {
        <li>
            @f.to_string()
        </li>
    }
    </ol>

    <h3>@format!("Controller Attributes ({}):", controller_attributes.len())</h3>
    <ol>
    @for f in controller_attributes {
        <li>
            @f.to_string()
        </li>
    }
    </ol>

    <h3>@format!("Controller Properties ({}):", controller_properties.len())</h3>
    <ol>
    @for f in controller_properties {
        <li>
            <b>@f.0</b>@":"<span>@f.1</span>
        </li>
    }
    </ol>

    <h3>@format!("Controller Methods ({}):", controller_methods.len())</h3>
    <ol>
    @for f in controller_methods {
        <li>
            <small>@f.0</small>
            <b>@f.1</b>@"("<span>@f.2</span>@") -> "<span>@f.3</span>
        </li>
    }
    </ol>

    <h3>@format!("Actions ({}):", actions.len())</h3>
    <ol>
    @for route in actions {
        let link_text = route.0;
        let link_href = url.url_action(false, Some(false), None, Some("route_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(&route.1)));
        <li>
            @html.link(&link_href, &link_text, None)
        </li>
    }
    </ol>
}