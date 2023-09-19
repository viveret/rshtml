

use proc_macro2::TokenTree;

// assert that the token is a punct with the given value
pub fn assert_tokentree_punct(punct: &TokenTree, expected: char) {
    if let TokenTree::Punct(punct) = punct {
        assert_eq!(punct.as_char(), expected);
    } else {
        panic!("expected punct, received {:?}", punct);
    }
}

// assert that the token is a group with the given delimiter
pub fn assert_tokentree_group(group: &TokenTree, expected: proc_macro2::Delimiter) {
    if let TokenTree::Group(group) = group {
        assert_eq!(group.delimiter(), expected);
    } else {
        panic!("expected group, received {:?}", group);
    }
}

// assert that the token is an ident with the given value
pub fn assert_tokentree_ident(ident: &TokenTree, expected: &str) {
    if let TokenTree::Ident(ident) = ident {
        assert_eq!(ident.to_string(), expected);
    } else {
        panic!("expected ident, received {:?}", ident);
    }
}

// assert that the token is a literal with the given value
pub fn assert_tokentree_literal(literal: &TokenTree, expected: &str) {
    if let TokenTree::Literal(literal) = literal {
        assert_eq!(literal.to_string(), expected);
    } else {
        panic!("expected literal, received {:?}", literal);
    }
}

// assert that the token is a stream with the given value
pub fn assert_tokentree_stream(stream: &proc_macro2::TokenStream, expected: &str) {
    for token in stream.clone() {
        match &token {
            proc_macro2::TokenTree::Ident(ident) => {
                assert_tokentree_ident(&token, expected);
            },
            proc_macro2::TokenTree::Punct(punct) => {
                assert_tokentree_punct(&token, expected.chars().next().unwrap());
            },
            proc_macro2::TokenTree::Literal(literal) => {
                assert_tokentree_literal(&token, expected);
            },
            proc_macro2::TokenTree::Group(group) => {
                assert_tokentree_group(&token, proc_macro2::Delimiter::Brace);
            },
        }
    }
}