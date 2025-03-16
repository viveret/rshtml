use std::{collections::HashMap, rc::Rc};

use core_lib::{assert::assert_tokentree::assert_tokentree_stream, asyncly::cancellation_token::CancellationToken};
use quote::quote;

use mvc_lib::{contexts::view_context, view::{macro_impl::{rusthtml_macro_impl, rusthtml_view_macro_impl, rusthtml_view_macro_with_context}, parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext, rusthtml::html_string::HtmlString}};



#[test]
pub fn test_parse_bug() {
    let html = quote! {
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
        <ol>
        @{
            for f in controller_features {
            <li>
                @f.to_string()
            </li>
            }
        }
        </ol>
    };


    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}


#[test]
pub fn test_parse_empty() {
    let html = quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_controller_details"
        @model crate::view_models::dev::controllers::ControllerDetailsViewModel
    };


    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}


// #[test]
// pub fn test_rendered_code() {
//     let view_context = 0;
//     let html_output = 0;
//     let custom_html = 0;
//     let url = 0;
//     let model = 0;
//     view_context.insert_str("Title", "Add Auth Role - Dev".to_string());
//     html_output.write_html(HtmlString :: from(custom_html.link(url.url_action(false, Some(false), None, Some("index"), Some("AuthRoles"), None, None).as_str(), "< Back to auth roles list", None)));
//     html_output.write_html_str("<h1");
//     html_output.write_html_str(">");
//     html_output.write_html(HtmlString :: from(view_context.get_str("Title")));
//     html_output.write_html_str("</h1>");
//     if let Some(validation_result) = &model.validation_result
//     {
//         let html_class = if validation_result.has_errors { "fc-error" } else
//         { "fc-success" }; html_output.write_html_str("<p");
//         html_output.write_html_str(">");
//         html_output.write_html(HtmlString ::
//         from(validation_result.message.clone()));
//         html_output.write_html_str("</p>");
//     }
//     html_output.write_html(HtmlString ::
//     from(custom_html.form(http::method::Method::POST,
//     url.url_action(false, Some(false), None, Some("add"), Some("AuthRoles"), None,
//     None).into(), Some(&HashMap::new()), || -> HtmlString
//     {
//         let role_name_label = "Role Name";
//         // @custom_html.label("role", role_name_label, None)
//         // @custom_html.input("role", "text", model.role.as_str(), None)
//         // @custom_html.submit("Submit", None)
//         HtmlString::from("")
//     })));
// }


#[test]
pub fn test_parse_single_use() {
    let html = quote! {
        @viewstart "dev/_view_start.rs"
        @name "dev_controller_details"
        @model crate::view_models::dev::controllers::ControllerDetailsViewModel
        @use quote::quote
    };

    let rust_expected = quote! {
        use quote::quote;
    };

    let result = rusthtml_view_macro_with_context(html.clone());
    assert_tokentree_stream(rust_expected, result.0.get_use_statements_stream());
}




#[test]
pub fn test_html_tag_attributes_bug() {
     let view_tokenstream = quote::quote! {
        @name "shared__layout"
        @{
            let untitled = "Untitled".to_string();
            let mut page_title = view_context.get_str("Title");
            if page_title.len() == 0 {
                page_title = untitled;
            }

            let page_action = view_context.get_str("ActionName");
            let page_controller = view_context.get_str("ControllerName");
            let page_area = view_context.get_str("AreaName");

        }
        @functions {
            pub fn is_same_action(
                a_action: &'static str, a_controller: &'static str, a_area: &'static str,
                b_action: &String, b_controller: &String, b_area: &String) -> bool {
                    (a_action == b_action.as_str() || a_action == "*" || b_action == "*") && 
                    (a_controller == b_controller.as_str() || a_controller == "*" || b_controller == "*") && 
                    (a_area == b_area.as_str() || a_area == "*" || b_area == "*")
                }
            pub fn is_same_action_is_selected(
                a_action: &'static str, a_controller: &'static str, a_area: &'static str,
                b_action: &String, b_controller: &String, b_area: &String) -> &'static str {
                    if is_same_action(a_action, a_controller, a_area, b_action, b_controller, b_area) {
                        "is-selected"
                    } else {
                        ""
                    }
                }
        }
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>@format!("{} - WebApplication1", page_title)</title>

        <environment include="Development">
            <link rel="stylesheet" href="/stacks.css" />
            <link rel="stylesheet" href="/css/site.css" />
        </environment>
        <environment exclude="Development">
            <link rel="stylesheet" href="/stacks.min.css" />
            <link rel="stylesheet" href="/css/site.min.css" asp-append-version="true" />
        </environment>
    </head>

<body class="theme-system">
<header class="s-topbar stacks-topbar ps-fixed h64 js-stacks-topbar print:d-none">
    <div class="s-topbar--container px8">
        <a href="#" class="s-topbar--menu-btn d-none md:d-flex js-hamburger-btn"><span></span></a>
        <a class="s-topbar--logo" href="/">
            <span class="v-visible-sr">Site home</span>
            @htmlfile "shared/_icon_svg.html"
            <environment include="Development"><i>Beta</i></environment>
        </a>

        <ul class="s-navigation ml8 fw-nowrap sm:d-none">
            @let home_class = format!("s-navigation--item {}", is_same_action_is_selected("*", "Home", "", &page_action, &page_controller, &page_area));
            @let learn_class = format!("s-navigation--item {}", is_same_action_is_selected("*", "Learn", "", &page_action, &page_controller, &page_area));
            @let learn_href = url.url_action(false, Some(false), None, Some("index"), Some("Learn"), None, None);
            
            <li><a class=@home_class href="/">Home</a></li>
            <li><a class=@learn_class href=@learn_href>Learn</a></li>
            <li><a class="s-navigation--item" href="https://github.com/viveret/rshtml">GitHub</a></li>
            <environment include="Development">
                @let is_dev_controller = is_same_action("*", "Dev", "", &page_action, &page_controller, &page_area) || is_same_action("*", "AuthRoles", "", &page_action, &page_controller, &page_area);
                @let dev_class = format!("s-navigation--item {}", if is_dev_controller { "is-selected" } else { "" });
                @let dev_href = url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None);
                <li><a class=@dev_class href=@dev_href>@"Dev Tools"</a></li>
            </environment>
        </ul>

        <ol class="s-topbar--content sm:ml0 overflow-hidden"></ol>

        <div class="s-topbar--searchbar w100 wmx3 sm:wmx-initial js-search">
            <div class="s-topbar--searchbar--input-group">
                <span class="algolia-autocomplete" style="position: relative; display: inline-block; direction: ltr;"><input id="searchbox" type="text" placeholder="Search…" value="" autocomplete="off" class="s-input s-input__search ds-input" spellcheck="false" role="combobox" aria-autocomplete="list" aria-expanded="false" aria-label="search input" aria-owns="algolia-autocomplete-listbox-0" style="position: relative; vertical-align: top;" dir="auto"><pre aria-hidden="true" style="position: absolute; visibility: hidden; white-space: pre; font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI Adjusted&quot;, &quot;Segoe UI&quot;, &quot;Liberation Sans&quot;, sans-serif; font-size: 13px; font-style: normal; font-variant: normal; font-weight: 400; word-spacing: 0px; letter-spacing: normal; text-indent: 0px; text-rendering: optimizelegibility; text-transform: none;"></pre><span class="ds-dropdown-menu" style="position: absolute; top: 100%; z-index: 100; display: none; left: 0px; right: auto;" role="listbox" id="algolia-autocomplete-listbox-0"><div class="ds-dataset-1"></div></span></span>
                <svg aria-hidden="true" class="svg-icon iconSearch s-input-icon s-input-icon__search" width="18" height="18" viewBox="0 0 18 18"><path d="m18 16.5-5.14-5.18h-.35a7 7 0 1 0-1.19 1.19v.35L16.5 18l1.5-1.5ZM12 7A5 5 0 1 1 2 7a5 5 0 0 1 10 0Z"></path></svg>
            </div>
        </div>
    </div>
</header>

<div>
    <partial name="_CookieConsentPartial" />
</div>

<div class="container body-content ps-relative py24 t64 mx-auto w100 wmx12">
    @render.body()
    <footer class="pt32">
        <hr />

        @let current_year = chrono::prelude::Utc::now().format("%Y");
        <p>&copy; @format!("{} - Example Rust Html Web Application", current_year)</p>

        @let compile_timestamp = format!("Page compiled at {}", self.when_compiled.format("%Y-%m-%d   %H:%M:%S"));
        @let view_timestamp = format!("Page viewed at {}", chrono::prelude::Utc::now().format("%Y-%m-%d   %H:%M:%S"));
        <p>@format!("{} — {}", compile_timestamp, view_timestamp)</p>
        <p>@format!("Layout path: {}, action: {}, controller: {}, area: {}", self.ViewPath, page_action, page_controller, page_area)</p>
    </footer>
</div>

<environment include="Development">
    <script src="/js/site.js" asp-append-version="true"></script>
</environment>
<environment exclude="Development">
    <script src="/js/site.min.js" asp-append-version="true"></script>
</environment>

@render.section_optional("Scripts")
</body>
</html>
    };

/*    let ct = Rc::new(CancellationToken::new());
    let parser = RustHtmlParser::new(true, "test".to_string());
    let result = parser.expand_tokenstream(view_tokenstream, ct).unwrap();

    let expected_result = quote::quote! {};

    // this fails
    assert_eq!(expected_result.to_string(), result.to_string());
*/
    // summarize the error:
    // convert_html_punct_to_rusthtmltoken Unexpected '=' before Literal { lit: "/" } (key was None)
    // this is because the parser is expecting a key before the literal, but there is none.
    // the parser is expecting a key because the literal is a string literal, and the parser
    // is expecting a key=value pair. the parser expects a key=value but the directive before the key=value
    // is messing up the parser. 

    // summarize the fix:
    // the parser needs to be able to handle a key=value pair with or without a directive before and after it.
    // the directive inside an HTML tag key=value pair needs to be handled differently than a regular directive.
    // it also probably needs tests
}

#[test]
pub fn test_html_single_tag_single_attribute_bug() {
    let input = quote::quote! {
        <li><a class=@home_class href="/">Home</a></li>
    };

    let result = rusthtml_macro_impl(input);
    let expected_result = quote::quote! {
        html_output.write_html_str("<li><a class=");
        html_output.write_html_str(&home_class);
        html_output.write_html_str(" href=\"/\">Home</a></li>");
    };

    assert_eq!(expected_result.to_string(), result.to_string());
}


#[test]
pub fn test_html_tag_attributes_bug2() {
    /*
    could not compile rust html: RustHtmlError("convert_html_punct_to_rusthtmltoken Unexpected '=' before Literal { kind: Str, symbol: \"/\", suffix: None, span: #0 bytes(64250..64253) } (key was None)")
   --> example_web_app/src/views/shared/_layout.rs:1:1
    |
    */
    let input = quote::quote! {
        <li><a class=@home_class href="/">Home</a></li>
        <li><a class=@learn_class href=@learn_href>Learn</a></li>
        <li><a class=@dev_class href=@dev_href>@"Dev Tools"</a></li>
    };

    let result = rusthtml_macro_impl(input);
    let expected_result = quote::quote! {
        html.write_str("<li><a class=");
        html.write_str(&home_class);
        html.write_str(" href=\"/\">Home</a></li>");
        html.write_str("<li><a class=");
        html.write_str(&learn_class);
        html.write_str(" href=");
        html.write_str(&learn_href);
        html.write_str(">Learn</a></li>");
        html.write_str("<li><a class=");
        html.write_str(&dev_class);
        html.write_str(" href=");
        html.write_str(&dev_href);
        html.write_str(">\"Dev Tools\"</a></li>");
    };

    assert_eq!(expected_result.to_string(), result.to_string());
}

