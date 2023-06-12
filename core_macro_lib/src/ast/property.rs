use proc_macro2::{TokenTree, Ident};


pub(crate) struct AstProperty {
    pub name: Ident,
    pub value: Vec<TokenTree>,
}

impl AstProperty {
    pub(crate) fn new(name: Ident, value: Vec<TokenTree>) -> Self {
        Self {
            name,
            value,
        }
    }
}