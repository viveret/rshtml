use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll};


#[test]
pub fn test_rusthtmlparser_all_constructor_works() {
    let _ = RustHtmlParserAll::new_default();
}

#[test]
pub fn test_rusthtmlparser_all_get_html_parser_works() {
    let parser = RustHtmlParserAll::new_default();
    let _ = parser.get_html_parser();
}

#[test]
pub fn test_rusthtmlparser_all_get_rust_parser_works() {
    let parser = RustHtmlParserAll::new_default();
    let _ = parser.get_rust_parser();
}

#[test]
pub fn test_rusthtmlparser_all_get_rust_or_html_parser_works() {
    let parser = RustHtmlParserAll::new_default();
    let _ = parser.get_rust_or_html_parser();
}

#[test]
pub fn test_rusthtmlparser_all_get_converter_works() {
    let parser = RustHtmlParserAll::new_default();
    let _ = parser.get_converter();
}

#[test]
pub fn test_rusthtmlparser_all_get_converter_out_works() {
    let parser = RustHtmlParserAll::new_default();
    let _ = parser.get_converter_out();
}

#[test]
pub fn test_rusthtmlparser_all_get_expander_works() {
    let parser = RustHtmlParserAll::new_default();
    let _ = parser.get_expander();
}
