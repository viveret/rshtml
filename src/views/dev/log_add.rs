mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_log_add"
    @model crate::view_models::dev::log_add::LogAddViewModel
    @{
        view_context.insert_str("Title", "Add to Log - Dev".to_string());
    }

    @html.link(url.url_action(false, Some(false), None, Some("log"), Some("Dev"), None, None).as_str(), "< Back to log messages", None)

    <h1>@view_context.get_str("Title")</h1>

    @html.form(http::method::Method::POST, url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).into(), Some(&HashMap::new()) /* optional attributes, takes precedent over route values */, || -> HtmlString {
        @html.label_for(expr_quote! { |m| m.input.message }, None)
        @html.input("message", "text", model.input.message.as_str(), None)

        @html.label_for(expr_quote! { |m| m.input.level }, None)
        @html.input("level", "level", model.input.level.as_str(), None)
        
        @html.submit("Submit", None)
    })
}
