mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_views"
    @model crate::view_models::dev::views::ViewsViewModel
    @{
        view_context.insert_str("Title", "Compiled Views - Dev".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
    
    <h1>@view_context.get_str("Title")</h1>
    
    <p>@format!("In total there are {} views:", model.views.len())</p>
    <ul>
    @for compiled_view in model.views.iter() {
        let href = url.url_action(false, Some(false), None, Some("view_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(compiled_view.path.as_str())));
        let model_type_name = match &compiled_view.model_type_name {
            Some(s) => format!("Requires model type {}", s),
            None => "No model type required".to_string(),
        };
        <li>
            <a href=@href>@compiled_view.path.as_str() <span>@" "</span> @model_type_name</a>
        </li>
    }
    </ul>
}