
mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/authroles/_view_start.rshtml"
    @name "authroles_index"
    @model crate::view_models::authroles::IndexViewModel
    @{
        view_context.insert_str("Title", "Auth Roles - Dev".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>
    <p>@format!("There are {} roles", model.roles.len())</p>
    <ul>
        @for role in model.roles.iter() {
            <li>@role.name.clone()</li>
        }
        <li><a href="/auth-roles/add">@"Add New"</a></li>
    </ul>
}