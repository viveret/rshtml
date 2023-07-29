use std::rc::Rc;

use mvc_lib::view::rusthtml::peekable_tokentree::PeekableTokenTree;
use mvc_lib::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlToken;
use proc_macro2::Span;



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
    let it = Rc::new(PeekableTokenTree::new(stream.clone()));
    let converter_context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let converter = RustToRustHtmlConverter::new(converter_context);
    let output = converter.parse_tokenstream_to_rusthtmltokens(true, it, false).unwrap();
    let output_string = output.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("");
    assert_eq!(stream.to_string().replace(" ", ""), output_string);
}