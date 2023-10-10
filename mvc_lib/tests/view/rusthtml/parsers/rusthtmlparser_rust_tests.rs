
#[test]
pub fn test_RustHtmlParserRust_compared_to_RustHtmlParser_parse_type_identifier() {
    let ctx = Rc::new(RustHtmlParserContext::new(false, true, "test".to_string()));
    let parser_old = RustToRustHtmlConverter::new(ctx);
    let parser_new = RustHtmlParserRust::new();

    let inputs = vec![
        quote::quote! {
            std::rc::Rc<std::cell::RefCell<std::vec::Vec<std::string::String>>>
        },
        quote::quote! {
            String
        },
        quote::quote! {
            std::vec::Vec<std::string::String>
        },
    ];

    for input in inputs {
        let it = Rc::new(PeekableTokenTree::new(input.clone()));
        let it2 = Rc::new(PeekableTokenTree::new(input.clone()));

        let old_output = parser_old.parse_type_identifier(it).unwrap();
        let new_output = parser_new.parse_type_identifier(it2).unwrap();

        let old_str = old_output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        let new_str = new_output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");

        assert_ne!(old_str, "");
        assert_ne!(new_str, "");
        assert_eq!(old_str, new_str);
    }
}