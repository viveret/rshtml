mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "dev/_view_start.rshtml"
    @name "dev_controllers"
    @model crate::view_models::dev::controllers::ControllersViewModel
    @{
        view_context.insert_str("Title", "Controllers - Dev".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)

    <h1>@view_context.get_str("Title")</h1>
    
    <p>@&format!("In total there are {} controllers:", model.controllers.len())</p>
    <ul>
    @for controller in model.controllers.iter() {
        let link_text = &controller.name;
        let link_href = url.url_action(false, Some(false), None, Some("controller_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(&controller.name)));
        <li>
            @html.link(&link_href, &link_text, None)
        </li>
    }
    </ul>
}