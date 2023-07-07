use std::rc::Rc;

use mvc_lib::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use mvc_lib::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use mvc_lib::view::rusthtml::directives::irusthtml_directive::IRustHtmlDirective;
use mvc_lib::view::rusthtml::directives::if_directive::IfDirective;
use proc_macro2::{TokenTree, TokenStream};



#[test]
pub fn if_directive_process_rust_basic() {
    let processor = IfDirective::new();
    let rusthtml = quote::quote! {
        @if true {
            <div>@"Hello, world!"</div>
        }
    };
    let rusthtml_expected = quote::quote! {
        if true {
            <div>"Hello, world!"</div>
        }
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let it = Rc::new(PeekableTokenTree::new(rusthtml)) as Rc<dyn IPeekableTokenTree>;
    // skip the '@' and 'if' tokens
    it.as_ref().next();

    let first_token = it.next().unwrap();
    let first_ident = if let TokenTree::Ident(x) = first_token { x } else { panic!("expected ident, not {:?}", first_token); };

    let context = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let parser = Rc::new(RustToRustHtmlConverter::new(context));

    // begin processing
    let mut output = Vec::new();
    let result = processor.execute(&first_ident, parser, &mut output, it).unwrap();
    assert_ne!(0, output.len());
    match result {
        RustHtmlDirectiveResult::OkContinue => {
            let rusthtml_actual = output.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("");
            assert_eq!(rusthtml_expected_string, rusthtml_actual);
        },
        _ => panic!("expected OkContinue, not {:?}", result)
    }
}