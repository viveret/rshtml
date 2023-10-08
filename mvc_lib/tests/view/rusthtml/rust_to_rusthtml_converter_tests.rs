use std::rc::Rc;

use assert_str::assert_str_eq;

use mvc_lib::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use mvc_lib::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use mvc_lib::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_parser_context_log::RustHtmlParserContextLog;
use mvc_lib::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunct, RustHtmlIdentOrPunctOrGroup};
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlIdentAndPunctOrLiteral;
use proc_macro2::{TokenTree, Group, Delimiter, TokenStream, Punct, Spacing, Ident, Span, Literal};


#[test]
pub fn rust_to_rusthtml_converter_constructor_works() {
    let parser_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let _ = RustToRustHtmlConverter::new(parser_context);
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
    let html_entry = quote::quote! { <div> };
    let it = Rc::new(PeekableTokenTree::new(html_entry));
    it.next();
    let is_in_html_mode = false;
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
    if let TokenTree::Ident(identifier) = &token {
        let is_raw_tokenstream = false;
        let result = converter.next_path_str(identifier, &token, it, is_raw_tokenstream).unwrap();
        assert_eq!(true, result.len() > 0);
    } else {
        panic!("expected TokenTree::Ident");
    }
}

// #[test]
// pub fn rust_to_rusthtml_converter_resolve_views_path_str() {
//     let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
//     let path = "_";
//     converter.path(path).unwrap();
// }

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
    let ident_token = TokenTree::Ident(identifier.clone());

    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let is_raw_tokenstream = false;

    let mut output = vec![];
    converter.parse_identifier_expression(true, &identifier, &ident_token, false, &mut output, it, is_raw_tokenstream).unwrap();
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
    let tag_ctx = Rc::new(HtmlTagParseContext::new(None));
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(ctx.clone());
    let token = TokenTree::Ident(Ident::new("test", Span::call_site()));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    let is_raw_tokenstream = false;
    let result = converter.next_and_parse_html_tag(&token, tag_ctx, &mut output, it, is_raw_tokenstream).unwrap();
    assert_eq!(false, result);
}

#[test]
pub fn rust_to_rusthtml_converter_convert_html_ident_to_rusthtmltoken() {
    let tag_ctx = Rc::new(HtmlTagParseContext::new(None));
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let mut output = vec![];
    let it = Rc::new(PeekableTokenTree::new(quote::quote! { test }));
    let ident = it.next().unwrap();
    let is_raw_tokenstream = false;
    match ident {
        TokenTree::Ident(ident) => {
            match converter.convert_html_ident_to_rusthtmltoken(&ident, tag_ctx, &mut output, it, is_raw_tokenstream) {
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
    let parse_ctx = Rc::new(HtmlTagParseContext::new(None));
    parse_ctx.set_parse_attrs(true);
    let is_raw_tokenstream = false;
    
    converter.convert_html_literal_to_rusthtmltoken(&literal, parse_ctx, &mut output, is_raw_tokenstream).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_convert_html_punct_to_rusthtmltoken() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let punct = Punct::new('_', Spacing::Alone);
    let mut output = vec![];
    let parse_ctx = Rc::new(HtmlTagParseContext::new(None));
    let is_raw_tokenstream = false;
    let it = Rc::new(PeekableTokenTree::new(TokenStream::new()));
    converter.convert_html_punct_to_rusthtmltoken(&punct, parse_ctx, &mut output, it, is_raw_tokenstream).unwrap();
}

#[test]
#[should_panic]
pub fn rust_to_rusthtml_converter_on_kvp_defined_when_empty_panics() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let ctx = Rc::new(HtmlTagParseContext::new(None));
    let mut output = vec![];
    converter.on_kvp_defined(ctx, &mut output).unwrap();
}

#[test]
pub fn rust_to_rusthtml_converter_on_kvp_defined_when_not_empty_works() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let ctx = Rc::new(HtmlTagParseContext::new(None));
    ctx.html_attr_key_push_str("test");
    ctx.html_attr_key_ident_push(&Ident::new("test", Span::call_site()));
    ctx.set_equals_punct(&Punct::new('=', Spacing::Alone));
    ctx.set_html_attr_val_literal(&Literal::string("test"));
    let mut output = vec![];
    converter.on_kvp_defined(ctx, &mut output).unwrap();
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
    let parse_ctx = Rc::new(HtmlTagParseContext::new(None));
    let mut output = vec![];
    let result = converter.on_html_tag_parsed(Some(&punct), parse_ctx, &mut output).unwrap();
    assert_eq!(true, result);
}

#[test]
pub fn rust_to_rusthtml_converter_on_html_node_parsed() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let ctx = Rc::new(HtmlTagParseContext::new(None));
    let mut output = vec![];
    let result = converter.on_html_node_parsed(ctx, &mut output).unwrap();
    assert_eq!(true, result);
}

#[test]
pub fn rust_to_rusthtml_converter_on_html_node_parsed_when_not_empty() {
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let ctx = Rc::new(HtmlTagParseContext::new(None));

    let mut output = vec![];

    ctx.tag_name_push_ident(&Ident::new("environment", Span::call_site()));
    ctx.on_html_tag_name_parsed(&mut output);
    assert_eq!(1, output.len());

    ctx.html_attr_key_ident_push(&Ident::new("include", Span::call_site()));
    ctx.set_equals_punct(&Punct::new('=', Spacing::Alone));
    ctx.set_html_attr_val_literal(&Literal::string("test"));
    let r = ctx.on_kvp_defined().unwrap();
    assert_eq!(true, r.len() > 0);
    match ctx.get_html_attr("include") {
        Some(v) => {
            match v {
                RustHtmlToken::HtmlTagAttributeValue(s, literal, _tokens, _tokens2) => {
                    let s = s.unwrap_or(literal.map(|x| x.to_string()).unwrap_or_default());
                    assert_eq!("\"test\"", s.as_str());
                },
                _ => {
                    panic!("expected RustHtmlIdentAndPunctOrLiteral::Literal, found: {:?}", v);
                }
            }
        },
        None => {
            panic!("expected Some");
        }
    }
    assert_eq!(1, ctx.get_html_attrs().len());

    let r2 = converter.on_html_tag_parsed(ctx.get_equals_punct().as_ref(), ctx.clone(), &mut output).unwrap();
    assert_eq!(true, r2);
    assert_eq!(2, output.len());
    assert_eq!(1, ctx.get_html_attrs().len());

    // the context loses context so the attrs are lost and don't work as expected,
    // like for the environment node helper that depends on include and exclude attrs.
    let result = converter.on_html_node_parsed(ctx.clone(), &mut output).unwrap();
    assert_eq!(true, result);
    assert_eq!(1, ctx.get_html_attrs().len());
    assert_eq!(3, output.len());
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
// make new test similar to above but tests the output for
// left: `"<divclass=\"container\"><divclass=\"row\"><divclass=\"col-md-12\"><h1>Hello,world!</h1></div></div></div>"`,
// right: `"<divclass=container><divclass=row><divclass=col-md-12><h1>Hello,world!</h1></div></div></div>"`', mvc_lib/tests/view/rusthtml/rusthtml_token_tests.rs:84:5
// since there is a problem with the converter removing quotes from strings
pub fn rust_to_rusthtml_converter_handles_strings_correctly() {
    let html_stream = quote::quote! {
        <div class="container">
            <div class="row">
                <div class="col-md-12">
                    <h1>Hello, world!</h1>
                </div>
            </div>
        </div>
    };
    let html_stream = Rc::new(PeekableTokenTree::new(html_stream));
    let converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let actual_tokens = converter.parse_tokenstream_to_rusthtmltokens(false, html_stream, false).unwrap();
    let actual_string = actual_tokens.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    
    assert!(actual_string.contains("class=\"container\""));
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
            None, None, None, Some(vec![
                RustHtmlToken::Identifier(Ident::new("html_class", Span::call_site())),
            ])
        ),
        RustHtmlToken::HtmlTagCloseStartChildrenPunct,
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
            (RustHtmlToken::HtmlTagAttributeValue(expected_string, expected_literal, expected_idents, expected_rust), RustHtmlToken::HtmlTagAttributeValue(actual_string, actual_literal, actual_idents, actual_rust)) => {
                let expected = if let Some(s) = expected_string { s.clone() } else if let Some(s) = expected_literal { s.to_string() } else if let Some(s) = expected_idents { s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("").to_string() } else { expected_rust.as_ref().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("") };
                let actual = if let Some(s) = actual_string { s.clone() } else if let Some(s) = actual_literal { s.to_string() } else if let Some(s) = actual_idents { s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("").to_string() } else { actual_rust.as_ref().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("") };

                assert_eq!(expected, actual);
                
                // if let Some(expected_literal) = expected_literal {
                //     assert_eq!(expected_literal.to_string(), actual_literal.as_ref().unwrap().to_string());
                // } else if let Some(_) = expected_idents {
                //     // need new function similar to existing one
                //     assert_vecs_of_rusthtmlidentsorpunct_eq(expected_idents.as_ref().unwrap(), actual_idents.as_ref().unwrap());
                // } else if let Some(expected_rust) = expected_rust {
                //     compare_rusthtmltokens(expected_rust, actual_rust.as_ref().unwrap_or(&vec![]));
                // } else {
                    // panic!("expected and actual are not the same or not supported: expected: {:?}, actual: {:?}", expected, actual);
                // }
            },
            (RustHtmlToken::HtmlTagCloseStartChildrenPunct, RustHtmlToken::HtmlTagCloseStartChildrenPunct) => {
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

    // need to do assertion that outputs differences in the two vectors
    if expected_output.len() != output.len() {
        panic!("expected and actual are not the same length, expected: {:?}, actual: {:?}", expected_output, output);
    }
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



// add test for 'a' element with classes and href="/" since the current issue
// is that the href is not being parsed correctly: the key for some reason is
// not being stored in the context, either from being skipped or something else is wrong.
#[test]
fn rust_to_rusthtml_converter_parse_complex_html() {
    let parse_context = RustHtmlParserContext::new(false, false, "test".to_string());
    let converter = RustToRustHtmlConverter::new(Rc::new(parse_context));
    let output = converter.parse_tokenstream_to_rusthtmltokens(
        true,
        Rc::new(PeekableTokenTree::new(quote::quote! {
            <a class="nav-link" href="/">Home</a>
        })),
        false
    ).unwrap();
    
    let expected_output = vec![
        RustHtmlToken::HtmlTagStart("a".to_string(), Some(vec![RustHtmlIdentOrPunct::Ident(Ident::new("a", Span::call_site()))])),
        RustHtmlToken::HtmlTagAttributeName("class".to_string(), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(vec![RustHtmlIdentOrPunct::Ident(Ident::new("class", Span::call_site()))]))),
        RustHtmlToken::HtmlTagAttributeEquals('=', Some(Punct::new('=', Spacing::Alone))),
        RustHtmlToken::HtmlTagAttributeValue(
            None, None, None, Some(vec![
                RustHtmlToken::Literal(Some(Literal::string("nav-link")), None),
            ])
        ),
        RustHtmlToken::HtmlTagAttributeName("href".to_string(), Some(RustHtmlIdentAndPunctOrLiteral::IdentAndPunct(vec![RustHtmlIdentOrPunct::Ident(Ident::new("href", Span::call_site()))]))),
        RustHtmlToken::HtmlTagAttributeEquals('=', Some(Punct::new('=', Spacing::Alone))),
        RustHtmlToken::HtmlTagAttributeValue(
            None, None, None, Some(vec![
                RustHtmlToken::Literal(Some(Literal::string("/")), None),
            ])
        ),
        RustHtmlToken::HtmlTagCloseStartChildrenPunct,
        RustHtmlToken::HtmlTextNode("Home".to_string(), Span::call_site()),
        RustHtmlToken::HtmlTagEnd("a".to_string(), Some(vec![RustHtmlIdentOrPunct::Ident(Ident::new("a", Span::call_site()))])),
    ];

    compare_rusthtmltokens(&expected_output, &output);
}

// test part of rust_to_rusthtml_converter_parse_complex_html to assert middle parse state since the 2nd attribute seems to clear out the first attribute.
#[test]
fn test_on_kvp_defined() {
    let ctx = HtmlTagParseContext::new(None);
    ctx.html_attr_key_push_str("testkey");
    ctx.html_attr_key_ident_push(&Ident::new("testkey", Span::call_site()));
    ctx.set_equals_punct(&Punct::new('=', Spacing::Alone));
    ctx.set_html_attr_val_literal(&Literal::string("testval"));
    let _converter = RustToRustHtmlConverter::new(Rc::new(RustHtmlParserContext::new(false, false, "test".to_string())));
    let (_actual_key_token, actual_key) = ctx.create_key_for_kvp().unwrap();
    let (_actual_val_token, actual_val) = ctx.create_val_for_kvp(actual_key.clone()).unwrap().unwrap();
    
    // assert
    // assert_eq!(true, output.len() > 0);
    assert_eq!("testkey", actual_key);
    assert_eq!("\"testval\"", actual_val);
}

// test half of rust_to_rusthtml_converter_parse_complex_html to assert middle parse state since the 2nd attribute seems to clear out the first attribute.
// for some reason the below passes, but the HTML attribute keys are not working properly
#[test]
pub fn test_rust_to_rusthtml_converter_parse_complex_html_inner() {
    let real_ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parse_context = Rc::new(RustHtmlParserContextLog::new(real_ctx));
    let converter = RustToRustHtmlConverter::new(parse_context.clone());
    let output = converter.parse_tokenstream_to_rusthtmltokens(
        true,
        Rc::new(PeekableTokenTree::new(quote::quote! {
            <a class="nav-link" href="/">Home</a>
        })),
        false
    ).unwrap();
    assert_eq!(true, output.len() > 0);
    // output not used, just testing that the parse context is being called correctly

    // ooo = order of operations (method calls, variable assignments, etc.)
    let expected_ooo = vec![
        "convert_punct_to_rusthtmltoken: <",
        "convert_html_ident_to_rusthtmltoken(a)",
        "convert_html_ident_to_rusthtmltoken(class)",
        "convert_html_punct_to_rusthtmltoken(=)",
        "convert_html_literal_to_rusthtmltoken(\"nav-link\")",
        "convert_html_ident_to_rusthtmltoken(href)",
        "convert_html_punct_to_rusthtmltoken(=)",
        "convert_html_literal_to_rusthtmltoken(\"/\")",
        "convert_html_punct_to_rusthtmltoken(>)",
        "get_tag_parsed_handler",
        "htmltag_scope_stack_push(a)",
        "convert_tokentree_to_rusthtmltoken: Ident(Home)",
        "convert_punct_to_rusthtmltoken: <",
        "convert_html_ident_to_rusthtmltoken(a)",
        "convert_html_punct_to_rusthtmltoken(>)",
        "get_tag_parsed_handler",
        "htmltag_scope_stack_pop(test)",
        "get_node_parsed_handler",
    ];
    let expected_ooo_string = expected_ooo.join("\n");
    let actual_ooo = parse_context.as_ref().get_ooo();
    let actual_ooo_string = actual_ooo.join("\n");
    assert_str_eq!(expected_ooo_string, actual_ooo_string);    
}

// new test that steps through the above and checks the keys in the context
#[test]
pub fn test_html_attr_key_parsing() {
    let real_ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parse_context = Rc::new(RustHtmlParserContextLog::new(real_ctx.clone()));

    let ctx = Rc::new(HtmlTagParseContext::new(Some(real_ctx.clone())));
    let mut output = vec![];
    let converter = RustToRustHtmlConverter::new(parse_context.clone());
    let it = Rc::new(PeekableTokenTree::new(quote::quote! {
        <a class="nav-link" href="/">Home</a>
    }));

    // assert each token in the stream
    // my_assert_punct('<', &mut output, &converter, &ctx, it.clone());
    it.next(); // skip first
    my_assert_ident("a", &mut output, &converter, ctx.clone(), it.clone());
    my_assert_ident("class", &mut output, &converter, ctx.clone(), it.clone());
    my_assert_punct('=', &mut output, &converter, ctx.clone(), it.clone());
}

fn my_assert_ident(s: &str, output: &mut Vec<RustHtmlToken>, converter: &dyn IRustToRustHtmlConverter, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<PeekableTokenTree>) {
    let token_option = it.next().unwrap();
    let _ident = core_lib::assert::assert_tokentree::assert_tokentree_ident(&token_option, s);
    match converter.next_and_parse_html_tag(&token_option, ctx, output, it.clone(), false) {
        Ok(x) => assert_eq!(false, x),
        Err(RustHtmlError(e)) => {
            panic!("expected Some: {}", e);
        }
    }
}

fn my_assert_punct(c: char, output: &mut Vec<RustHtmlToken>, converter: &dyn IRustToRustHtmlConverter, ctx: Rc<dyn IHtmlTagParseContext>, it: Rc<PeekableTokenTree>) {
    let token_option = it.next().unwrap();
    let _punct = core_lib::assert::assert_tokentree::assert_tokentree_punct(&token_option, c);
    match converter.next_and_parse_html_tag(&token_option, ctx, output, it.clone(), false) {
        Ok(_x) => {},
        Err(RustHtmlError(e)) => {
            panic!("expected Some: {}", e);
        }
    }
}