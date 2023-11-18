use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parser_parts::peekable_tokentree::StreamPeekableTokenTree;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAssignSharedParts};
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_converter_in::{RustHtmlParserConverterIn, IRustHtmlParserConverterIn};
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::{TokenTree, Ident, Span};



#[test]
pub fn test_rusthtmlparser_converter_in_constructor_works() {
    let _ = RustHtmlParserConverterIn::new();
}

#[test]
pub fn test_rusthtmlparser_converter_in_assign_shared_parser_works() {
    let parser = RustHtmlParserConverterIn::new();
    let _ = parser.assign_shared_parser(RustHtmlParserAll::new_default());
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_works() {
    let parser = RustHtmlParserConverterIn::new();
    let ct = Rc::new(CancellationToken::new());
    let ident = Ident::new("test", Span::call_site());
    let tokens = parser.convert(TokenTree::Ident(ident), ct).unwrap();

    match tokens {
        RustHtmlToken::Identifier(identifier) => {
            assert_eq!(identifier.to_string(), "test");
        },
        _ => panic!("test_rusthtmlparser_converter_in_convert_works: tokens is not RustHtmlToken::Identifier")
    }
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_rust_works() {
    let input_stream = quote::quote! {
        test
    };
    let input = Rc::new(StreamPeekableTokenTree::new(input_stream));
    let parser = RustHtmlParserConverterIn::new();
    let tokens = parser.convert_rust(input, Rc::new(CancellationToken::new())).unwrap();

    assert_eq!(tokens.len(), 1);
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_stream_works() {
    let input_stream = quote::quote! {
        test
    };
    let parser = RustHtmlParserConverterIn::new();
    let tokens = parser.convert_stream(input_stream, Rc::new(CancellationToken::new())).unwrap();

    assert_eq!(tokens.len(), 1);
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_vec_works() {
    let input_stream = quote::quote! {
        test
    };
    let input = input_stream.into_iter().collect();
    let parser = RustHtmlParserConverterIn::new();
    let tokens = parser.convert_vec(input, Rc::new(CancellationToken::new())).unwrap();

    assert_eq!(tokens.len(), 1);
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_group_works() {
    let input_stream = quote::quote! {
        (test)
    };
    let input = input_stream.into_iter().collect::<Vec<TokenTree>>();
    let group = if let TokenTree::Group(g) = &input[0] {
        g
    } else {
        panic!("test_rusthtmlparser_converter_in_convert_group_works: input[0] is not a group");
    };
    let parser = RustHtmlParserConverterIn::new();
    let group_actual = parser.convert_group(group, false, Rc::new(CancellationToken::new())).unwrap();

    match group_actual {
        RustHtmlToken::GroupParsed(delimiter, tokens) => {
            assert_eq!(delimiter, group.delimiter());
            assert_eq!(tokens.len(), 1);
        },
        _ => panic!("test_rusthtmlparser_converter_in_convert_group_works: group_actual is not RustHtmlToken::GroupParsed")
    }
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_literal_works() {
    let input_stream = quote::quote! {
        "test"
    };
    let input = input_stream.into_iter().collect::<Vec<TokenTree>>();
    let literal = if let TokenTree::Literal(l) = &input[0] {
        l
    } else {
        panic!("test_rusthtmlparser_converter_in_convert_literal_works: input[0] is not a literal");
    };
    let parser = RustHtmlParserConverterIn::new();
    let literal_actual = parser.convert_literal(literal, Rc::new(CancellationToken::new())).unwrap();

    match literal_actual {
        RustHtmlToken::Literal(literal, _) => {
            assert_eq!(literal.unwrap().to_string(), "\"test\"");
        },
        _ => panic!("test_rusthtmlparser_converter_in_convert_literal_works: literal_actual is not RustHtmlToken::Literal")
    }
}

#[test]
pub fn test_rusthtmlparser_converter_in_convert_punct_works() {
    let input_stream = quote::quote! {
        ,
    };
    let input = input_stream.into_iter().collect::<Vec<TokenTree>>();
    let punct = if let TokenTree::Punct(p) = &input[0] {
        p
    } else {
        panic!("test_rusthtmlparser_converter_in_convert_punct_works: input[0] is not a punct");
    };
    let parser = RustHtmlParserConverterIn::new();
    let punct_actual = parser.convert_punct(punct).unwrap();

    match punct_actual {
        RustHtmlToken::ReservedChar(c, _) => {
            assert_eq!(c, ',');
        },
        _ => panic!("test_rusthtmlparser_converter_in_convert_punct_works: punct_actual is not RustHtmlToken::ReservedChar")
    }
}