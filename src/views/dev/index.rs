mvc_macro_lib::rusthtml_view_macro! {
    @name "dev_index"
    @model mvc_lib::view_models::dev::IndexViewModel
    @let x = 3;
    // can also do @arguments (<arguments>)
    @{
        // Layout = "_Layout_Dev_Index";
        ViewData.insert("Title", "Compiled Views - Dev");
        
        // pub fn custom_html() -> HtmlString {
        //     HtmlString::new_from_html_str("<b>raw html</b>")
        // }

        // this is also allowed
        let y = 0;
    }
    
    <h1>@ViewData.get("Title")</h1>
    <p>Dev index page</p>
}

// should generate:
/*

pub fn dev_index(model: String, view_context: &dyn IViewContext) -> HtmlString {
    let output = Vec::new();
    // functions pulled to top {}
    
    let x = 3;
    // Layout = "_Layout_Dev_Index";
    view_data.insert("Title", "Compiled Views - Dev");
    // this is also allowed
    let y = 0;

    output.push(HtmlString::new_from_html("<ul>"));
    for compiled_view in model.compiled_views {
        output.push(HtmlString::new_from_html("<li></li>"));
    }
    output.push(HtmlString::new_from_html("</ul>"));
}


*/