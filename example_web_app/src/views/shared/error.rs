mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "dev/_view_start.rshtml"
    @name "error"
    @model mvc_lib::error::error_viewmodel_service::BasicErrorViewModel
    @{
        view_context.insert_str("Title", "Error".to_string());
    }
    <h1>@view_context.get_str("Title")</h1>
    <h3>@format!("{}", model.error)</h3>
    <p>@format!("Source: {:?}", model.error.source())</p>
}