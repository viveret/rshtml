mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/learn/_view_start.rshtml"
    @model crate::view_models::learn::DetailsViewModel
    @name "learn_details"
    @{
        view_context.insert_str("Title", "Learn Rust HTML (rshtml)".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>

    // @htmlfile "src/views/home/index.html"
    // @rshtmlfile "src/views/home/index.rshtml"
    @mdfile_nocache model.path.clone().as_str()
}