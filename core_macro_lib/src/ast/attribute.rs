use proc_macro2::{Ident, TokenTree};



pub(crate) struct AstAttributeSimple {
    pub name: String,
    pub value: String,
}

pub(crate) struct AstAttribute {
    pub name: Ident,
    pub value: Vec<TokenTree>,
}