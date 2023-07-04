
mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "authroles/_view_start.rshtml"
    @name "authroles_index"
    @model crate::view_models::authroles::index::IndexViewModel
    @{
        view_context.insert_str("Title", "Auth Roles - Dev".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
    
    <h1>@view_context.get_str("Title")</h1>
    
    <p>@format!("There are {} roles:", model.roles.len())</p>
    <ol>
        @for role in model.roles.iter() {
            <li>@&role.name</li>
        }
    </ol>
    @html.link(url.url_action(false, Some(false), None, Some("add"), Some("AuthRoles"), None, None).as_str(), "+ Add New", None)
}