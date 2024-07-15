use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::parserv3::parserv3::ParserV3;



#[test]
fn parserv3_awd() {
    // arrange
    let parser = ParserV3::new_default();
    let input = quote::quote! {
        @name "HelloWorld"
    };
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let ct = Rc::new(CancellationToken::new());

    // act
    let result = parser.expand_tokentree(input, context, ct);

    // assert
    match result {
        Ok(output) => {
            assert!(true);
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e);
        }
    }
}