mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_log_add"
    @model crate::view_models::dev::LogAddViewModel
    @{
        view_context.insert_str("Title", "Add to Log - Dev".to_string());
    }

    @html.link(url.url_action(false, Some(false), None, Some("log"), Some("Dev"), None, None).as_str(), "< Back to log messages", None)

    <h1>@view_context.get_str("Title")</h1>

    <b>@"todo: Not implemented yet"</b>
}
