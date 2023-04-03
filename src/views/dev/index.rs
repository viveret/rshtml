mvc_macro_lib::rusthtml_view_macro! {
    @name "dev_index"
    @model mvc_lib::view_models::dev::IndexViewModel
    @{
        ViewData.insert("Title", "Compiled Views - Dev");
    }
    
    <h1>@ViewData.get("Title")</h1>
    <ul>
        <li><a href="/dev/views">@"Compiled views"</a></li>
        <li><a href="/dev/sysinfo">@"Sys Info"</a></li>
    </ul>
}