
mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/authroles/_view_start.rshtml"
    @name "authroles_add"
    @model crate::view_models::authroles::AddViewModel
    @{
        view_context.insert_str("Title", "Add Auth Role - Dev".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>
    
    @{
        if let Some(validation_result) = &model.validation_result {
            let html_class = if validation_result.as_ref().has_errors { "fc-error" } else { "fc-success" };
            <p class=@html_class>@validation_result.message.clone()</p>
        }
    }

    <form method="POST">
        <label for="role">@"Role Name"</label>
        <input name="role" type="text" value=@model.role.clone() />
        @html.form.submit("Submit")
    </form>

    
    <a href="/dev/auth-roles">@"Back to auth roles list"</a>
}