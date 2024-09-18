use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::parserv3::parserv3::ParserV3;
use mvc_lib::view::parserv3::contexts::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::parserv3::contexts::irusthtml_parser_context::IRustHtmlParserContext;


#[test]
pub fn parserv3_minimal() {
    // arrange
    let parser = ParserV3::new_default();
    let input = quote::quote! {
        @name "HelloWorld"
    };
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let ct = Rc::new(CancellationToken::new());

    // act
    let result = parser.expand_tokentree(input, context.clone(), ct);

    // assert
    match result {
        Ok(output) => {
            assert!(true);
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e);
        }
    }

    // check that the name parameter is set
    match context.try_get_param_string("name") {
        Some(name) => {
            assert_eq!(name, "HelloWorld");
        }
        None => {
            assert!(false, "name parameter not found");
        }
    }
}

#[test]
pub fn parserv3_minimal_block_empty() {
    // arrange
    let parser = ParserV3::new_default();
    let input = quote::quote! {
        @name "HelloWorld"
        @{

        }
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

#[test]
pub fn parserv3_inject() {
    // arrange
    let parser = ParserV3::new_default();
    let input = quote::quote! {
        @name "HelloWorld"
        @inject custom_html: StacksHtmlHelpers::<AddViewModel>
    };
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let ct = Rc::new(CancellationToken::new());

    // act
    let result = parser.expand_tokentree(input, context.clone(), ct);

    // assert
    match result {
        Ok(output) => {
            assert!(true);
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e);
        }
    }

    // check that the name parameter is set
    match context.try_get_param_string("name") {
        Some(name) => {
            assert_eq!(name, "HelloWorld");
        }
        None => {
            assert!(false, "name parameter not found");
        }
    }
}

#[test]
pub fn parserv3_model() {
    // arrange
    let parser = ParserV3::new_default();
    let input = quote::quote! {
        @name "HelloWorld"
        @model AddViewModel
    };
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let ct = Rc::new(CancellationToken::new());

    // act
    let result = parser.expand_tokentree(input, context.clone(), ct);

    // assert
    match result {
        Ok(output) => {
            assert!(true);
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e);
        }
    }

    // check that the name parameter is set
    match context.try_get_param_string("name") {
        Some(name) => {
            assert_eq!(name, "HelloWorld");
        }
        None => {
            assert!(false, "name parameter not found");
        }
    }
}

#[test]
pub fn parserv3_model_with_inject() {
    // arrange
    let parser = ParserV3::new_default();
    let input = quote::quote! {
        @name "HelloWorld"
        @model AddViewModel
        @inject custom_html as StacksHtmlHelpers::<AddViewModel>
    };
    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let ct = Rc::new(CancellationToken::new());

    // act
    let result = parser.expand_tokentree(input, context.clone(), ct);

    // assert
    match result {
        Ok(output) => {
            assert!(true);
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e);
        }
    }

    // check that the name parameter is set
    match context.try_get_param_string("name") {
        Some(name) => {
            assert_eq!(name, "HelloWorld");
        }
        None => {
            assert!(false, "name parameter not found");
        }
    }
}