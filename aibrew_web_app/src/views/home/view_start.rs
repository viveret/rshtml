mvc_macro_lib::rusthtml_view_macro! {
    @name "home_view_start"
    // this code is executed before every view in this folder
    // unless the view uses @viewstart null or @viewstart ""
    view_context.insert_str("Layout", "shared/_layout.rs".to_string());
}