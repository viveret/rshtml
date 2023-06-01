mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_views"
    @model crate::view_models::dev::ViewsViewModel
    @{
        view_context.insert_str("Title", "Compiled Views - Dev".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
    
    <h1>@view_context.get_str("Title")</h1>
    
    <p>@format!("In total there are {} views:", model.views.len())</p>
    <ul>
    @for compiled_view in model.views.iter() {
        let href = url.url_action(false, Some(false), None, Some("view_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(compiled_view.get_path().to_string().as_str())));
        <li>
            <a href=@href>@compiled_view.get_path() <span>@" requires "</span> @compiled_view.get_model_type_name()</a>
        </li>
    }
    </ul>
}