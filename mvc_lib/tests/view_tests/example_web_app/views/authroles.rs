use mvc_lib::view::macro_impl::rusthtml_view_macro_impl;

#[test]
pub fn example_web_app_authroles_add_view() {
    let html = quote::quote! {
        @use mvc_lib::view::rusthtml::helpers::stacks_html_helpers::StacksHtmlHelpers
        @use crate::view_models::authroles::add::AddViewModel
        @viewstart "authroles/_view_start.rs"
        @name "authroles_add"
        @model AddViewModel
        @inject custom_html: StacksHtmlHelpers::<AddViewModel>
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
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_authroles_index_view() {
    let html = quote::quote! {
        @viewstart "authroles/_view_start.rs"
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
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();
    
    assert_ne!(0, result_len)
}