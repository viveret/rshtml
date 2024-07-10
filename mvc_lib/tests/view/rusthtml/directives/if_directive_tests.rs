use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::parserv3::converters::iconverter_input::{ConverterInput, IConverterInput};
use mvc_lib::view::parserv3::converters::iparserv3_rust_parser::ParserV3RustParser;
use mvc_lib::view::parserv3::parserv3::ParserV3;
use mvc_lib::view::rusthtml::parser_parts::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_directive_result::{RustHtmlDirectiveResult, RustHtmlDirectiveResultV3};
use mvc_lib::view::rusthtml::directives::irusthtml_directive::IRustHtmlDirective;
use mvc_lib::view::rusthtml::directives::if_directive::IfDirective;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::TokenTree;



#[test]
pub fn if_directive_process_rust_basic() {
    let ct = Rc::new(CancellationToken::new());
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
    let it = ConverterInput::new().convert(it);

    // skip the '@' and 'if' tokens
    it.as_ref().next();

    let first_token = it.next().unwrap();
    let first_ident = if let RustHtmlToken::Identifier(x) = &first_token { x } else { panic!("expected ident, not {:?}", first_token); };

    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = ParserV3::new_default();

    // begin processing
    let result = processor.execute_new_v3(context, &first_ident, &first_token, parser, it.clone(), ct).unwrap();
    match result.0 {
        RustHtmlDirectiveResult::OkContinue => {
            let rusthtml_actual = result.1.unwrap().to_splice().iter().map(|t| t.to_string()).collect::<Vec<String>>().join("");
            assert_eq!(rusthtml_expected_string.replace(" ", ""), rusthtml_actual.replace(" ", ""));
        },
        _ => panic!("expected OkContinue, not {:?}", result)
    }
}


// test if else
#[test]
fn if_directive_process_rust_basic_else() {
    let ct = Rc::new(CancellationToken::new());
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
    
    // convert it to rust html stream
    let it = ConverterInput::new().convert(it);
    
    // skip the '@' and 'if' tokens
    it.as_ref().next();

    let first_token = it.next().unwrap();
    let first_ident = if let RustHtmlToken::Identifier(x) = &first_token { x } else { panic!("expected ident, not {:?}", first_token); };

    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = ParserV3::new_default();

    // begin processing
    let result = processor.execute_new_v3(context, &first_ident, &first_token, parser.clone(), it.clone(), ct).unwrap();
    // assert_ne!(0, output.len());
    match result.0 {
        RustHtmlDirectiveResult::OkContinue => {
            let rusthtml_actual = result.1.unwrap().to_splice().iter().map(|t| t.to_string()).collect::<Vec<String>>().join("");
            assert_eq!(rusthtml_expected_string.replace(" ", ""), rusthtml_actual.replace(" ", ""));
        },
        _ => panic!("expected OkContinue, not {:?}", result)
    }
}