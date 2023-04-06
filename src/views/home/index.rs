mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/home/_view_start.rshtml"
    @name "home_index"
    @{
        view_context.insert_str("Title", "Rust HTML (rshtml) Home".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>

    // @html "src/views/home/index.html"
    // @rshtml "src/views/home/index.rshtml"
    @mdfile_const "README.md"
}