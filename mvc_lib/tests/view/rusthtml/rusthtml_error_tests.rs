use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;


#[test]
pub fn rusthtml_error_from_str() {
    RustHtmlError::from_str("test");
}

#[test]
pub fn rusthtml_error_from_string() {
    RustHtmlError::from_string("test".to_string());
}