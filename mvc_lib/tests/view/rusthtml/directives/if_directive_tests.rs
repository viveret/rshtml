use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parsers::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use mvc_lib::view::rusthtml::parsers::rusthtmlparser_all::RustHtmlParserAll;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use mvc_lib::view::rusthtml::directives::irusthtml_directive::IRustHtmlDirective;
use mvc_lib::view::rusthtml::directives::if_directive::IfDirective;
use proc_macro2::TokenTree;



#[test]
pub fn if_directive_process_rust_basic() {
    let processor = IfDirective::new();
    let rusthtml = quote::quote! {
        @if true {
            <div>@"Hello, world!"</div>
        }
    };
    let rusthtml_expected = quote::quote! {
        if true {
            <div>"Hello, world!"</div>
        }
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let it = Rc::new(StreamPeekableTokenTree::new(rusthtml)) as Rc<dyn IPeekableTokenTree>;
    // skip the '@' and 'if' tokens
    it.as_ref().next();

    let first_token = it.next().unwrap();
    let first_ident = if let TokenTree::Ident(x) = &first_token { x } else { panic!("expected ident, not {:?}", first_token); };

    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = RustHtmlParserAll::new_default();

    // begin processing
    let mut output = Vec::new();
    let ct = Rc::new(CancellationToken::new());
    let result = processor.execute(context, &first_ident, &first_token, parser, &mut output, it, ct).unwrap();
    assert_ne!(0, output.len());
    match result {
        RustHtmlDirectiveResult::OkContinue => {
            let rusthtml_actual = output.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("");
            assert_eq!(rusthtml_expected_string.replace(" ", ""), rusthtml_actual.replace(" ", ""));
        },
        _ => panic!("expected OkContinue, not {:?}", result)
    }
}


// test if else
#[test]
fn if_directive_process_rust_basic_else() {
    let processor = IfDirective::new();
    let rusthtml = quote::quote! {
        @if true {
            <div>@"Hello, world!"</div>
        } else {
            <div>@"Hello, world!"</div>
        }
    };
    let rusthtml_expected = quote::quote! {
        if true {
            <div>"Hello, world!"</div>
        } else {
            <div>"Hello, world!"</div>
        }
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let it = Rc::new(StreamPeekableTokenTree::new(rusthtml)) as Rc<dyn IPeekableTokenTree>;
    // skip the '@' and 'if' tokens
    it.as_ref().next();

    let first_token = it.next().unwrap();
    let first_ident = if let TokenTree::Ident(x) = &first_token { x } else { panic!("expected ident, not {:?}", first_token); };

    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = RustHtmlParserAll::new_default();

    // begin processing
    let mut output = Vec::new();
    let ct = Rc::new(CancellationToken::new());
    let result = processor.execute(context, &first_ident, &first_token, parser, &mut output, it, ct).unwrap();
    assert_ne!(0, output.len());
    match result {
        RustHtmlDirectiveResult::OkContinue => {
            let rusthtml_actual = output.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("");
            assert_eq!(rusthtml_expected_string.replace(" ", ""), rusthtml_actual.replace(" ", ""));
        },
        _ => panic!("expected OkContinue, not {:?}", result)
    }
}