mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "learn/_view_start.rs"
    @model crate::view_models::learn::IndexViewModel
    @name "learn_index"
    @{
        view_context.insert_str("Title", "Learn Rust HTML (rshtml)".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>

    <ul>
    @for doc_name in model.learn_docs.iter() {
        // doc_name as text and as href id
        let href = url.url_action(false, Some(false), None, Some("details"), Some("Learn"), None, Some(&RouteValuesBuilder::build_area(doc_name)));
        <li>
            @html.link(href.as_str(), doc_name, None)
        </li>
    }
    </ul>
    
    @mdfile_nocache "docs/learn/README.md"
}