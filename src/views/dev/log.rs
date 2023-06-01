mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_log"
    @model crate::view_models::dev::LogViewModel
    @{
        view_context.insert_str("Title", "Log - Dev".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
    
    <h1>@view_context.get_str("Title")</h1>

    <p>@format!("There are {} log entries", model.logs.len())</p>
    @html.link(url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).as_str(), "Add log message", None)

    <ul>
    @for log in model.logs.iter() {
        <li>@log</li>
    }
    </ul>
}
