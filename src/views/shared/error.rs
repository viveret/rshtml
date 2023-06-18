mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @use mvc_lib::error::error_viewmodel_service::BasicErrorViewModel
    @name "error"
    @model BasicErrorViewModel
    @{
        view_context.insert_str("Title", "Error".to_string());
    }
    <div>
    @format!("Error: {}", model.error)
    </div>
}