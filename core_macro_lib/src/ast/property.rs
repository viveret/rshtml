use proc_macro2::{TokenTree, Ident};



pub(crate) struct AstPropertySimple {
    pub name: String,
    pub value: String,
}

pub(crate) struct AstProperty {
    pub name: Ident,
    pub value: Vec<TokenTree>,
}