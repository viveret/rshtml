use std::rc::Rc;

use mvc_lib::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;
use mvc_lib::view::parserv3::contexts::rusthtml_parser_context_log::RustHtmlParserContextLog;
use mvc_lib::view::parserv3::contexts::rusthtml_parser_context::MockRustHtmlParserContext;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::{Span, TokenTree};


#[test]
pub fn rusthtml_parser_context_log_test() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context_log = RustHtmlParserContextLog::new(mock_context.clone());

    // Execute
    let is_raw = context_log.get_is_raw_tokenstream();
    let model_type_name = context_log.get_model_type_name();

    // Assert
    assert!(is_raw, "Expected get_is_raw_tokenstream to return true");
    assert_eq!(model_type_name, "MockModel", "Expected get_model_type_name to return 'MockModel'");

    // Verify that the operations were logged
    let operations = context_log.get_order_of_operations();
    assert_eq!(operations.len(), 2, "Expected 2 operations to be logged");
    assert_eq!(operations[0], "get_is_raw_tokenstream", "Expected first operation to be 'get_is_raw_tokenstream'");
    assert_eq!(operations[1], "get_model_type_name", "Expected second operation to be 'get_model_type_name'");
}

#[test]
pub fn rusthtml_parser_context_log_push_output_token_test() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext {});
    let context_log = RustHtmlParserContextLog::new(mock_context.clone());

    // Execute
    let result = context_log.push_output_token(RustHtmlToken::Identifier(proc_macro2::Ident::new("test", Span::call_site())));

    // Assert
    assert!(result.is_ok(), "Expected push_output_token to return Ok");

    // Verify that the operation was logged
    let operations = context_log.get_order_of_operations();
    assert_eq!(operations.len(), 1, "Expected 1 operation to be logged");
    assert_eq!(operations[0], "push_output_token", "Expected operation to be 'push_output_token'");
}