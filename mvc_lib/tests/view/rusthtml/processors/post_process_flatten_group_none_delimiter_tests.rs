use mvc_lib::view::rusthtml::irust_processor::IRustProcessor;
use mvc_lib::view::rusthtml::processors::post_process_flatten_group_none_delimiter::PostProcessFlattenGroupNoneDelimiter;
use proc_macro2::{TokenTree, Group, Delimiter, TokenStream, Literal};



#[test]
pub fn post_process_flatten_group_none_delimiter_process_rust_empty() {
    let processor = PostProcessFlattenGroupNoneDelimiter::new();
    let input = vec![];
    let result = processor.process_rust(&input)
                                    .expect("post_process_flatten_group_none_delimiter_process_rust_empty");
    assert_eq!(0, result.len());
}

#[test]
pub fn post_process_flatten_group_none_delimiter_process_rust_basic() {
    let processor = PostProcessFlattenGroupNoneDelimiter::new();
    let input = vec![
        TokenTree::Group(Group::new(Delimiter::None, TokenStream::from_iter(vec![
            TokenTree::Literal(Literal::string("test")),
        ]))),
    ];
    let result = processor.process_rust(&input)
                                    .expect("post_process_flatten_group_none_delimiter_process_rust_basic");
    assert_eq!(1, result.len());

    let output_stream = TokenStream::from_iter(result).to_string();
    assert_eq!("\"test\"", output_stream);
}