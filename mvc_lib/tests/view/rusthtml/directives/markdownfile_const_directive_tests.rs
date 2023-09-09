use std::rc::Rc;

use mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use mvc_lib::view::rusthtml::{directives::markdownfile_const_directive::MarkdownFileConstDirective, rusthtml_token::RustHtmlToken};
use mvc_lib::view::rusthtml::peekable_tokentree::PeekableTokenTree;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use proc_macro2::{TokenStream, Ident, TokenTree, Span};





#[test]
pub fn convert_mdfile_const_directive_test() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "Test".to_string()));
    let parser = RustToRustHtmlConverter::new(parser_context);
    let mut output = vec![];
    let it = PeekableTokenTree::new(quote::quote! { "test.md" });

    let identifier = Ident::new("test", Span::call_site());
    let ident_token = TokenTree::Ident(identifier.clone());

    let result = MarkdownFileConstDirective::convert_mdfile_const_directive(
        &identifier, &ident_token, Rc::new(parser), &mut output, Rc::new(it)
    ).unwrap();

    assert_eq!(output.len(), 1);
    match output.get(0).unwrap() {
        RustHtmlToken::AppendToHtml(tokens) => {
            assert_eq!(tokens.len(), 1);
            match tokens.get(0).unwrap() {
                RustHtmlToken::Group(d, g) => {
                    assert_eq!(*d, proc_macro2::Delimiter::Brace);

                    let full_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("test.md");
                    let full_path = full_path.to_str().unwrap();
                    let expected_stream = quote::quote! {
                        match view_context.open_data_file(#full_path) {
                            Ok(mut f) => {
                                let mut buffer = String::new();
                                f.read_to_string(&mut buffer).expect("could not read markdown file");
                                match comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()) {
                                    Some(n) => {
                                        if n == 0 {
                                            panic!("beep boop bop");
                                        } else {
                                            Ok(HtmlString::new_from_html(buffer))
                                        }
                                    },
                                    None => {
                                        panic!("beep beep beep");
                                    }
                                }
                            },
                            Err(e) => {
                                panic!("cannot read external markdown file const '{}', could not open: {:?}", #full_path, e);
                            }
                        }
                    };
                    g.stream().into_iter().zip(expected_stream.into_iter()).for_each(|(a, b)| {
                        assert_eq!(a.to_string(), b.to_string());
                    });
                },
                _ => {
                    panic!("expected RustHtmlToken::Group");
                }
            }
        },
        _ => {
            panic!("expected RustHtmlToken::AppendToHtml");
        }
    }
}


// need to test within parser as integration test
#[test]
pub fn convert_mdfile_const_directive_integration_test() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "Test".to_string()));
    let parser = RustToRustHtmlConverter::new(parser_context);
    let it = PeekableTokenTree::new(quote::quote! { @mdfile_const "test.md" });

    let output = parser.parse_tokenstream_to_rusthtmltokens(false, Rc::new(it), false).unwrap();
    assert_eq!(output.len(), 1);

    match output.get(0).unwrap() {
        RustHtmlToken::AppendToHtml(tokens) => {
            assert_eq!(tokens.len(), 1);
            match tokens.get(0).unwrap() {
                RustHtmlToken::Group(d, g) => {
                    assert_eq!(*d, proc_macro2::Delimiter::Brace);

                    let full_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("test.md");
                    let full_path = full_path.to_str().unwrap();
                    let expected_stream = quote::quote! {
                        match view_context.open_data_file(#full_path) {
                            Ok(mut f) => {
                                let mut buffer = String::new();
                                f.read_to_string(&mut buffer).expect("could not read markdown file");
                                match comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()) {
                                    Some(n) => {
                                        if n == 0 {
                                            panic!("beep boop bop");
                                        } else {
                                            Ok(HtmlString::new_from_html(buffer))
                                        }
                                    },
                                    None => {
                                        panic!("beep beep beep");
                                    }
                                }
                            },
                            Err(e) => {
                                panic!("cannot read external markdown file const '{}', could not open: {:?}", #full_path, e);
                            }
                        }
                    };
                    g.stream().into_iter().zip(expected_stream.into_iter()).for_each(|(a, b)| {
                        assert_eq!(a.to_string(), b.to_string());
                    });
                },
                _ => {
                    panic!("expected RustHtmlToken::Group, actual: {:?}", tokens.get(0).unwrap());
                }
            }
        },
        _ => {
            panic!("expected RustHtmlToken::AppendToHtml, actual: {:?}", output.get(0).unwrap());
        }
    }
}