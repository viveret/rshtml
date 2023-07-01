use std::rc::Rc;

use mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::{TokenTree, Group, Delimiter, TokenStream, Punct, Spacing};


#[test]
pub fn rust_to_rusthtml_converter_constructor_works() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let _ = RustToRustHtmlConverter::new(parser_context);
}

#[test]
pub fn rust_to_rusthtml_converter_panic_or_return_error_works_for_error() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);
    assert_eq!(true, converter.panic_or_return_error::<()>("test".to_string()).is_err());
}

#[test]
#[should_panic]
pub fn rust_to_rusthtml_converter_panic_or_return_error_works_for_panic() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, true, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);

    // should not get to below statement
    let _ = converter.panic_or_return_error::<()>("test".to_string());
}

#[test]
pub fn rust_to_rusthtml_converter_peek_reserved_char() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);
    let it = Rc::new(PeekableTokenTree::new(quote::quote! { . }));
    let mut output = vec![];
    assert_eq!(false, converter.peek_reserved_char('a', &mut output, it.clone(), false).unwrap());
    assert_eq!(true, converter.peek_reserved_char('.', &mut output, it, false).unwrap());
}

#[test]
pub fn rust_to_rusthtml_converter_parse_tokenstream_to_rusthtmltokens_works() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);
    let rust_output = quote::quote! {
        fn test() {
            println!("test");
        }
    };
    let it = Rc::new(PeekableTokenTree::new(rust_output));
    let p = converter.parse_tokenstream_to_rusthtmltokens(false, it, false).unwrap();
    assert_eq!(true, p.len() > 0);
}

#[test]
pub fn rust_to_rusthtml_converter_loop_next_and_convert_works() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);
    let rust_output = quote::quote! {
        fn test() {
            println!("test");
        }
    };
    let it = Rc::new(PeekableTokenTree::new(rust_output));
    let mut output = vec![];
    converter.loop_next_and_convert(false, &mut output, it, false).unwrap();
    assert_eq!(true, output.len() > 0);
}

#[test]
pub fn rust_to_rusthtml_converter_next_and_convert_works() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);
    let rust_output = quote::quote! {
        fn test() {
            println!("test");
        }
    };
    let it = Rc::new(PeekableTokenTree::new(rust_output));
    let mut output = vec![];
    converter.next_and_convert(false, &mut output, it, false).unwrap();
    assert_eq!(true, output.len() > 0);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_tokentree_to_rusthtmltoken_works() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(parser_context);
    let rust_output = quote::quote! {
        fn test() {
            println!("test");
        }
    };
    let it = Rc::new(PeekableTokenTree::new(rust_output));
    let mut output = vec![];
    let is_in_html_mode = false;
    let token = it.next().unwrap();
    converter.convert_tokentree_to_rusthtmltoken(token, is_in_html_mode, &mut output, it, false).unwrap();
    assert_eq!(true, output.len() > 0);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_group_to_rusthtmltoken_works() {
    let is_in_html_mode = false;
    let expect_return_html = false;

    // try some different group braces
    for delimiter in [Delimiter::Brace, Delimiter::Bracket, Delimiter::Parenthesis].iter() {
        let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
        let converter = RustToRustHtmlConverter::new(parser_context);
        let mut output = vec![];

        let group_token = Group::new(*delimiter, TokenStream::new());
        converter.convert_group_to_rusthtmltoken(group_token, expect_return_html, is_in_html_mode, &mut output, false).unwrap();
        assert_eq!(true, output.len() > 0);
    }
}

#[test]
pub fn rust_to_rusthtml_converter_convert_punct_to_rusthtmltoken_works() {
    let is_in_html_mode = false;

    let returns_true_chars = vec!['}'];
    let returns_empty_output = vec!['}', '<', '@'];

    // try some different puncts
    for spacing in [Spacing::Alone, Spacing::Joint].iter() {
        for expected_c in ['.', ',', ';', ':', '(', ')', '[', ']', '{', '}', '<', '>', '=', '!', '+', '-', '*', '/', '&', '|', '^', '%', '@', '#', '$', '~', '?'].iter() {
            let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
            let converter = RustToRustHtmlConverter::new(parser_context);
            let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));

            let mut output = vec![];
            let punct_token = Punct::new(*expected_c, *spacing);
            let actual_result = converter.convert_punct_to_rusthtmltoken(punct_token, is_in_html_mode, &mut output, it.clone(), false).unwrap();
            let expected_result = returns_true_chars.contains(expected_c);
            if expected_result != actual_result {
                panic!("expected_result != actual_result for c: '{}'", expected_c);
            }

            if returns_empty_output.contains(expected_c) {
                if output.len() != 0 {
                    panic!("output.len() != 0 for c: '{}'", expected_c);
                }
            } else {
                if output.len() == 0 {
                    panic!("output.len() == 0 for c: '{}'", expected_c);
                }
                let actual_c = match output.first().unwrap() {
                    RustHtmlToken::ReservedChar(c, _) => c,
                    _ => panic!("expecting punct token")
                };
                assert_eq!(*expected_c, *actual_c);
            }
        }
    }
}


pub fn rust_to_rusthtml_converter_convert_html_entry_to_rusthtmltoken() {
    
}

pub fn rust_to_rusthtml_converter_convert_html_entry_to_rusthtmltoken2() {
    
}