
mvc_macro_lib::rusthtml_view_macro! {
    @use mvc_lib::view::rusthtml::helpers::stacks_html_helpers::StacksHtmlHelpers
    @use crate::view_models::authroles::add::AddViewModel
    @viewstart "authroles/_view_start.rshtml"
    @name "authroles_add"
    @model AddViewModel
    @inject StacksHtmlHelpers::<AddViewModel>: custom_html
    @{
        view_context.insert_str("Title", "Add Auth Role - Dev".to_string());
    }
    
    @custom_html.link(url.url_action(false, Some(false), None, Some("index"), Some("AuthRoles"), None, None).as_str(), "< Back to auth roles list", None)
    
    <h1>@view_context.get_str("Title")</h1>
    
    @if let Some(validation_result) = &model.validation_result {
        let html_class = if validation_result.has_errors { "fc-error" } else { "fc-success" };
        <p class=@html_class>@validation_result.message.clone()</p>
    }

    @custom_html.form(http::method::Method::POST, url.url_action(false, Some(false), None, Some("add"), Some("AuthRoles"), None, None).into(), Some(&HashMap::new()) /* optional attributes, takes precedent over route values */, || -> HtmlString {
        let role_name_label = "Role Name";
        @custom_html.label("role", role_name_label, None)
        @custom_html.input("role", "text", model.role.as_str(), None)
        
        @custom_html.submit("Submit", None)
    })
}