mvc_macro_lib::rusthtml_view_macro! {
    @name "home_index"
    @{
        // Layout = "_Layout_Home_Index";
        ViewData.insert("Title", "Home");
    }
    @html "src/views/home/index.rshtml"

}