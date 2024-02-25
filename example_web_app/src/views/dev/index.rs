mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "dev/_view_start.rs"
    @name "dev_index"
    @model crate::view_models::dev::index::IndexViewModel
    @{
        view_context.insert_str("Title", "Dev Routes".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>
    <ul>
        <li>@html.link(url.url_action(false, Some(false), None, Some("log"), Some("Dev"), None, None).as_str(), "Log", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("perf_log"), Some("Dev"), None, None).as_str(), "Performance Log", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("controllers"), Some("Dev"), None, None).as_str(), "Controllers", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("routes"), Some("Dev"), None, None).as_str(), "Routes", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("views"), Some("Dev"), None, None).as_str(), "Compiled views", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("sys_info"), Some("Dev"), None, None).as_str(), "Sys Info", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("index"), Some("AuthRoles"), None, None).as_str(), "Auth Roles", None)</li>
        <li>@html.link(url.url_action(false, Some(false), None, Some("error"), Some("Dev"), None, None).as_str(), "Return Error", None)</li>
    </ul>
}