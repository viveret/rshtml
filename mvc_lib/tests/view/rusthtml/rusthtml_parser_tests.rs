use proc_macro2::{TokenTree, Delimiter};
use quote::quote;

use mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser;




#[test]
fn rusthtml_parser_constructor_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    // assert_eq!(true, ctx.print_as_code(rust_output).is_ok());
}


#[test]
fn rusthtml_parser_print_as_code_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    let rust_output = quote! {
        fn test() {
            println!("test");
        }
    };
    parser.print_as_code(rust_output);
}


#[test]
fn rusthtml_parser_expect_punct_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    let rust_output = quote! {
        fn test() {
            println!("test");
        }
    };

    let mut it = rust_output.into_iter().peekable();
    parser.expect_punct(',', &mut it).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_empty_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    let rust_output = quote! {};
    parser.expand_tokenstream(rust_output).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_simple_function_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    let rust_output = quote! {
        fn test() {
            println!("test");
        }
    };

    parser.expand_tokenstream(rust_output).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_simple_struct_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    let rust_output = quote! {
        struct x {
        }
    };

    parser.expand_tokenstream(rust_output).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_simple_view_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    let rust_output = quote! {
        @name "dev_index"
        <ul>
            <li>test</li>
        </ul>
    };

    parser.expand_tokenstream(rust_output).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_complex_view_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    // more complex html
    let rust_output = quote! {
        @name "dev_index"
        <div>
            <ul>
                <li>test</li>
            </ul>
            <p>test</p>
            <br/>
            <input type="text" />
            <table>
                <tr>
                    <td>test</td>
                </tr>
            </table>
            <hr>
            <img src="test.png" />
            <a href="test.html">test</a>
            <form action="test.html" method="post">
                <input type="text" />
                <input type="submit" />
            </form>
        </div>
    };

    parser.expand_tokenstream(rust_output).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_for_loop_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    // more complex html
    let rust_output = quote! {
        @name "dev_index"
        @for x in 0..10 {
            <div>@x</div>
        }
    };

    parser.expand_tokenstream(rust_output).unwrap();
}


#[test]
fn rusthtml_parser_expand_tokenstream_for_loop_complex_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    // more complex html
    let rust_output = quote! {
        @name "dev_index"
        <h1>@view_context.get_str("Title")</h1>
        @if model.supports_read {
            <p>@format!("There are {} log entries", model.logs.len())</p>
            @html.link(url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).as_str(), "Add log message", None)
    
            <ul>
                @{
                    for log in model.logs.iter() {
                        <li>@log</li>
                    }
                }
            </ul>
        } else {
            <p>@"Reading from log is not supported."</p>
        }
    };
    let rust_output_expected = quote! {
        pub struct view_dev_index {

        }

        impl view_dev_index {
            pub fn new() -> view_dev_index {
                view_dev_index {

                }
            }
        }

        impl IView for view_dev_index {
            fn name(&self) -> &str {
                "dev_index"
            }

            fn render(&self, view_context: &mut ViewContext, model: &mut IModel, url: &mut UrlHelper, html: &mut HtmlHelper) -> Result<String, String> {
                view_context.insert_str("Title", "Log - Dev".to_string());
                let model = model.downcast_mut::<crate::view_models::dev::log::LogViewModel>().unwrap();
                let url = url.downcast_mut::<crate::url_helper::UrlHelper>().unwrap();
                let html = html.downcast_mut::<crate::html_helper::HtmlHelper>().unwrap();
                let mut output = String::new();
                output.push_str("<h1>");
                output.push_str(&view_context.get_str("Title"));
                output.push_str("</h1>");
                if model.supports_read {
                    output.push_str("<p>");
                    output.push_str(&format!("There are {} log entries", model.logs.len()));
                    output.push_str("</p>");
                    output.push_str(&html.link(url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).as_str(), "Add log message", None).unwrap());
                    output.push_str("<ul>");
                    for log in model.logs.iter() {
                        output.push_str("<li>");
                        output.push_str(&log.to_string());
                        output.push_str("</li>");
                    }
                    output.push_str("</ul>");
                } else {
                    output.push_str("<p>");
                    output.push_str("Reading from log is not supported.");
                    output.push_str("</p>");
                }
                Ok(output)
            }
        }
    };
    let mut expected_it = rust_output_expected.into_iter().peekable();

    let actual = parser.expand_tokenstream(rust_output.clone()).unwrap();
    let mut actual_it = actual.into_iter().peekable();

    // do simple string comparison
    let expected_str = expected_it.map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    let actual_str = actual_it.map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

    assert_str::assert_str_trim_eq!(expected_str, actual_str);

    // let mut group_stack_expected: Vec<TokenTree> = vec![];
    // let mut group_stack_actual: Vec<TokenTree> = vec![];

    // assert_tokenstreams_eq(expected_it, actual_it);
}

fn assert_tokenstreams_eq(mut expected_it: std::iter::Peekable<proc_macro2::token_stream::IntoIter>, mut actual_it: std::iter::Peekable<proc_macro2::token_stream::IntoIter>) {
    let mut index_at: usize = 0;
    loop {
        let expected_token = expected_it.next();
        let actual_token = actual_it.next();

        match (&expected_token, &actual_token) {
            (Some(expected_token), Some(actual_token)) => {
                match (expected_token, actual_token) {
                    (TokenTree::Punct(expected_punct), TokenTree::Punct(actual_punct)) => {
                        assert_eq!(expected_punct.as_char(), actual_punct.as_char());
                    },
                    (TokenTree::Ident(expected_ident), TokenTree::Ident(actual_ident)) => {
                        assert_eq!(expected_ident.to_string(), actual_ident.to_string());
                    },
                    (TokenTree::Literal(expected_literal), TokenTree::Literal(actual_literal)) => {
                        assert_eq!(expected_literal.to_string(), actual_literal.to_string());
                    },
                    (TokenTree::Group(expected_group), TokenTree::Group(actual_group)) => {
                        assert_eq!(expected_group.delimiter(), actual_group.delimiter());
                        assert_eq!(expected_group.stream().to_string(), actual_group.stream().to_string());
                    },
                    _ => {
                        if let TokenTree::Group(expected_group) = expected_token {
                            if expected_group.delimiter() == Delimiter::None {
                                assert_tokenstreams_eq(expected_group.stream().into_iter().peekable(), actual_it);
                            }
                        }
                        if let TokenTree::Group(actual_group) = actual_token {
                            if actual_group.delimiter() == Delimiter::None {
                                assert_tokenstreams_eq(expected_it, actual_group.stream().into_iter().peekable());
                            }
                        }
                        panic!("Expected and actual token streams are not equal at {} (expected: {:?}, actual: {:?})", index_at, expected_token, actual_token);
                    }
                }
            },
            (None, None) => {
                break;
            },
            _ => {
                let which_is_longer = if expected_token.is_some() { "expected" } else { "actual" };
                panic!("Expected and actual token streams differ in length at {} ({} is longer)", index_at, which_is_longer);
            }
        }
        index_at += 1;
    }
}
