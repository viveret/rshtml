use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use core_lib::assert::assert_tokentree::assert_tokentree_stream;
use mvc_lib::view::{parserv3::{contexts::rusthtml_parser_context::RustHtmlParserContext, converters::{iconverter_input::{ConverterInput, IConverterInput}, iparserv3_html_parser::{IParserV3HtmlParser, ParserV3HtmlParser}}, core::{peekable::stream_peekable_tokentree::StreamPeekableTokenTree, rusthtml_directive_result::RustHtmlDirectiveResult}, parserv3::ParserV3}, rusthtml::rusthtml_token::RustHtmlToken};




#[test]
pub fn parser_v3_html_parser_parse_tag() {
    // setup
    let parserv3 = ParserV3::new_default();
    let x = parserv3.get_html_parser();
    let input = quote::quote! {
        <!DOCTYPE html>
        <html>
        <head>
        </head>
        <body>
        </body>
        </html>
    };
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let ct = Rc::new(CancellationToken::new());

    // execute
    let input = Rc::new(StreamPeekableTokenTree::new(input));
    let input = parserv3.get_converter_in().convert(input);
    // skip starting <
    input.next();
    let result = x.parse_tag(input, context, ct.clone()).expect("parse tag returned error");

    // assert
    assert_eq!(RustHtmlDirectiveResult::OkContinue, result.0, "expected parse_tag to return OkContinue");
    let r = result.1.expect("expected output tokens");

    let peek_token = r.peek().expect("expected peek");
    let peek_token_type_name = peek_token.variant_name();
    assert_eq!("HtmlTagVoid", peek_token_type_name);
    if let RustHtmlToken::HtmlTagVoid(name, tokens) = peek_token {
        assert_eq!("!DOCTYPE", name);
    } else {
        panic!("not actually HtmlTagVoid")
    }
    
    let converted_out = parserv3.get_converter_out().convert(r, ct).expect("expected converter out to succeed");
    let out_stream = converted_out.to_stream();

    let expected_out_stream = quote::quote! {
        html_output.write_html_str("<!DOCTYPE");
        html_output.write_html_str(" html");
        html_output.write_html_str(">");
    };

    assert_tokentree_stream(expected_out_stream, out_stream);
}