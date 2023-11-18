use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::directives::irusthtml_directive::IRustHtmlDirective;
use mvc_lib::view::rusthtml::directives::htmlfile_directive::HtmlFileDirective;
use mvc_lib::view::rusthtml::parser_parts::peekable_tokentree::{StreamPeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::RustHtmlParserAll;
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
    let it = Rc::new(StreamPeekableTokenTree::new(rust.clone()));
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = RustHtmlParserAll::new_default();
    let ident_token = it.next().unwrap();
    let identifier = match &ident_token {
        TokenTree::Ident(i) => i,
        _ => panic!("Expected an identifier."),
    };

    let mut output = vec![];

    let x = HtmlFileDirective::new();
    let ct = Rc::new(CancellationToken::new());

    match x.execute(ctx, &identifier, &ident_token, parser, &mut output, it, ct) {
        Err(RustHtmlError(e)) =>
            assert!(e.starts_with("(@htmlfile) cannot read external HTML file, could not parse path")),
        Ok(x) => {
            assert_eq!(x, RustHtmlDirectiveResult::OkContinue);
            assert_eq!(output.len(), 1);
        },
    }
}

#[test]
fn htmlfile_directive_basic_readme() {
    let rust = quote::quote! {
        htmlfile "../README.md"
    };
    let it = Rc::new(StreamPeekableTokenTree::new(rust.clone()));
    let ctx = Rc::new(RustHtmlParserContext::new(false, true, "test".to_string()));
    let parser = RustHtmlParserAll::new_default();
    let ident_token = it.next().unwrap();
    let identifier = match &ident_token {
        TokenTree::Ident(i) => i,
        _ => panic!("Expected an identifier."),
    };

    let mut output = vec![];

    let x = HtmlFileDirective::new();
    let ct = Rc::new(CancellationToken::new());

    match x.execute(ctx, &identifier, &ident_token, parser, &mut output, it, ct) {
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
                            //assert_eq!("", format!("Expected literal, found {:?}", tokens.first().unwrap()));
                        }
                    }
                },
                _ => assert!(false),
            }
        }
    }
}