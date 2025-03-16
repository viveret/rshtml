use std::rc::Rc;

use mvc_lib::view::parserv3::contexts::ihtml_tag_parse_context::IHtmlTagParseContext;
use mvc_lib::view::parserv3::contexts::rusthtml_parser_context::MockRustHtmlParserContext;
use mvc_lib::view::parserv3::contexts::html_tag_parse_context::HtmlTagParseContext;
use mvc_lib::view::rusthtml::rusthtml_token::{RustHtmlIdentOrPunct, RustHtmlToken};
use proc_macro2::Punct;




#[test]
fn html_tag_parse_context_test_initialization() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context = HtmlTagParseContext::new(Some(mock_context.clone()));

    // Assert
    assert!(context.main_context.is_some(), "Expected main_context to be set");
    assert!(context.tag_name.borrow().is_empty(), "Expected tag_name to be empty");
    assert!(context.html_attrs.borrow().is_empty(), "Expected html_attrs to be empty");
    assert!(!context.is_self_contained_tag(), "Expected is_self_contained_tag to be false");
    assert!(context.is_opening_tag(), "Expected is_opening_tag to be true");
    assert!(!context.is_explicit_void_tag(), "Expected is_explicit_void_tag to be false");
    assert!(context.tag_end_punct.borrow().is_empty(), "Expected tag_end_punct to be empty");
    assert!(context.get_add_inner(), "Expected add_inner to be true");
}

#[test]
fn html_tag_parse_context_test_tag_name_parsing() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context = HtmlTagParseContext::new(Some(mock_context.clone()));

    // Execute
    let tag_name = vec![RustHtmlIdentOrPunct::Ident(proc_macro2::Ident::new("div", proc_macro2::Span::call_site()))];
    let result = context.on_html_tag_name_parsed(tag_name.clone());

    // Assert
    assert!(result.is_ok(), "Expected on_html_tag_name_parsed to return Ok");
    assert_eq!(context.tag_name_as_str(), "div", "Expected tag_name_as_str to return 'div'");
    assert!(context.has_tag_name(), "Expected has_tag_name to return true");

    for (i, x) in context.get_tag_name().iter().enumerate() {
        assert_eq!(x, &tag_name[i], "Expected get_tag_name to return the correct tag name");
    }
}

#[test]
fn html_tag_parse_context_test_void_tag_handling() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context = HtmlTagParseContext::new(Some(mock_context.clone()));

    // Test built-in void tags
    let void_tags = vec!["input", "hr", "br", "!DOCTYPE"];
    for tag in void_tags {
        let tag_name = vec![RustHtmlIdentOrPunct::Ident(proc_macro2::Ident::new(tag, proc_macro2::Span::call_site()))];
        context.on_html_tag_name_parsed(tag_name).unwrap();
        assert!(context.is_void_tag(), "Expected {} to be a void tag", tag);
    }

    // Test explicit void tag setting
    context.set_is_void_tag(true);
    assert!(context.is_void_tag(), "Expected is_void_tag to return true after setting");
}

#[test]
fn html_tag_parse_context_test_html_attributes() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context = HtmlTagParseContext::new(Some(mock_context.clone()));

    // Execute
    let key = "class".to_string();
    let value_token = RustHtmlToken::HtmlTagAttributeValue(Some("container".to_string()), None, None, None);
    let value = Some("container".to_string());
    context.html_attrs_insert(None, None, key.clone(), None, None, value.clone(), None, None, None);

    // Assert
    assert_eq!(context.get_html_attrs().len(), 1, "Expected get_html_attrs to return 1 attribute");

    let output_attr = context.html_attrs_get(&key).flatten().unwrap();
    assert_eq!(output_attr.len(), 1, "Expected output_attr.len() to return 1");
    let output_attr = output_attr.first().unwrap();
    assert_eq!(output_attr, &value_token, "Expected html_attrs_get to return the inserted value");

    let output_attr = context.get_html_attr(&key).unwrap();
    assert_eq!(output_attr.len(), 1, "Expected output_attr.len() to return 1");
    let output_attr = output_attr.first().unwrap();

    assert_eq!(output_attr, &value_token, "Expected get_html_attr to return the inserted value");

}

fn compare_punct(a: &Punct, b: &Punct) -> bool {
    a.as_char() == b.as_char() && a.spacing() == b.spacing()
}

#[test]
fn html_tag_parse_context_test_tag_end_punct() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context = HtmlTagParseContext::new(Some(mock_context.clone()));

    // Execute
    let punct = proc_macro2::Punct::new('>', proc_macro2::Spacing::Alone);
    context.add_tag_end_punct(&punct);

    // Assert
    assert!(compare_punct(&context.get_tag_end_punct().expect("Expected get_tag_end_punct to return Some"), &punct));
}


#[test]
fn html_tag_parse_context_test_add_inner() {
    // Setup
    let mock_context = Rc::new(MockRustHtmlParserContext::new());
    let context = HtmlTagParseContext::new(Some(mock_context.clone()));

    // Execute
    context.set_add_inner(false);

    // Assert
    assert!(!context.get_add_inner(), "Expected add_inner to be false after setting");
}