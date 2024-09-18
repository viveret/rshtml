use mvc_lib::view::macro_impl::rusthtml_view_macro_impl;

// #[test]
// pub fn example_web_app_dev__view() {
//     let html = quote::quote! {
//     };

//     let result = rusthtml_view_macro_impl(html);
//     let result_len = result.into_iter().count();

//     assert_ne!(0, result_len)
// }


#[test]
pub fn example_web_app_dev_controller_details_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_controller_details"
        @model crate::view_models::dev::controllers::ControllerDetailsViewModel
        @{
            let route_path = model.name;
            let title = format!("Controller details of {}", route_path);
            view_context.insert_str("Title", title.clone());

            let actions = model.actions;
            let controller_features = model.features;
            let controller_attributes = model.attributes;
            let controller_properties = model.properties;
            let controller_methods = model.methods;
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("controllers"), Some("Dev"), None, None).as_str(), "< Back to controllers list", None)

        <h1>@title</h1>

        <h3>@format!("Controller Features ({}):", controller_features.len())</h3>
        <ol>
        @for f in controller_features {
            <li>
                @f.to_string()
            </li>
        }
        </ol>

        <h3>@format!("Controller Attributes ({}):", controller_attributes.len())</h3>
        <ol>
        @for f in controller_attributes {
            <li>
                @f.to_string()
            </li>
        }
        </ol>

        <h3>@format!("Controller Properties ({}):", controller_properties.len())</h3>
        <ol>
        @for f in controller_properties {
            <li>
                <b>@f.0</b>@":"<span>@f.1</span>
            </li>
        }
        </ol>

        <h3>@format!("Controller Methods ({}):", controller_methods.len())</h3>
        <ol>
        @for f in controller_methods {
            <li>
                <small>@f.0</small>
                <b>@f.1</b>@"("<span>@f.2</span>@") -> "<span>@f.3</span>
            </li>
        }
        </ol>

        <h3>@format!("Actions ({}):", actions.len())</h3>
        <ol>
        @for route in actions {
            let link_text = route.0;
            let link_href = url.url_action(false, Some(false), None, Some("route_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(&route.1)));
            <li>
                @html.link(&link_href, &link_text, None)
            </li>
        }
        </ol>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controllers_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_controllers"
        @model crate::view_models::dev::controllers::ControllersViewModel
        @{
            view_context.insert_str("Title", "Controllers - Dev".to_string());
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)

        <h1>@view_context.get_str("Title")</h1>
        
        <p>@&format!("In total there are {} controllers:", model.controllers.len())</p>
        <ul>
        @for controller in model.controllers.iter() {
            let link_text = &controller.name;
            let link_href = url.url_action(false, Some(false), None, Some("controller_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(&controller.name)));
            <li>
                @html.link(&link_href, &link_text, None)
            </li>
        }
        </ul>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_index_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_index"
        @model crate::view_models::dev::index::IndexViewModel
        @{
            view_context.insert_str("Title", "Dev Routes".to_string());
        }
        
        <h1>@view_context.get_str("Title")</h1>
        <ul>
            <li>@html.link(url.url_action(false, Some(false), None, Some("log"), Some("Dev"), None, None).as_str(), "Log", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("perf_log"), Some("Dev"), None, None).as_str(), "Performance Log", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("controllers"), Some("Dev"), None, None).as_str(), "Controllers", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("routes"), Some("Dev"), None, None).as_str(), "Routes", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("views"), Some("Dev"), None, None).as_str(), "Compiled views", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("sys_info"), Some("Dev"), None, None).as_str(), "Sys Info", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("index"), Some("AuthRoles"), None, None).as_str(), "Auth Roles", None)</li>
            <li>@html.link(url.url_action(false, Some(false), None, Some("error"), Some("Dev"), None, None).as_str(), "Return Error", None)</li>
        </ul>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_log_add_view() {
    let html = quote::quote! {
        @use mvc_lib::view::rusthtml::helpers::stacks_html_helpers::StacksHtmlHelpers
        @use crate::view_models::dev::log_add::LogAddViewModel
        @viewstart "dev/_view_start.rs"
        @name "dev_log_add"
        @model LogAddViewModel
        @inject custom_html: StacksHtmlHelpers::<LogAddViewModel>
        @{
            view_context.insert_str("Title", "Add to Log - Dev".to_string());
        }

        @html.link(url.url_action(false, Some(false), None, Some("log"), Some("Dev"), None, None).as_str(), "< Back to log messages", None)

        <h1>@view_context.get_str("Title")</h1>

        @html.form(http::method::Method::POST, url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).into(), Some(&HashMap::new()) /* optional attributes, takes precedent over route values */, || -> HtmlString {
            <p class="fc-error">@html.validation_summary()</p>

            @custom_html.label_for(expr_quote! { |m| m.input.message }, None)
            @custom_html.input_for(expr_quote! { |m| m.input.message }, "text", None)
            
            @custom_html.label_for(expr_quote! { |m| m.input.level }, None)
            @custom_html.input_for(expr_quote! { |m| m.input.level }, "text", None)
            
            @custom_html.submit("Submit", None)
        })
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_log_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_log"
        @model crate::view_models::dev::log::LogViewModel
        @{
            view_context.insert_str("Title", "Log - Dev".to_string());
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
        
        <h1>@view_context.get_str("Title")</h1>
        @if model.supports_read {
            <p>@format!("There are {} log entries", model.logs.len())</p>
            @html.link(url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).as_str(), "Add log message", None)

            //<ul>
                // @for log in model.logs.iter() {
                //     <li>@log</li>
                // }
            //</ul>
        } else {
            <p>@"Reading from log is not supported."</p>
        }
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_perf_log_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_perf_log"
        @model crate::view_models::dev::perf_log::PerfLogViewModel
        @{
            view_context.insert_str("Title", "Performance Log - Dev".to_string());
        }
    
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes", None)
    
        <h1>@view_context.get_str("Title")</h1>
    
        <b>@"todo: Not implemented yet"</b>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_route_details_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_route_details"
        @model crate::view_models::dev::route_details::RouteDetailsViewModel
        @{
            let route_path = model.path;
            let title = format!("Route details of {}", route_path);
            view_context.insert_str("Title", title.clone());

            let action_features = model.features;
            let controller_features = model.controller_features;
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("routes"), Some("Dev"), None, None).as_str(), "< Back to routes list", None)

        <h1>@title</h1>

        <h3>@format!("Action Features ({}):", action_features.len())</h3>
        <ol>
        @for f in action_features {
            <li>
                @f
            </li>
        }
        </ol>

        <h3>@format!("Controller Features ({}):", controller_features.len())</h3>
        <ol>
        @for f in controller_features {
            <li>
                @f
            </li>
        }
        </ol>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_routes_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_routes"
        @model crate::view_models::dev::routes::RoutesViewModel
        @{
            view_context.insert_str("Title", "Routes - Dev".to_string());
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)

        <h1>@view_context.get_str("Title")</h1>
        
        <p>@format!("In total there are {} routes:", model.routes.len())</p>
        <ul>
        @for route in model.routes.iter() {
            let link_text = &route.as_string;
            let link_href = url.url_action(false, Some(false), None, Some("route_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(route.path.as_str())));
            <li>
                @html.link(&link_href, link_text.as_str(), None)
            </li>
        }
        </ul>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_sysinfo_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_sysinfo"
        @use sysinfo::SystemExt
        @use sysinfo::NetworkExt
        @use sysinfo::ProcessExt
        @model crate::view_models::dev::sys_info::SysInfoViewModel
        @{
            view_context.insert_str("Title", "Sys Info - Dev".to_string());

            let mut sys = sysinfo::System::new_all();
            sys.refresh_all();
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
        
        <h1>@view_context.get_str("Title")</h1>

        <h3>disks:</h3>
        <ul>
        @for disk in sys.disks() {
            <li>@format!("{:?}", disk)</li>
        }
        </ul>

        // Network interfaces name, data received and data transmitted:
        <h3>networks:</h3>
        <ul>
        @for (interface_name, data) in sys.networks() {
            <li>@format!("{}: {}/{} B", interface_name, data.received(), data.transmitted())</li>
        }
        </ul>

        // Components temperature:
        <h3>components:</h3>
        <ul>
        @for component in sys.components() {
            <li>@format!("{:?}", component)</li>
        }
        </ul>

        // RAM and swap information:
        <h3>system:</h3>
        <ul>
            <li>@format!("total memory: {} bytes", sys.total_memory())</li>
            <li>@format!("used memory : {} bytes", sys.used_memory())</li>
            <li>@format!("total swap  : {} bytes", sys.total_swap())</li>
            <li>@format!("used swap   : {} bytes", sys.used_swap())</li>

            // Display system information:
            <li>@format!("System name:             {:?}", sys.name())</li>
            <li>@format!("System kernel version:   {:?}", sys.kernel_version())</li>
            <li>@format!("System OS version:       {:?}", sys.os_version())</li>
            <li>@format!("System host name:        {:?}", sys.host_name())</li>

            // Number of CPUs:
            <li>@format!("NB CPUs: {}", sys.cpus().len())</li>
        </ul>

        // Display processes ID, name na disk usage:
        <h3>processes:</h3>
        <ul>
        @for (pid, process) in sys.processes() {
            <li>@format!("[{}] {} {:?}", pid, process.name(), process.disk_usage())</li>
        }
        </ul>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_view_details_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
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
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}



#[test]
pub fn example_web_app_dev_controller_views_view() {
    let html = quote::quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_views"
        @model crate::view_models::dev::views::ViewsViewModel
        @{
            view_context.insert_str("Title", "Compiled Views - Dev".to_string());
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
        
        <h1>@view_context.get_str("Title")</h1>
        
        <p>@format!("In total there are {} views:", model.views.len())</p>
        <ul>
        @for compiled_view in model.views.iter() {
            let href = url.url_action(false, Some(false), None, Some("view_details"), Some("Dev"), None, Some(&RouteValuesBuilder::build_area(compiled_view.path.as_str())));
            let model_type_name = match &compiled_view.model_type_name {
                Some(s) => format!("Requires model type {}", s),
                None => "No model type required".to_string(),
            };
            <li>
                <a href=@href>@compiled_view.path.as_str() <span>@" "</span> @model_type_name</a>
            </li>
        }
        </ul>
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}

