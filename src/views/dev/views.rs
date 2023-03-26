mvc_macro_lib::rusthtml_view_macro! {
    @name "dev_views"
    @model mvc_lib::view_models::dev::ViewsViewModel
    @{
        // Layout = "_Layout_Dev_Index";
        ViewData.insert("Title", "Compiled Views - Dev");
    }
    <ul>
    @{
        for compiled_view in model.views.iter() {
            <li></li>
        }
    }
    </ul>
}