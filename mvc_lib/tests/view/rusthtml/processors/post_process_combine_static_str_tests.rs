use mvc_lib::view::rusthtml::irust_processor::IRustProcessor;
use mvc_lib::view::rusthtml::processors::post_process_combine_static_str::PostProcessCombineStaticStr;
use proc_macro2::{TokenTree, TokenStream};


#[test]
pub fn post_process_combine_static_str_process_rust_basic() {
    let processor = PostProcessCombineStaticStr::new();
    let rusthtml = quote::quote! {
        html_output.write_html_str("Hello, world!");
        html_output.write_html_str("Hello, world!");
        html_output.write_html_str("Hello, world!");
    };
    let rusthtml_expected = quote::quote! {
        html_output.write_html_str("Hello, world!Hello, world!Hello, world!");
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let result = processor.process_rust(&rusthtml.into_iter().collect::<Vec<TokenTree>>()).unwrap();
    assert_ne!(0, result.len());

    let rusthtml_actual = TokenStream::from_iter(result.into_iter()).to_string();
    assert_eq!(rusthtml_expected_string, rusthtml_actual);
}


#[test]
pub fn post_process_combine_static_str_process_rust_split() {
    let processor = PostProcessCombineStaticStr::new();
    let rusthtml = quote::quote! {
        html_output.write_html_str("Hello, world!");
        html_output.write_html_str("Hello, world!");
        something.do_another_thing();
        html_output.write_html_str("Hello, world!");
        html_output.write_html_str("Hello, world!");
    };
    let rusthtml_expected = quote::quote! {
        html_output.write_html_str("Hello, world!Hello, world!");
        something.do_another_thing();
        html_output.write_html_str("Hello, world!Hello, world!");
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let result = processor.process_rust(&rusthtml.into_iter().collect::<Vec<TokenTree>>()).unwrap();
    assert_ne!(0, result.len());

    let rusthtml_actual = TokenStream::from_iter(result.into_iter()).to_string();
    assert_eq!(rusthtml_expected_string, rusthtml_actual);
}


#[test]
pub fn post_process_combine_static_str_process_rust_nested_basic() {
    let processor = PostProcessCombineStaticStr::new();
    let rusthtml = quote::quote! {
        fn foobar() {
            html_output.write_html_str("Hello, world!");
            html_output.write_html_str("Hello, world!");
            html_output.write_html_str("Hello, world!");
        }
    };
    let rusthtml_expected = quote::quote! {
        fn foobar() {
            html_output.write_html_str("Hello, world!Hello, world!Hello, world!");
        }
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let result = processor.process_rust(&rusthtml.into_iter().collect::<Vec<TokenTree>>()).unwrap();
    assert_ne!(0, result.len());

    let rusthtml_actual = TokenStream::from_iter(result.into_iter()).to_string();
    assert_eq!(rusthtml_expected_string, rusthtml_actual);
}


#[test]
pub fn post_process_combine_static_str_process_rust_nested_split() {
    let processor = PostProcessCombineStaticStr::new();
    let rusthtml = quote::quote! {
        fn foobar() {
            html_output.write_html_str("Hello, world!");
            html_output.write_html_str("Hello, world!");
            something.do_another_thing();
            html_output.write_html_str("Hello, world!");
            html_output.write_html_str("Hello, world!");
        }
    };
    let rusthtml_expected = quote::quote! {
        fn foobar() {
            html_output.write_html_str("Hello, world!Hello, world!");
            something.do_another_thing();
            html_output.write_html_str("Hello, world!Hello, world!");
        }
    };
    let rusthtml_expected_string = rusthtml_expected.to_string();

    let result = processor.process_rust(&rusthtml.into_iter().collect::<Vec<TokenTree>>()).unwrap();
    assert_ne!(0, result.len());

    let rusthtml_actual = TokenStream::from_iter(result.into_iter()).to_string();
    assert_eq!(rusthtml_expected_string, rusthtml_actual);
}