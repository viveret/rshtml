use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll};
use proc_macro2::Span;

use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;



#[test]
fn rusthtml_token_to_string_space() {
    let token = RustHtmlToken::Space(' ');
    let token_string = token.to_string();
    assert_eq!(" ", token_string);
}

#[test]
fn rusthtml_token_to_string_html_text_node() {
    let token = RustHtmlToken::HtmlTextNode("Hello, world!".to_string(), Span::call_site());
    let token_string = token.to_string();
    assert_eq!("Hello, world!", token_string);
}

#[test]
fn rusthtml_token_to_string_html_tag_void() {
    let token = RustHtmlToken::HtmlTagVoid("div".to_string(), None);
    let token_string = token.to_string();
    assert_eq!("<div />", token_string);
}

#[test]
fn rusthtml_token_to_string_html_tag_start() {
    let token = RustHtmlToken::HtmlTagStart("div".to_string(), None);
    let token_string = token.to_string();
    assert_eq!("<div", token_string);
}

#[test]
fn rusthtml_token_to_string_html_tag_end() {
    let token = RustHtmlToken::HtmlTagEnd("div".to_string(), None);
    let token_string = token.to_string();
    assert_eq!("</div>", token_string);
}

#[test]
fn rusthtml_token_to_string_html_tag_attribute_name() {
    let token = RustHtmlToken::HtmlTagAttributeName("class".to_string(), None);
    let token_string = token.to_string();
    assert_eq!("class", token_string);
}

#[test]
fn rusthtml_token_to_string_html_tag_attribute_equals() {
    let token = RustHtmlToken::HtmlTagAttributeEquals('=', None);
    let token_string = token.to_string();
    assert_eq!("=", token_string);
}

#[test]
fn rusthtml_token_to_string_html_tag_attribute_value() {
    let token = RustHtmlToken::HtmlTagAttributeValue(Some("\"container\"".to_string()), None, None, None);
    let token_string = token.to_string();
    assert_eq!("\"container\"", token_string);
}

#[test]
fn rusthtml_token_to_string_spacing() {
    let stream = quote::quote! {
        <div class="container">
            <div class="row">
                <div class="col-md-12">
                    <h1>Hello, world!</h1>
                </div>
            </div>
        </div>
    };
    let parser = RustHtmlParserAll::new_default();
    
    let ct = Rc::new(CancellationToken::new());
    let output = parser.expand_rust(stream.clone(), ct).unwrap();
    let output_string = output.to_string();
    assert_eq!(stream.to_string().replace(" ", ""), output_string);
}