use mvc_lib::view::rusthtml::{irust_processor::IRustProcessor, peekable_tokentree::PeekableTokenTree};
use mvc_lib::view::rusthtml::processors::post_process_combine_static_str::PostProcessCombineStaticStr;
use proc_macro2::{TokenTree, TokenStream, Punct, Group, Delimiter, Spacing, Literal, Ident};

// test individual parts of the processor
#[test]
pub fn is_ident_with_name_true() {
    let mut output = Vec::<TokenTree>::new();
    let input = vec![
        TokenTree::Ident(Ident::new("foobar", proc_macro2::Span::call_site())),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_ident_with_name(true, "foobar", &mut output, &mut it);

    assert!(result);
    assert_eq!(1, output.len());
}

#[test]
pub fn is_ident_with_name_false() {
    let mut output = Vec::<TokenTree>::new();
    let input = vec![
        TokenTree::Ident(Ident::new("foobar1", proc_macro2::Span::call_site())),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_ident_with_name(true, "foobar", &mut output, &mut it);

    assert!(!result);
    assert_eq!(0, output.len());
}

#[test]
pub fn post_process_combine_static_str_try_group() {
    let input = vec![
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(vec![ TokenTree::Ident(Ident::new("foobar", proc_macro2::Span::call_site())) ])
        )),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::try_group(Delimiter::Parenthesis, &mut it).unwrap();

    assert_eq!(1, result.stream().into_iter().collect::<Vec<TokenTree>>().len());
}

#[test]
pub fn post_process_combine_static_str_is_group_with_string_literal_arg_some() {
    let input = vec![
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(vec![ TokenTree::Literal(Literal::string("Hello, world!")) ])
        )),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_group_with_string_literal_arg(Delimiter::Parenthesis, &mut it).unwrap();

    assert_eq!("Hello, world!", result);
}

#[test]
pub fn post_process_combine_static_str_is_group_with_string_literal_arg_none() {
    let input = vec![
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(Vec::<TokenTree>::new())
        )),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_group_with_string_literal_arg(Delimiter::Parenthesis, &mut it);

    assert!(result.is_none());
}

#[test]
pub fn post_process_combine_static_str_is_string_literal() {
    let input = quote::quote! {
        "Hello, world!"
    };
    let result = PostProcessCombineStaticStr::is_string_literal(PeekableTokenTree::new(input)).unwrap();

    assert_eq!("Hello, world!", result);
}

#[test]
pub fn post_process_combine_static_str_is_html_output() {
    let mut is_first = true;
    let mut output = Vec::<TokenTree>::new();
    let input = vec![
        TokenTree::Ident(Ident::new("html_output", proc_macro2::Span::call_site())),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_html_output(is_first, &mut output, &mut it);

    assert!(result);
    assert_eq!(1, output.len());


    // test when is_first is false
    is_first = false;
    output.clear();
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_html_output(is_first, &mut output, &mut it);

    assert!(result);
    assert_eq!(0, output.len());

    
    // test when next token is not html_output
    is_first = true;
    output.clear();
    let input = vec![
        TokenTree::Ident(Ident::new("foobar", proc_macro2::Span::call_site())),
    ];
    let mut it = input.iter().peekable();
    let result = PostProcessCombineStaticStr::is_html_output(is_first, &mut output, &mut it);

    assert!(!result);
    assert_eq!(0, output.len());
}

pub fn test_is_html_output_write_html_str_with_string_literal_arg_and_semicolon_for_n(n: usize) {
    let mut is_first = true;
    let mut current_str = String::new();
    let mut output = Vec::<TokenTree>::new();
    let input = vec![
        TokenTree::Ident(Ident::new("html_output", proc_macro2::Span::call_site())),
        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
        TokenTree::Ident(Ident::new("write_html_str", proc_macro2::Span::call_site())),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(vec![ TokenTree::Literal(Literal::string("Hello, world!")) ])
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ];
    let input_len = input.len();
    // multiply input
    let input = if n > 1 { input.into_iter().cycle().take(input_len * n).collect::<Vec<TokenTree>>() } else { input };
    let mut it = input.iter().peekable();
    loop {
        if PostProcessCombineStaticStr::is_html_output_write_html_str_with_string_literal_arg_and_semicolon(
            &mut is_first,
            &mut current_str,
            &mut output,
            &mut it
        ) {
            // do nothing
        } else {
            break;
        }
    }

    // compare string content
    let expected_str = "html_output . write_html_str";
    let actual_str = TokenStream::from_iter(output.clone().into_iter()).to_string();
    assert_eq!(expected_str, actual_str);
    assert_eq!(3, output.len());

    let hello_world_str = "Hello, world!";
    let s_len = hello_world_str.len();
    let s: String = hello_world_str.chars().cycle().take(s_len * n).collect();
    assert_eq!(s, current_str);
}

// #[test]
// pub fn punct_are_same_different_spacing() {
//     let a = Punct::new('.', Spacing::Alone);
//     let b = Punct::new('.', Spacing::Joint);
//     assert_eq!(a, b);
// }

#[test]
pub fn is_html_output_write_html_str_with_string_literal_arg_and_semicolon_single_works() {
    test_is_html_output_write_html_str_with_string_literal_arg_and_semicolon_for_n(1);
}


#[test]
pub fn is_html_output_write_html_str_with_string_literal_arg_and_semicolon_multiple_works() {
    test_is_html_output_write_html_str_with_string_literal_arg_and_semicolon_for_n(3);
    test_is_html_output_write_html_str_with_string_literal_arg_and_semicolon_for_n(9);
}


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
pub fn post_process_combine_static_str_append_and_clear() {
    let mut output = Vec::<TokenTree>::new();
    let mut current_str = String::new();

    PostProcessCombineStaticStr::append_and_clear(&mut output, &mut current_str);
    assert_eq!(0, output.len());
    assert_eq!(0, current_str.len());

    current_str.push_str("Hello, world!");
    // only appends ("Hello, world!") when current_str is not empty
    PostProcessCombineStaticStr::append_and_clear(&mut output, &mut current_str);
    assert_eq!(2, output.len());
    match output[0] {
        TokenTree::Group(ref group) => {
            assert_eq!(Delimiter::Parenthesis, group.delimiter());
            let actual_str = snailquote::unescape(group.stream().into_iter().collect::<Vec<TokenTree>>()[0].to_string().as_str()).unwrap();
            assert_eq!("Hello, world!", actual_str);
        },
        _ => panic!("output[0] is not a group")
    }
    match output[1] {
        TokenTree::Punct(ref punct) => {
            assert_eq!(';', punct.as_char());
        },
        _ => panic!("output[1] is not a punct")
    }
    assert_eq!(0, current_str.len());
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