
mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/authroles/_view_start.rshtml"
    @name "authroles_add"
    @model crate::view_models::authroles::AddViewModel
    @inject crate::helpers::custom_html_helpers::CustomHtmlHelpers: custom_html
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

    @custom_html.form(http::method::Method::POST, "/dev/auth-roles/add".into(), Some(HashMap::new()) /* optional attributes, takes precedent over route values */, HashMap::new() /* optional route values */, || -> HtmlString {
        let role_name_label = "Role Name";
        @custom_html.label("role", role_name_label, None)
        @custom_html.input("role", "text", model.role.as_str(), None)
        
        @custom_html.submit("Submit", None)
    })
    
    @custom_html.link("/dev/auth-roles", "Back to auth roles list", None)
}