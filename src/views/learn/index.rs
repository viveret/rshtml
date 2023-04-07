mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/learn/_view_start.rshtml"
    @model crate::view_models::learn::IndexViewModel
    @name "learn_index"
    @{
        view_context.insert_str("Title", "Learn Rust HTML (rshtml)".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>

    <ul>
    @for doc_name in model.learn_docs.iter() {
        let href = format!("/learn/{}", doc_name);
        <li>
            <a href=@href>@doc_name</a>
        </li>
    }
    </ul>
    
    @mdfile_nocache "docs/learn/README.md"
}