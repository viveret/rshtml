use std::rc::Rc;

use mvc_lib::view::rusthtml::directives::irusthtml_directive::IRustHtmlDirective;
use mvc_lib::view::rusthtml::directives::htmlfile_directive::HtmlFileDirective;
use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::TokenTree;


#[test]
fn htmlfile_directive_constructor() {
    let x = HtmlFileDirective::new();
    assert!(x.matches(&"htmlfile".to_string()));
}

#[test]
fn htmlfile_directive_basic_cannot_find_file() {
    let rust = quote::quote! {
        htmlfile "shared/_icon_svg.html"
    };
    let it = Rc::new(PeekableTokenTree::new(rust.clone()));
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = Rc::new(RustToRustHtmlConverter::new(ctx));
    let identifier = match it.next().unwrap() {
        TokenTree::Ident(i) => i,
        _ => panic!("Expected an identifier."),
    };

    let mut output = vec![];

    let x = HtmlFileDirective::new();

    match x.execute(&identifier, parser, &mut output, it) {
        Err(RustHtmlError(e)) =>
            assert!(e.starts_with("(@htmlfile) cannot read external HTML file, could not parse path")),
        _ => assert!(false),
    }
}

#[test]
fn htmlfile_directive_basic_readme() {
    let rust = quote::quote! {
        htmlfile "../README.md"
    };
    let it = Rc::new(PeekableTokenTree::new(rust.clone()));
    let ctx = Rc::new(RustHtmlParserContext::new(false, true, "test".to_string()));
    let parser = Rc::new(RustToRustHtmlConverter::new(ctx));
    let identifier = match it.next().unwrap() {
        TokenTree::Ident(i) => i,
        _ => panic!("Expected an identifier."),
    };

    let mut output = vec![];

    let x = HtmlFileDirective::new();

    match x.execute(&identifier, parser, &mut output, it) {
        Err(RustHtmlError(e)) =>
            assert_eq!("", e),
        Ok(r) => {
            assert_eq!(r, RustHtmlDirectiveResult::OkContinue);
            assert_eq!(output.len(), 1);
            match output.first().unwrap() {
                RustHtmlToken::AppendToHtml(tokens) => {
                    assert_eq!(1, tokens.len());
                    match tokens.first().unwrap() {
                        RustHtmlToken::Literal(l, s) => {
                            let actual_s = match l {
                                Some(l) => l.to_string(),
                                _ => {
                                    match s {
                                        Some(s) => s.to_string(),
                                        _ => String::new(),
                                    }
                                }
                            };
                            assert_eq!(std::fs::read_to_string("../example_web_app/README.md").unwrap(), actual_s);
                        },
                        _ => {
                            assert_eq!("", format!("Expected literal, found {:?}", tokens.first().unwrap()));
                        }
                    }
                },
                _ => assert!(false),
            }
        }
    }
}