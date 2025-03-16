

use as_any::AsAny;
use nameof::{name_of, name_of_type};
use proc_macro2::{TokenTree, Literal, Ident, Group, Punct};

// assert that the token is a punct with the given value
pub fn assert_tokentree_punct(punct: &TokenTree, expected: char) -> Punct {
    if let TokenTree::Punct(punct) = punct {
        assert_eq!(punct.as_char(), expected);
        punct.clone()
    } else {
        panic!("expected punct, received {:?}", punct);
    }
}

// assert that the token is a group with the given delimiter
pub fn assert_tokentree_group(group: &TokenTree, expected: proc_macro2::Delimiter) -> Group {
    if let TokenTree::Group(group) = group {
        assert_eq!(group.delimiter(), expected);
        group.clone()
    } else {
        panic!("expected group, received {:?}", group);
    }
}

// assert that the token is an ident with the given value
pub fn assert_tokentree_ident(ident: &TokenTree, expected: &str) -> Ident {
    if let TokenTree::Ident(ident) = ident {
        assert_eq!(ident.to_string(), expected);
        ident.clone()
    } else {
        panic!("expected ident, received {:?}", ident);
    }
}

// assert that the token is a literal with the given value
pub fn assert_tokentree_literal(literal: &TokenTree, expected: &str) -> Literal {
    if let TokenTree::Literal(literal) = literal {
        assert_eq!(literal.to_string(), expected);
        literal.clone()
    } else {
        panic!("expected literal, received {:?}", literal);
    }
}

// assert that the token is a stream with the given value
pub fn assert_tokentree_stream(left: proc_macro2::TokenStream, right: proc_macro2::TokenStream) {
    let mut it_left = left.into_iter();
    let mut it_right = right.into_iter();
    while let Some((token_left, token_right)) = iterate_left_and_right(&mut it_left, &mut it_right) {
        assert_tokentree(token_left, token_right);
    }

    // Ensure no extra tokens are left in either stream
    assert!(it_left.next().is_none(), "Left token stream has extra tokens.");
    assert!(it_right.next().is_none(), "Right token stream has extra tokens.");
}

// Helper function to iterate over both token streams and return pairs of tokens
fn iterate_left_and_right<'a>(
    it_left: &mut impl Iterator<Item = TokenTree>,
    it_right: &mut impl Iterator<Item = TokenTree>
) -> Option<(TokenTree, TokenTree)> {
    let token_left = it_left.next();
    let token_right = it_right.next();
    
    match (token_left, token_right) {
        (Some(tl), Some(tr)) => Some((tl, tr)),
        (None, None) => None, // Both iterators exhausted
        (Some(tl), None) => panic!(
            "Mismatch: Left token stream has more tokens than the right.\n\
             Extra token in left: `{}`",
            tl
        ),
        (None, Some(tr)) => panic!(
            "Mismatch: Right token stream has more tokens than the left.\n\
             Extra token in right: `{}`",
            tr
        ),
    }
}

// Function to assert equality of individual tokens
fn assert_tokentree(token_left: TokenTree, token_right: TokenTree) {
    let ltype = tokentree_variant_name(&token_left);
    let rtype = tokentree_variant_name(&token_right);
    assert_eq!(token_left.to_string(), token_right.to_string(), "Token mismatch: left ({}) = {}, right ({}) = {}", ltype, token_left, rtype, token_right);
}


fn tokentree_variant_name(t: &TokenTree) -> &'static str {
    match t {
        TokenTree::Group(g) => {
            match g.delimiter() {
                proc_macro2::Delimiter::Parenthesis => "paren group",
                proc_macro2::Delimiter::Brace => "brace group",
                proc_macro2::Delimiter::Bracket => "bracket group",
                proc_macro2::Delimiter::None => "paren group",
            }
        }
        TokenTree::Ident(_) => "ident",
        TokenTree::Punct(_) => "punct",
        TokenTree::Literal(_) => "literal",
    }
}