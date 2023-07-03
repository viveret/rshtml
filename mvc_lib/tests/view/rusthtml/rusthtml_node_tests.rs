use mvc_lib::view::rusthtml::rusthtml_node::RustHtmlNode;
use proc_macro2::{TokenTree, Literal};


#[test]
pub fn rusthtml_node_tests_constructor_works() {
    let _ = RustHtmlNode::Rust(TokenTree::Literal(Literal::string("test")));
}