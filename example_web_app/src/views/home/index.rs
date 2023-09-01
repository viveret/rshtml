mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "home/_view_start.rshtml"
    @name "home_index"
    @{
        view_context.insert_str("Title", "Rust HTML (rshtml) Home".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>

    @mdfile_const "README.md"
}