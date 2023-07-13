use std::rc::Rc;

use mvc_lib::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunct, RustHtmlIdentOrPunctOrGroup};
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlIdentAndPunctOrLiteral;
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
    let rust_directive = quote::quote! { _ };
    let token = rust_directive.into_iter().next().unwrap();
    let actual_result = converter.convert_rust_directive_to_rusthtmltoken(token, None, &mut output, it.clone(), false).unwrap();
    assert_eq!(true, actual_result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_views_path_str() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let it = Rc::new(PeekableTokenTree::new(quote::quote! {
        test "_"
    }));
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
    let path = "_";
    converter.resolve_views_path_str(path).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_expand_external_rshtml_string() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    converter.expand_external_rshtml_string(&"".to_string(), &mut output).unwrap();
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
pub fn rust_to_rusthtml_converter_convert_string_or_ident_empty_error() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    assert_eq!(true, converter.convert_string_or_ident(it, false).is_err());
}

#[test]
pub fn rust_to_rusthtml_converter_convert_string_or_ident_basic_works() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let it = Rc::new(PeekableTokenTree::new(quote::quote! {
        test
    }));
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
pub fn rust_to_rusthtml_converter_convert_rusthtmltokens_to_ident_or_punct_or_group_empty_error() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    assert_eq!(true, converter.convert_rusthtmltokens_to_ident_or_punct_or_group(vec![]).is_err());
}

#[test]
pub fn rust_to_rusthtml_converter_convert_rusthtmltokens_to_ident_or_punct_or_group_basic_works() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    converter.convert_rusthtmltokens_to_ident_or_punct_or_group(vec![
        RustHtmlToken::Identifier(Ident::new("test", Span::call_site()))
    ]).unwrap();
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
    assert_eq!(false, result);
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
            match converter.convert_html_ident_to_rusthtmltoken(&ident, &mut tag_ctx, &mut output, it, is_raw_tokenstream) {
                Ok(_) => {
                    // assert_ne!(0, output.len());
                },
                Err(RustHtmlError(e)) => {
                    panic!("expected Some: {}", e);
                }
            }
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
    parse_ctx.parse_attrs = true;
    let is_raw_tokenstream = false;
    
    converter.convert_html_literal_to_rusthtmltoken(&literal, &mut parse_ctx, &mut output, is_raw_tokenstream).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_convert_html_punct_to_rusthtmltoken() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let punct = Punct::new('_', Spacing::Alone);
    let mut output = vec![];
    let mut parse_ctx = HtmlTagParseContext::new();
    let is_raw_tokenstream = false;
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    converter.convert_html_punct_to_rusthtmltoken(&punct, &mut parse_ctx, &mut output, it, is_raw_tokenstream).unwrap();
}

#[test]
#[should_panic]
pub fn rust_to_rusthtml_converter_on_kvp_defined_when_empty_panics() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut ctx = HtmlTagParseContext::new();
    let mut output = vec![];
    converter.on_kvp_defined(&mut ctx, &mut output).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_on_kvp_defined_when_not_empty_works() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut ctx = HtmlTagParseContext::new();
    ctx.html_attr_key = "test".to_string();
    ctx.html_attr_key_ident.push(RustHtmlIdentOrPunct::Ident(Ident::new("test", Span::call_site())));
    ctx.equals_punct = Some(Punct::new('=', Spacing::Alone));
    ctx.html_attr_val_literal = Some(Literal::string("test"));
    let mut output = vec![];
    converter.on_kvp_defined(&mut ctx, &mut output).unwrap();
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
pub fn rust_to_rusthtml_converter_convert_ident_and_punct_and_group_or_literal_to_tokenstream_empty_error() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let tag = RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(vec![]);
    assert_eq!(true, converter.convert_ident_and_punct_and_group_or_literal_to_tokenstream(&tag).is_err());
}

#[test]
pub fn rust_to_rusthtml_converter_convert_ident_and_punct_and_group_or_literal_to_tokenstream_basic_works() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let tag = RustHtmlIdentAndPunctAndGroupOrLiteral::IdentAndPunctAndGroup(vec![
        RustHtmlIdentOrPunctOrGroup::Ident(Ident::new("test", Span::call_site()))
    ]);
    let actual_tokenstream = converter.convert_ident_and_punct_and_group_or_literal_to_tokenstream(&tag).unwrap();
    assert_eq!("test".to_string(), actual_tokenstream.to_string());
}

#[test]
pub fn rust_to_rusthtml_converter_get_context() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    converter.get_context();
}

#[test]
pub fn rust_to_rusthtml_converter_parse_complex() {
    let parse_context = RustHtmlParserContext::new(false, false, "test".to_string());
    let converter = RustToRustHtmlConverter::new(Rc::new(parse_context));
    let output = converter.parse_tokenstream_to_rusthtmltokens(
        false,
        Rc::new(PeekableTokenTree::new(quote::quote! {
            fn test() {
                println!("test");
            }
        })),
        false
    ).unwrap();
    
    let expected_output = vec![
        RustHtmlToken::Identifier(Ident::new("fn", Span::call_site())),
        RustHtmlToken::Identifier(Ident::new("test", Span::call_site())),
        RustHtmlToken::Group(Delimiter::Parenthesis, Group::new(Delimiter::Parenthesis, TokenStream::new())),
        RustHtmlToken::GroupParsed(Delimiter::Brace, vec![
            RustHtmlToken::Identifier(Ident::new("println", Span::call_site())),
            RustHtmlToken::ReservedChar('!', Punct::new('!', Spacing::Alone)),
            RustHtmlToken::Group(Delimiter::Parenthesis, Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vec![
                TokenTree::Literal(Literal::string("test")),
            ].into_iter())).into()),
            RustHtmlToken::ReservedChar(';', Punct::new(';', Spacing::Alone)),
        ]),
    ];

    assert_eq!(expected_output.len(), output.len());

    compare_rusthtmltokens(&expected_output, &output);
}

#[test]
pub fn rust_to_rusthtml_converter_parse_complex_if_else_followed_by_html() {
    let parse_context = RustHtmlParserContext::new(false, false, "test".to_string());
    let converter = RustToRustHtmlConverter::new(Rc::new(parse_context));
    let output = converter.parse_tokenstream_to_rusthtmltokens(
        false,
        Rc::new(PeekableTokenTree::new(quote::quote! {
            let html_class = if is_active { "active" } else { "" };
            <p class=@html_class>test</p>
        })),
        false
    ).unwrap();
    
    let expected_output = vec![
        RustHtmlToken::Identifier(Ident::new("let", Span::call_site())),
        RustHtmlToken::Identifier(Ident::new("html_class", Span::call_site())),
        RustHtmlToken::ReservedChar('=', Punct::new('=', Spacing::Alone)),
        RustHtmlToken::Identifier(Ident::new("if", Span::call_site())),
        RustHtmlToken::Identifier(Ident::new("is_active", Span::call_site())),
        RustHtmlToken::GroupParsed(Delimiter::Brace, vec![
            RustHtmlToken::Literal(Some(Literal::string("active")), None),
        ]),
        RustHtmlToken::Identifier(Ident::new("else", Span::call_site())),
        RustHtmlToken::GroupParsed(Delimiter::Brace, vec![
            RustHtmlToken::Literal(Some(Literal::string("")), None),
        ]),
        RustHtmlToken::ReservedChar(';', Punct::new(';', Spacing::Alone)),
        RustHtmlToken::HtmlTagStart("p".to_string(), Some(vec![RustHtmlIdentOrPunct::Ident(Ident::new("p", Span::call_site()))])),
        RustHtmlToken::HtmlTagAttributeName("class".to_string(), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(vec![RustHtmlIdentOrPunct::Ident(Ident::new("class", Span::call_site()))]))),
        RustHtmlToken::HtmlTagAttributeEquals('=', Some(Punct::new('=', Spacing::Alone))),
        RustHtmlToken::HtmlTagAttributeValue(
            None, None, Some(vec![
                RustHtmlToken::Identifier(Ident::new("html_class", Span::call_site())),
            ])
        ),
        RustHtmlToken::HtmlTagCloseStartChildrenPunct('>', Some(Punct::new('>', Spacing::Alone))),
        RustHtmlToken::HtmlTextNode("test".to_string(), Span::call_site()),
        RustHtmlToken::HtmlTagEnd("p".to_string(), Some(vec![RustHtmlIdentOrPunct::Ident(Ident::new("p", Span::call_site()))])),
    ];

    compare_rusthtmltokens(&expected_output, &output);
}

fn compare_rusthtmltokens(expected_output: &Vec<RustHtmlToken>, output: &Vec<RustHtmlToken>) {
    let mut previous_token: Option<&RustHtmlToken> = None;
    for i in 0..std::cmp::min(expected_output.len(), output.len()) {
        let expected = &expected_output[i];
        let actual = &output[i];

        match (expected, actual) {
            (RustHtmlToken::Identifier(expected_ident), RustHtmlToken::Identifier(actual_ident)) => {
                assert_eq!(expected_ident.to_string(), actual_ident.to_string());
            },
            (RustHtmlToken::Group(expected_delimiter, expected_group), RustHtmlToken::Group(actual_delimiter, actual_group)) => {
                assert_eq!(*expected_delimiter, *actual_delimiter);
                assert_eq!(expected_group.to_string(), actual_group.to_string());
            },
            (RustHtmlToken::GroupParsed(expected_delimiter, expected_group), RustHtmlToken::GroupParsed(actual_delimiter, actual_group)) => {
                assert_eq!(*expected_delimiter, *actual_delimiter);
                compare_rusthtmltokens(expected_group, actual_group);
            },
            (RustHtmlToken::ReservedChar(expected_c, _), RustHtmlToken::ReservedChar(actual_c, _)) => {
                assert_eq!(*expected_c, *actual_c);
            },
            (RustHtmlToken::Literal(expected_literal, expected_str), RustHtmlToken::Literal(actual_literal, actual_str)) => {
                let expected = if let Some(s) = expected_literal { s.to_string() } else { expected_str.as_ref().unwrap().clone() };
                let actual = if let Some(s) = actual_literal { s.to_string() } else { actual_str.as_ref().unwrap().clone() };
                assert_eq!(expected, actual);
            },
            (RustHtmlToken::HtmlTagStart(expected_tag, expected_idents), RustHtmlToken::HtmlTagStart(actual_tag, actual_idents)) => {
                assert_eq!(expected_tag, actual_tag);
                assert_vecs_of_option_rusthtmlidentsorpunct_eq(expected_idents, actual_idents);
            },
            (RustHtmlToken::HtmlTagEnd(expected_tag, expected_idents), RustHtmlToken::HtmlTagEnd(actual_tag, actual_idents)) => {
                assert_eq!(expected_tag, actual_tag);
                assert_vecs_of_option_rusthtmlidentsorpunct_eq(expected_idents, actual_idents);
            },
            (RustHtmlToken::HtmlTagAttributeName(expected_name, expected_idents), RustHtmlToken::HtmlTagAttributeName(actual_name, actual_idents)) => {
                assert_eq!(expected_name, actual_name);
                assert_vecs_of_rusthtmlidentsandpuncts_or_literal_eq(expected_idents, actual_idents);
            },
            (RustHtmlToken::HtmlTagAttributeEquals(expected_c, _), RustHtmlToken::HtmlTagAttributeEquals(actual_c, _)) => {
                assert_eq!(*expected_c, *actual_c);
            },
            (RustHtmlToken::HtmlTagAttributeValue(expected_literal, expected_idents, expected_rust), RustHtmlToken::HtmlTagAttributeValue(actual_literal, actual_idents, actual_rust)) => {
                if let Some(expected_literal) = expected_literal {
                    assert_eq!(expected_literal.to_string(), actual_literal.as_ref().unwrap().to_string());
                } else if let Some(_) = expected_idents {
                    // need new function similar to existing one
                    assert_vecs_of_rusthtmlidentsorpunct_eq(expected_idents.as_ref().unwrap(), actual_idents.as_ref().unwrap());
                } else if let Some(expected_rust) = expected_rust {
                    compare_rusthtmltokens(expected_rust, actual_rust.as_ref().unwrap());
                } else {
                    panic!("expected and actual are not the same or not supported: expected: {:?}, actual: {:?}", expected, actual);
                }
            },
            (RustHtmlToken::HtmlTagCloseStartChildrenPunct(expected_c, _), RustHtmlToken::HtmlTagCloseStartChildrenPunct(actual_c, _)) => {
                assert_eq!(*expected_c, *actual_c);
            },
            (RustHtmlToken::HtmlTextNode(expected_text, _), RustHtmlToken::HtmlTextNode(actual_text, _)) => {
                assert_eq!(expected_text, actual_text);
            },
            _ => {
                let expected_next = if i + 1 < expected_output.len() { Some(&expected_output[i + 1]) } else { None };
                let actual_next = if i + 1 < output.len() { Some(&output[i + 1]) } else { None };
                panic!("expected and actual are not the same or not supported: expected: {:?} (next: {:?}), actual: {:?} (next: {:?}), previous: {:?}", 
                        expected, expected_next, actual, actual_next, previous_token)
            }
        }

        previous_token = Some(&actual);
    }
    assert_eq!(expected_output.len(), output.len());
}

fn assert_vecs_of_rusthtmlidentsandpuncts_or_literal_eq(expected_idents: &Option<RustHtmlIdentAndPunctOrLiteral>, actual_idents: &Option<RustHtmlIdentAndPunctOrLiteral>) {
    match (expected_idents, actual_idents) {
        (Some(expected_idents), Some(actual_idents)) => {
            match (expected_idents, actual_idents) {
                (RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(expected_idents), RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(actual_idents)) => {
                    assert_vecs_of_rusthtmlidentsorpunct_eq(expected_idents, actual_idents);
                },
                (RustHtmlIdentAndPunctOrLiteral::Literal(expected_literal), RustHtmlIdentAndPunctOrLiteral::Literal(actual_literal)) => {
                    assert_eq!(expected_literal.to_string(), actual_literal.to_string());
                },
                _ => panic!("expected and actual are not the same or not supported: expected: {:?}, actual: {:?}", expected_idents, actual_idents)
            }
        },
        (None, None) => {},
        _ => panic!("expected and actual are not the same or not supported: expected: {:?}, actual: {:?}", expected_idents, actual_idents)
    }
}

fn assert_vecs_of_option_rusthtmlidentsorpunct_eq(
    expected_idents: &Option<Vec<RustHtmlIdentOrPunct>>,
    actual_idents: &Option<Vec<RustHtmlIdentOrPunct>>
) {
    if let Some(expected_idents) = expected_idents {
        if let Some(actual_idents) = actual_idents {
            assert_vecs_of_rusthtmlidentsorpunct_eq(expected_idents, actual_idents);
        } else {
            panic!("Actual idents is None");
        }
    } else {
        panic!("Expected idents is None");
    }
}

fn assert_vecs_of_rusthtmlidentsorpunct_eq(
    expected_idents: &Vec<RustHtmlIdentOrPunct>,
    actual_idents: &Vec<RustHtmlIdentOrPunct>
) {
    for i in 0..std::cmp::min(expected_idents.len(), actual_idents.len()) {
        let expected_ident = &expected_idents[i];
        let actual_ident = &actual_idents[i];
        match (expected_ident, actual_ident) {
            (RustHtmlIdentOrPunct::Ident(expected_ident), RustHtmlIdentOrPunct::Ident(actual_ident)) => {
                assert_eq!(expected_ident.to_string(), actual_ident.to_string());
            },
            (RustHtmlIdentOrPunct::Punct(expected_punct), RustHtmlIdentOrPunct::Punct(actual_punct)) => {
                assert_eq!(expected_punct.as_char(), actual_punct.as_char());
            },
            _ => {
                panic!("expected and actual are not the same or not supported: expected: {:?}, actual: {:?}", expected_ident, actual_ident)
            }
        }
    }
    assert_eq!(expected_idents.len(), actual_idents.len());
}