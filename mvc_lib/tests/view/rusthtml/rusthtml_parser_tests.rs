use proc_macro2::{TokenTree, Delimiter, TokenStream};
use quote::quote;

use mvc_lib::html::gen::HtmlGenerator;
use mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser;



#[test]
fn rusthtml_parser_constructor_works() {
    let _ = RustHtmlParser::new(false, "test".to_string());
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
        <div></div>
    };

    let mut it = rust_output.into_iter().peekable();
    match parser.expect_punct('<', &mut it) {
        Ok(_) => {},
        Err(e) => panic!("Expected punct, not {:?}", e)
    }
}


// #[test]
// fn rusthtml_parser_expect_ident_works() {
//     let parser = RustHtmlParser::new(false, "test".to_string());
//     let rust_output = quote! {
//         fn test() {
//             println!("test");
//         }
//     };

//     let mut it = rust_output.into_iter().peekable();
//     match parser.expect_ident("fn", &mut it) {
//         Ok(_) => {},
//         Err(e) => panic!("Expected ident, not {:?}", e)
//     }
// }


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
    let html = "
        <ul>
            <li>test</li>
        </ul>
    ";
    let html_tokenstream: TokenStream = html.parse().unwrap();
    let rust_output = quote! {
        #html_tokenstream
    };

    let actual_stream = parser.expand_tokenstream(rust_output).unwrap();
    assert_eq!("html_output . write_html_str (\"<ul><li>test</li></ul>\") ;", actual_stream.to_string());
}


#[test]
fn rusthtml_parser_expand_tokenstream_complex_view_works() {
    let parser = RustHtmlParser::new(false, "test".to_string());
    // more complex html
    let html = HtmlGenerator::new().generate();
    let html_tokenstream: TokenStream = html.parse().unwrap();
    let html_tokenstream_str = html_tokenstream.to_string();
    let actual_stream = parser.expand_tokenstream(html_tokenstream).unwrap();
    assert_eq_ignore_whitespace(quote::quote! { html_output . write_html_str (#html_tokenstream_str) ; }.to_string(), actual_stream.to_string());
}

fn assert_eq_ignore_whitespace(expected: String, actual: String) {
    let expected = expected.replace(" ", "").replace("\n", "");
    let actual = actual.replace(" ", "").replace("\n", "");
    assert_eq!(expected, actual);
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
        html_output . write_html_str("<h1>");
        html_output . write_html_str(&view_context.get_str("Title"));
        html_output . write_html_str("</h1>");
        if model.supports_read {
            html_output . write_html_str("<p>");
            html_output . write_html_str(&format!("There are {} log entries", model.logs.len()));
            html_output . write_html_str("</p>");
            html_output . write_html_str(&html.link(url.url_action(false, Some(false), None, Some("log_add"), Some("Dev"), None, None).as_str(), "Add log message", None).unwrap());
            html_output . write_html_str("<ul>");
            for log in model.logs.iter() {
                html_output . write_html_str("<li>");
                html_output . write_html_str(&log.to_string());
                html_output . write_html_str("</li>");
            }
            html_output . write_html_str("</ul>");
        } else {
            html_output . write_html_str("<p>");
            html_output . write_html_str("Reading from log is not supported.");
            html_output . write_html_str("</p>");
        }
    };
    let expected_it = rust_output_expected.into_iter().peekable();

    let actual = parser.expand_tokenstream(rust_output.clone()).unwrap();
    let actual_it = actual.into_iter().peekable();

    // do simple string comparison
    let expected_str = expected_it.map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    let actual_str = actual_it.map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

    assert_str::assert_str_trim_eq!(expected_str, actual_str);
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







#[test]
fn rusthtml_parser_expand_tokenstream_if_else_followed_by_html() {
    let stream = quote::quote! {
        @{
            let html_class = if validation_result . has_errors { "fc-error" } else { "fc-success" } ; <p class = @html_class> @validation_result . message </p>
        }
    };
    let expected_output = quote::quote! {
        let html_class = if validation_result . has_errors { "fc-error" } else { "fc-success" } ;
        html_output.write_html_str("<p class=\"");
        html_output.write_html_str(html_class);
        html_output.write_html_str(">");
        html_output.write_html_str(validation_result . message);
        html_output.write_html_str("</p>");
    };
    let expected_it = expected_output.into_iter().peekable();

    let parser = RustHtmlParser::new(false, "test".to_string());
    let actual_output = parser.expand_tokenstream(stream).unwrap();

    let actual = parser.expand_tokenstream(actual_output.clone()).unwrap();
    let actual_it = actual.into_iter().peekable();

    // do simple string comparison
    let expected_str = expected_it.map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    let actual_str = actual_it.map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

    assert_str::assert_str_trim_eq!(expected_str, actual_str);
}
