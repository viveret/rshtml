use std::rc::Rc;

use mvc_lib::view::rusthtml::irusthtml_to_rust_converter::IRustHtmlToRustConverter;
use mvc_lib::view::rusthtml::peekable_rusthtmltoken::{PeekableRustHtmlToken, IPeekableRustHtmlToken};
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rusthtml_to_rust_converter::RustHtmlToRustConverter;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::{Delimiter, Literal};



#[test]
pub fn rusthtml_to_rust_converter_constructor_works() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let _converter = RustHtmlToRustConverter::new(context);
}

#[test]
pub fn rusthtml_to_rust_converter_parse_rusthtmltokens_to_plain_rust_works() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let result = converter.parse_rusthtmltokens_to_plain_rust(&vec![]).unwrap();
    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_convert_rusthtmltokens_to_plain_rust() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let mut output = vec![];
    let input = vec![];
    let it = PeekableRustHtmlToken::new(&input);
    let result = converter.convert_rusthtmltokens_to_plain_rust(&mut output, &it).unwrap();
    assert_eq!(true, result);
    assert_eq!(0, output.len());
}

#[test]
pub fn rusthtml_to_rust_converter_convert_rusthtmlgroupparsed_to_tokentree() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let mut output = vec![];
    let input = vec![];
    let it = PeekableRustHtmlToken::new(&input);
    converter.convert_rusthtmlgroupparsed_to_tokentree(&Delimiter::Bracket, &vec![], &mut output, &it).unwrap();
}

#[test]
pub fn rusthtml_to_rust_converter_convert_rusthtmlappendhtml_to_tokentree() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let mut output = vec![];
    let inner = None;
    converter.convert_rusthtmlappendhtml_to_tokentree(Some(&"".to_string()), inner, &mut output).unwrap();
}

#[test]
pub fn rusthtml_to_rust_converter_convert_rusthtmltoken_to_tokentree() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let mut output = vec![];
    let input = vec![];
    let it = PeekableRustHtmlToken::new(&input);
    let token = RustHtmlToken::Literal(Some(Literal::string("")), None);
    converter.convert_rusthtmltoken_to_tokentree(&token, &mut output, &it).unwrap();
}

#[test]
pub fn rusthtml_to_rust_converter_preprocess_rusthtmltokens() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let input = vec![];
    let result = converter.preprocess_rusthtmltokens(&input).unwrap();
    assert_eq!(0, result.len());
}

#[test]
pub fn rusthtml_to_rust_converter_postprocess_rusthtmltokens() {
    let is_raw_tokenstream = false;
    let should_panic_or_return_error = true;
    let environment_name = "test".to_string();
    let context = Rc::new(RustHtmlParserContext::new(is_raw_tokenstream, should_panic_or_return_error, environment_name));
    let converter = RustHtmlToRustConverter::new(context);
    let input = vec![];
    let result = converter.postprocess_rusthtmltokens(&input).unwrap();
    assert_eq!(0, result.len());
}