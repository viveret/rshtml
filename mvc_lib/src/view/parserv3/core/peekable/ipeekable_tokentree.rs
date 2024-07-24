use proc_macro2::{TokenStream, TokenTree};

use super::ipeekable::IPeekable;

pub trait IPeekableTokenTree: IPeekable<TokenTree> + std::fmt::Debug {
    fn to_stream(&self) -> TokenStream;
}