use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll};
use mvc_lib::view::rusthtml::parser_parts::peekable_tokentree::StreamPeekableTokenTree;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_converter_out::RustHtmlParserConverterOut;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::{TokenTree, Ident};



#[test]
pub fn test_rusthtmlparser_converter_out_constructor() {
    let _ = RustHtmlParserConverterOut::new();
}

#[test]
pub fn test_rusthtmlparser_converter_out_convert_token() {
    let parser = RustHtmlParserAll::new_default();

    let ident = Ident::new("x", proc_macro2::Span::call_site());
    let token_rshtml = RustHtmlToken::Identifier(ident);
    
    let test_parser = parser.get_converter_out();
    let tokens = test_parser.convert_token(token_rshtml).unwrap();

    assert_eq!(tokens.len(), 4);
}