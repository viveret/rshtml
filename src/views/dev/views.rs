mvc_macro_lib::rusthtml_view_macro! {
    @name "dev_views"
    @model mvc_lib::view_models::dev::ViewsViewModel
    @{
        // Layout = "_Layout_Dev_Index";
        ViewData.insert("Title", "Compiled Views - Dev");
    }
    
    <h1>@ViewData.get("Title")</h1>
    <p>@format!("In total there are {} views:", model.views.len())</p>
    <ul>
    @for compiled_view in model.views.iter() {
        let href = format!("/dev/views/{}", compiled_view.get_path());
        <li>
            <a href=@href>@compiled_view.get_path() <span>@" requires "</span> @compiled_view.get_model_type_name()</a>
        </li>
    }
    </ul>
}