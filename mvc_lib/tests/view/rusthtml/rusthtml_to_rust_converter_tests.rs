use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll};
use mvc_lib::view::rusthtml::parser_parts::peekable_rusthtmltoken::VecPeekableRustHtmlToken;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::{Delimiter, Literal, TokenStream};


#[test]
pub fn rusthtml_to_rust_converter_parse_rusthtmltokens_to_plain_rust_empty() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_converter_out();
    let ct = Rc::new(CancellationToken::new());
    let result = converter.convert_vec(vec![], ct).unwrap();
    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_parse_rusthtmltokens_to_plain_rust_basic_html() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_converter_out();

    let html = vec![
        RustHtmlToken::HtmlTagStart("html".to_string(), None),
        RustHtmlToken::HtmlTagStart("body".to_string(), None),
        RustHtmlToken::HtmlTagStart("div".to_string(), None),
        RustHtmlToken::HtmlTagEnd("div".to_string(), None),
        RustHtmlToken::HtmlTagEnd("body".to_string(), None),
        RustHtmlToken::HtmlTagEnd("html".to_string(), None),
    ];

    let ct = Rc::new(CancellationToken::new());
    let result = TokenStream::from_iter(converter.convert_vec(html, ct).unwrap());
    let expected_result = quote::quote! {
        html_output . write_html_str ( "<html><body><div></div></body></html>" ) ;
    };

    assert_ne!(expected_result.to_string(), result.to_string());
}

#[test]
pub fn rusthtml_convert_rusthtmltokens_to_plain_rust() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_converter_out();
    
    // act
    let input = vec![];
    let it = Rc::new(VecPeekableRustHtmlToken::new(input));
    let ct = Rc::new(CancellationToken::new());
    let result = converter.convert_rusthtmltokens_to_plain_rust(it, ct).unwrap();

    // assert
    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_convert_rusthtmlgroupparsed_to_tokentree() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_converter_out();
    
    let input = vec![];
    let it = Rc::new(VecPeekableRustHtmlToken::new(input));
    let ct = Rc::new(CancellationToken::new());
    let result = converter.convert_rusthtmlgroupparsed_to_tokentree(&Delimiter::Bracket, vec![], it, ct).unwrap();

    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_convert_rusthtmlappendhtml_to_tokentree() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_converter_out();

    let inner = None;
    let result = converter.convert_rusthtmlappendhtml_to_tokentree(Some(&"".to_string()), None, inner, None).unwrap();
}

#[test]
pub fn rusthtml_to_rust_converter_convert_rusthtmltoken_to_tokentree() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_converter_out();

    let input = vec![];
    let it = Rc::new(VecPeekableRustHtmlToken::new(input));
    let token = RustHtmlToken::Literal(Some(Literal::string("")), None);
    let ct = Rc::new(CancellationToken::new());
    let result = converter.convert_rusthtmltoken_to_tokentree(&token, it, ct).unwrap();

    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_preprocess_rusthtmltokens_empty() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_rust_or_html_parser();
    let input = vec![];
    let result = converter.preprocess_rusthtmltokens(&input).unwrap();
    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_postprocess_rusthtmltokens_empty() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_rust_or_html_parser();
    let input = vec![];
    let result = converter.postprocess_rusthtmltokens(&input).unwrap();
    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_postprocess_rusthtmltokens_complex() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_rust_or_html_parser();
    let input = vec![
        RustHtmlToken::HtmlTagStart("html".to_string(), None),
        RustHtmlToken::HtmlTagStart("body".to_string(), None),
        RustHtmlToken::HtmlTagStart("div".to_string(), None),
        RustHtmlToken::HtmlTagEnd("div".to_string(), None),
        RustHtmlToken::HtmlTagEnd("body".to_string(), None),
        RustHtmlToken::HtmlTagEnd("html".to_string(), None),
    ];
    let result = converter.postprocess_rusthtmltokens(&input).unwrap();
    assert_ne!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_postprocess_tokenstream() {
    let parser = RustHtmlParserAll::new_default();
    let converter = parser.get_rust_or_html_parser();
    let input = quote::quote! {
        html_output . write_html_str("<html>");
        html_output . write_html_str("<body>");
        html_output . write_html_str("<div>");
        html_output . write_html_str("</div>");
        html_output . write_html_str("</body>");
        html_output . write_html_str("</html>");
    }.into_iter().collect();

    let output = converter.postprocess_tokenstream(&input).unwrap();

    let result = TokenStream::from_iter(output.into_iter());
    assert_eq!("html_output . write_html_str (\"<html><body><div></div></body></html>\") ;", result.to_string());
}