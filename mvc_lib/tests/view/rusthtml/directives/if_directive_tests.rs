use mvc_lib::view::rusthtml::directives::if_directive::IfDirective;
use proc_macro2::{TokenStream, TokenTree};



#[test]
pub fn if_directive_process_rust_basic() {
    let processor = IfDirective::new();
    let rusthtml = quote::quote! {
        if true {
            html_output.write_html_str("Hello, world!");
        }
    };
    let rusthtml_expected = quote::quote! {
        if true {
            html_output.write_html_str("Hello, world!");
        }
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let result = processor.process_tokenstream(&rusthtml.into_iter().collect::<Vec<TokenTree>>()).unwrap();
    assert_ne!(0, result.len());

    let rusthtml_actual = TokenStream::from_iter(result.into_iter()).to_string();
    assert_eq!(rusthtml_expected_string, rusthtml_actual);
}