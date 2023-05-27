mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_index"
    @model crate::view_models::dev::IndexViewModel
    @{
        view_context.insert_str("Title", "Compiled Views - Dev".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>
    <ul>
    // TODO: use url.action_url() to build automatically
        <li><a href="/dev/views">@"Compiled views"</a></li>
        <li><a href="/dev/routes">@"Routes"</a></li>
        <li><a href="/dev/sysinfo">@"Sys Info"</a></li>
        <li><a href="/dev/auth-roles">@"Auth Roles"</a></li>
    </ul>
}