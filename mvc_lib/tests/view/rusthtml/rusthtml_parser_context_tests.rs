use mvc_lib::view::rusthtml::rusthtml_parser_context::{RustHtmlParserContext, IRustHtmlParserContext};



#[test]
fn rusthtml_parser_context_constructor_works() {
    let ctx = RustHtmlParserContext::new(false, false, "test".to_string());
    assert_eq!(true, ctx.is_ok());
}

// each test can be described by passing some data and seeing if the context
// "picks up" or correctly parses what we are trying to do.
// for example:
#[test]
fn rusthtml_parser_context_set_model_type_works() {
    let ctx = RustHtmlParserContext::new(false, false, "test".to_string());
    let mt = vec![];
    ctx.set_model_type(Some(mt.clone()));
    let returned = ctx.get_model_type();
    assert_eq!(mt.len(), returned.len());
    // compare arrays
}