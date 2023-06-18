mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_view_details"
    @model crate::view_models::dev::view_details::ViewDetailsViewModel
    @{
        // Layout = "_Layout_Dev_Index";
        let title = format!("Compiled Rust HTML View at {}", model.path);
        view_context.insert_str("Title", title.clone());
        let raw = model.raw;
        let statements = raw.split([';', '{', '}',]);

        let model_type_name = match model.model_type_name {
            Some(s) => format!("Requires model type {}", s),
            None => "No model type required".to_string(),
        };
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("views"), Some("Dev"), None, None).as_str(), "< Back to views list", None)
    <h1>@title</h1>
    <h3>@model_type_name</h3>
    <ol>
    @for s in statements {
        <li>
            @s
        </li>
    }
    </ol>
}