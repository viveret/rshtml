mvc_macro_lib::rusthtml_view_macro! {
    @name "home_index"
    @{
        // Layout = "_Layout_Home_Index";
        ViewData.insert("Title", "Rust HTML (rshtml) Home");
    }
    
    <h1>@ViewData.get("Title")</h1>

    // @html "src/views/home/index.html"
    // @rshtml "src/views/home/index.rshtml"
    @mdfile_const "README.md"
}