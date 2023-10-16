use std::rc::Rc;
use core_lib::asyncly::cancellation_token::CancellationToken;
use proc_macro2::{Literal, Ident};

use assert_str::assert_str_eq;

use mvc_lib::view::rusthtml::rusthtml_parser::RustHtmlParser;
use mvc_lib::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use mvc_lib::view::rusthtml::node_helpers::inode_parsed::IHtmlNodeParsed;
use mvc_lib::view::rusthtml::node_helpers::environment_node::EnvironmentHtmlNodeParsed;


#[test]
pub fn test_environment_node_new() {
    let node = EnvironmentHtmlNodeParsed::new();
    assert_eq!(true, node.matches("environment"));
}

#[test]
pub fn test_environment_node_on_node_parsed_empty_error() {
    let node = EnvironmentHtmlNodeParsed::new();
    let html_ctx = Rc::new(RustHtmlParserContext::new(false, false, "Test".to_string()));
    let tag_ctx = Rc::new(HtmlTagParseContext::new(Some(html_ctx.clone())));
    let mut output = vec![];
    let result = node.on_node_parsed(
        tag_ctx,
        html_ctx,
        &mut output
    );

    match result {
        Ok(_) => {
            panic!("Expected error");
        },
        Err(RustHtmlError(e)) => {
            assert_str_eq!("rust html tag environment expects attribute 'include' or 'exclude' to be defined (attrs: {})", e);
        }
    }
}

#[test]
pub fn test_environment_node_on_node_parsed_include() {
    let node = EnvironmentHtmlNodeParsed::new();
    let html_ctx = Rc::new(RustHtmlParserContext::new(false, true, "Test".to_string()));
    let tag_ctx = Rc::new(HtmlTagParseContext::new(Some(html_ctx.clone())));

    tag_ctx.html_attr_key_push_str("include");
    tag_ctx.html_attr_key_ident_push(&Ident::new("include", proc_macro2::Span::call_site()));
    tag_ctx.set_html_attr_val_literal(&Literal::string("test"));
    tag_ctx.on_kvp_defined().unwrap();

    let mut output = vec![];
    let result = node.on_node_parsed(
        tag_ctx,
        html_ctx,
        &mut output
    );

    match result {
        Ok(x) => {
            assert_eq!(false, x);
            assert_eq!(0, output.len());
        },
        Err(RustHtmlError(e)) => {
            panic!("Unexpected error: {}", e);
        }
    }
}

// try with HTML tokens from rust stream
#[test]
pub fn test_environment_node_on_node_parsed_include_html_tokens() {
    let input = quote::quote! {
        @viewstart ""
        <environment include="test">
            <div></div>
        </environment>
    };

    let parser = RustHtmlParser::new(true, "test".to_string());
    let ct = Rc::new(CancellationToken::new());
    let output = parser.expand_tokenstream(input, ct).unwrap();

    let expected_output = quote::quote! {
        html_output . write_html_str ("<div></div>");
    };

    assert_eq!(expected_output.to_string(), output.to_string());
}

#[test]
pub fn test_environment_node_on_node_parsed_exclude_html_tokens() {
    let input = quote::quote! {
        @viewstart ""
        <environment exclude="test">
            <div></div>
        </environment>
    };

    let parser = RustHtmlParser::new(true, "test".to_string());
    let ct = Rc::new(CancellationToken::new());
    let output = parser.expand_tokenstream(input, ct).unwrap();

    let expected_output = quote::quote! {
    };

    assert_eq!(expected_output.to_string(), output.to_string());
}