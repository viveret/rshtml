mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_routes"
    @model crate::view_models::dev::RoutesViewModel
    @{
        view_context.insert_str("Title", "Routes - Dev".to_string());
    }
    
    <h1>@view_context.get_str("Title")</h1>
    <p>@format!("In total there are {} routes:", model.routes.len())</p>
    <ul>
    @for route in model.routes.iter() {
        let href = format!("/dev/routes/{}", route.get_path());
        <li>
            <a href=@href>@route.to_string()</a>
        </li>
    }
    </ul>
}