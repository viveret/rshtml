mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "learn/_view_start.rshtml"
    @model crate::view_models::learn::DetailsViewModel
    @name "learn_details"
    @{
        view_context.insert_str("Title", "Learn Rust HTML (rshtml)".to_string());
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Learn"), None, None).as_str(), "< Back to learning index", None)
    
    <h1>@&view_context.get_str("Title")</h1>

    // @htmlfile "home/index.html"
    // @rshtmlfile "home/index.rshtml"
    @mdfile_nocache model.path.clone()
}