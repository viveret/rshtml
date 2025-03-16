use crate::view::rusthtml::rusthtml_token::RustHtmlToken;



pub fn assert_rusthtmltoken(left: &RustHtmlToken, right: &RustHtmlToken) {
    assert_eq!(left.variant_name(), right.variant_name(), "different types");
}