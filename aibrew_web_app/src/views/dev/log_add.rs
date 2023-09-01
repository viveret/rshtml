mvc_macro_lib::rusthtml_view_macro! {
    @use mvc_lib::view::rusthtml::helpers::stacks_html_helpers::StacksHtmlHelpers
    @use crate::view_models::dev::log_add::LogAddViewModel
    @viewstart "dev/_view_start.rshtml"
    @name "dev_log_add"
    @model LogAddViewModel
    @inject StacksHtmlHelpers::<LogAddViewModel>: custom_html
    @{
        view_context.insert_str("Title", "Add to Log - Dev".to_string());
    }

    @html.link(url.url_action(false, Some(false), None, Some("log"), Some("Dev"), None, None).as_str(), "< Back to log messages", None)

    <h1>@view_context.get_str("Title")</h1>

    @html.form(http::method::Method::POST, url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).into(), Some(&HashMap::new()) /* optional attributes, takes precedent over route values */, || -> HtmlString {
        <p class="fc-error">@html.validation_summary()</p>

        @custom_html.label_for(expr_quote! { |m| m.input.message }, None)
        @custom_html.input_for(expr_quote! { |m| m.input.message }, "text", None)
        
        @custom_html.label_for(expr_quote! { |m| m.input.level }, None)
        @custom_html.input_for(expr_quote! { |m| m.input.level }, "text", None)
        
        @custom_html.submit("Submit", None)
    })
}
