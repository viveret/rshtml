mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "dev/_view_start.rshtml"
    @name "dev_perf_log"
    @model crate::view_models::dev::perf_log::PerfLogViewModel
    @{
        view_context.insert_str("Title", "Performance Log - Dev".to_string());
    }

    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes", None)

    <h1>@view_context.get_str("Title")</h1>

    <b>@"todo: Not implemented yet"</b>
}
