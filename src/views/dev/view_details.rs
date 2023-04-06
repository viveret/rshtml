mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_view_details"
    @model mvc_lib::view_models::dev::ViewDetailsViewModel
    @{
        // Layout = "_Layout_Dev_Index";
        let title = format!("Compiled Rust HTML View at {}", model.view.get_path());
        view_context.insert_str("Title", title.clone());
        let raw = model.view.get_raw();
        let statements = raw.split([';', '{', '}',]);

        let model_type_name = match model.view.get_model_type_name() {
            Some(s) => format!("Requires model type {}", s),
            None => "No model type required".to_string(),
        };
    }
    
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