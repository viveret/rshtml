rusthtml_macro::rusthtml_view_macro! {
    @name "dev_index"
    @model String // same as @arguments (<model type> model)
    @let x = 3;
    // can also do @arguments (<arguments>)
    @{
        // Layout = "_Layout_Dev_Index";
        view_data.insert("Title", "Compiled Views - Dev");
        
        pub fn custom_html() -> HtmlString {
            HtmlString::new_from_html("<b>raw html</b>")
        }

        // this is also allowed
        let y = 0;
    }
    <ul>
    @{
        for compiled_view in model.compiled_views {
            <li></li>
        }
    }
    </ul>
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