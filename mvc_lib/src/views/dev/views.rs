rusthtml_macro::rusthtml_view_macro! {
    @name "dev_views"
    @model crate::view_models::dev::ViewsViewModel
    @{
        // Layout = "_Layout_Dev_Index";
        ViewData.insert("Title", "Compiled Views - Dev");
    }
    <ul>
    @{
        for compiled_view in Model.compiled_views {
            <li></li>
        }
    }
    </ul>
}