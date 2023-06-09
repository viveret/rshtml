use mvc_lib::view::rusthtml::rusthtml_token::RustHtmlIdentOrPunct;
use mvc_lib::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use proc_macro2::Ident;
use proc_macro2::Span;



#[test]
fn html_tag_parse_context_is_void_tag_false_by_default() {
    let ctx = HtmlTagParseContext::new();
    assert_eq!(false, ctx.is_void_tag());
}


#[test]
fn html_tag_parse_context_is_void_tag_true_works() {
    let mut output = vec![];
    let mut ctx = HtmlTagParseContext::new();
    ctx.tag_name.push(RustHtmlIdentOrPunct::Ident(Ident::new("br", Span::call_site())));
    ctx.on_html_tag_name_parsed(&mut output);
    assert_eq!(true, ctx.is_void_tag());
}


#[test]
fn html_tag_parse_context_clear_attr_kvp_works() {
    let mut ctx = HtmlTagParseContext::new();
    ctx.parse_attr_val = true;

    assert_eq!(true, ctx.parse_attr_val);
    assert_eq!(true, ctx.equals_punct.is_none());
    assert_eq!(0, ctx.html_attr_val_ident.len());
    assert_eq!(0, ctx.html_attr_val_rust.len());
    assert_eq!(0, ctx.html_attr_key_ident.len());
    assert_eq!(0, ctx.html_attr_key.len());
    ctx.clear_attr_kvp();
    assert_eq!(false, ctx.parse_attr_val);
}