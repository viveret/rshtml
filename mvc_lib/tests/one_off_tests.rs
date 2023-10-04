use mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser;


#[test]
pub fn test_html_tag_attributes_bug() {
    /*
    could not compile rust html: RustHtmlError("convert_html_punct_to_rusthtmltoken Unexpected '=' before Literal { kind: Str, symbol: \"/\", suffix: None, span: #0 bytes(64250..64253) } (key was None)")
   --> example_web_app/src/views/shared/_layout.rs:1:1
    |
1   | / mvc_macro_lib::rusthtml_view_macro! {
2   | |     @name "shared__layout"
3   | |     @{
4   | |         let untitled = "Untitled".to_string();
...   |
114 | |      }
115 | | }
        // these lines cause the view to break
        // <li><a class=@home_class href="/">Home</a></li>
        // <li><a class=@learn_class href=@learn_href>Learn</a></li>
        // <li><a class=@dev_class href=@dev_href>@"Dev Tools"</a></li>
    | |_^
    |
    = note: this error originates in the macro `mvc_macro_lib::rusthtml_view_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
     */

    /*
    Test output on failure:
    thread 'one_off_tests::test_html_tag_attributes_bug' panicked at 'convert_html_punct_to_rusthtmltoken Unexpected '=' before Literal { lit: "/" } (key was None)', mvc_lib/src/core/panic_or_return_error.rs:14:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/std/src/panicking.rs:593:5
   1: core::panicking::panic_fmt
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/core/src/panicking.rs:67:14
   2: core::panicking::panic_display
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/core/src/panicking.rs:150:5
   3: mvc_lib::core::panic_or_return_error::PanicOrReturnError::panic_or_return_error
             at ./src/core/panic_or_return_error.rs:14:13
   4: mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter::panic_or_return_error
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:41:16
   5: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:835:32
   6: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_parse_html_tag
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:711:24
   7: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:262:24
   8: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
   9: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  10: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  11: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:277:24
  12: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
  13: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  14: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  15: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:277:24
  16: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
  17: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  18: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  19: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:277:24
  20: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
  21: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  22: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  23: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:277:24
  24: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
  25: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  26: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  27: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:277:24
  28: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
  29: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  30: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  31: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_html_entry_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:277:24
  32: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_punct_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:176:17
  33: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::convert_tokentree_to_rusthtmltoken
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:150:20
  34: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:114:16
  35: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::loop_next_and_convert
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:97:16
  36: <mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter as mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter>::parse_tokenstream_to_rusthtmltokens
             at ./src/view/rusthtml/rust_to_rusthtml_converter.rs:85:9
  37: mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser::expand_tokenstream
             at ./src/view/rusthtml/rusthtml_parser.rs:51:40
  38: rusthtml_parser_tests::one_off_tests::test_html_tag_attributes_bug
             at ./tests/one_off_tests.rs:144:18
  39: rusthtml_parser_tests::one_off_tests::test_html_tag_attributes_bug::{{closure}}
             at ./tests/one_off_tests.rs:5:39
  40: core::ops::function::FnOnce::call_once
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/core/src/ops/function.rs:250:5
  41: core::ops::function::FnOnce::call_once
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test one_off_tests::test_html_tag_attributes_bug ... FAILED
     */

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

    let parser = RustHtmlParser::new(true, "test".to_string());
    let result = parser.expand_tokenstream(view_tokenstream).unwrap();

    let expected_result = quote::quote! {};

    // this fails
    assert_eq!(expected_result.to_string(), result.to_string());

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

    let parser = RustHtmlParser::new(true, "test".to_string());
    let result = parser.expand_tokenstream(input).unwrap();

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