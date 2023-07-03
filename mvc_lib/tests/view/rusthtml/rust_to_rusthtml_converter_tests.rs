use std::rc::Rc;

use mvc_lib::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup};
use proc_macro2::{TokenTree, Group, Delimiter, TokenStream, Punct, Spacing, Ident, Span, Literal};


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


#[test]
pub fn rust_to_rusthtml_converter_convert_html_entry_to_rusthtmltoken() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let is_in_html_mode = false;
    let html_entry = quote::quote! { <div> };
    let c = '<';
    let punct = Punct::new(c, Spacing::Alone);
    converter.convert_html_entry_to_rusthtmltoken(c, punct, is_in_html_mode, &mut output, it.clone(), false).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_convert_rust_directive_to_rusthtmltoken_ident() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let rust_directive = quote::quote! { fn test() {} };
    let token = rust_directive.into_iter().next().unwrap();
    let actual_result = converter.convert_rust_directive_to_rusthtmltoken(token, None, &mut output, it.clone(), false).unwrap();
    assert_eq!(true, actual_result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_rust_directive_to_rusthtmltoken_group() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let rust_directive = quote::quote! { (fn test() {}) };
    let token = rust_directive.into_iter().next().unwrap();
    let actual_result = converter.convert_rust_directive_to_rusthtmltoken(token, None, &mut output, it.clone(), false).unwrap();
    assert_eq!(true, actual_result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_rust_directive_to_rusthtmltoken_literal() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let rust_directive = quote::quote! { "fn test() {}" };
    let token = rust_directive.into_iter().next().unwrap();
    let actual_result = converter.convert_rust_directive_to_rusthtmltoken(token, None, &mut output, it.clone(), false).unwrap();
    assert_eq!(true, actual_result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_rust_directive_to_rusthtmltoken_punct() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let rust_directive = quote::quote! { / };
    let token = rust_directive.into_iter().next().unwrap();
    let actual_result = converter.convert_rust_directive_to_rusthtmltoken(token, None, &mut output, it.clone(), false).unwrap();
    assert_eq!(true, actual_result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_views_path_str() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let token = it.next().unwrap();
    if let TokenTree::Ident(identifier) = token {
        let is_raw_tokenstream = false;
        let result = converter.convert_views_path_str(identifier, it, is_raw_tokenstream).unwrap();
        assert_eq!(true, result.len() > 0);
    } else {
        panic!("expected TokenTree::Ident");
    }
}

#[test]
pub fn rust_to_rusthtml_converter_resolve_views_path_str() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let path = "";
    converter.resolve_views_path_str(path).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_expand_external_tokenstream() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    converter.expand_external_tokenstream(&"".to_string(), &mut output).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_is_start_of_current_expression() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let x = converter.is_start_of_current_expression(&mut output);
    assert_eq!(true, x);
}

#[test]
pub fn rust_to_rusthtml_converter_parse_identifier_expression() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let identifier = Ident::new("test", Span::call_site());
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let is_raw_tokenstream = false;
    converter.parse_identifier_expression(identifier, &mut output, it, is_raw_tokenstream).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_convert_string_or_ident() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let result = converter.convert_string_or_ident(it, false).unwrap();
    match result {
        RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(ident_or_punct_or_group) => {
            let x = ident_or_punct_or_group.first().unwrap();
            match x {
                RustHtmlIdentOrPunctOrGroup::Ident(ident) => {
                    assert_eq!("test", ident.to_string());
                },
                _ => panic!("expected RustHtmlToken::Ident")
            }
        },
        _ => panic!("expected RustHtmlToken::Ident")
    }
}

#[test]
pub fn rust_to_rusthtml_converter_convert_rusthtmltokens_to_ident_or_punct_or_group() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    converter.convert_rusthtmltokens_to_ident_or_punct_or_group(vec![]).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_next_and_parse_html_tag() {
    let mut tag_ctx = HtmlTagParseContext::new();
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(ctx.clone());
    let token = TokenTree::Ident(Ident::new("test", Span::call_site()));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let is_raw_tokenstream = false;
    let result = converter.next_and_parse_html_tag(Some(token), &mut tag_ctx, &mut output, it, is_raw_tokenstream).unwrap();
    assert_eq!(true, result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_html_ident_to_rusthtmltoken() {
    let mut tag_ctx = HtmlTagParseContext::new();
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(quote::quote! { test }));
    let ident = it.next().unwrap();
    let is_raw_tokenstream = false;
    match ident {
        TokenTree::Ident(ident) => {
            converter.convert_html_ident_to_rusthtmltoken(&ident, &mut tag_ctx, &mut output, it, is_raw_tokenstream).unwrap();
            assert_ne!(0, output.len());
        },
        _ => panic!("expected TokenTree::Ident")
    }
}

#[test]
pub fn rust_to_rusthtml_converter_convert_html_literal_to_rusthtmltoken() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let literal = Literal::string("test");
    let mut output = vec![];
    let mut parse_ctx = HtmlTagParseContext::new();
    let is_raw_tokenstream = false;
    converter.convert_html_literal_to_rusthtmltoken(&literal, &mut parse_ctx, &mut output, is_raw_tokenstream).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_convert_html_punct_to_rusthtmltoken() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let punct = Punct::new('<', Spacing::Alone);
    let mut output = vec![];
    let mut parse_ctx = HtmlTagParseContext::new();
    let is_raw_tokenstream = false;
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    converter.convert_html_punct_to_rusthtmltoken(&punct, &mut parse_ctx, &mut output, it, is_raw_tokenstream).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_on_kvp_defined() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut ctx = HtmlTagParseContext::new();
    let mut output = vec![];
    converter.on_kvp_defined(&mut ctx, &mut output);
}

#[test]
pub fn rust_to_rusthtml_converter_parse_type_identifier() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let it = Rc::new(PeekableTokenTree::new(quote::quote! { test }));
    let actual_tokens = converter.parse_type_identifier(it).unwrap();
    assert_eq!(true, actual_tokens.len() > 0);
}

#[test]
pub fn rust_to_rusthtml_converter_on_html_tag_parsed() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let punct = Punct::new('<', Spacing::Alone);
    let mut parse_ctx = HtmlTagParseContext::new();
    let mut output = vec![];
    let result = converter.on_html_tag_parsed(&punct, &mut parse_ctx, &mut output).unwrap();
    assert_eq!(true, result);
}

#[test]
pub fn rust_to_rusthtml_converter_on_html_node_parsed() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let ctx = HtmlTagParseContext::new();
    let mut output = vec![];
    let result = converter.on_html_node_parsed(&ctx, &mut output).unwrap();
    assert_eq!(true, result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_copy() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let token = TokenTree::Ident(Ident::new("test", Span::call_site()));
    let mut output = vec![];
    converter.convert_copy(token, &mut output).unwrap();
    assert_ne!(0, output.len());
}

#[test]
pub fn rust_to_rusthtml_converter_convert_ident_and_punct_and_group_or_literal_to_tokenstream() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let tag = RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(vec![]);
    let actual_tokenstream = converter.convert_ident_and_punct_and_group_or_literal_to_tokenstream(&tag).unwrap();
    assert_eq!("".to_string(), actual_tokenstream.to_string());
}

#[test]
pub fn rust_to_rusthtml_converter_get_context() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    converter.get_context();
}