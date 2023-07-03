use std::borrow::Borrow;

use proc_macro2::{TokenTree, Delimiter, Group, TokenStream};

use crate::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::irust_processor::IRustProcessor;


// combine static strings into one literal
pub struct PostProcessCombineStaticStr { }

impl PostProcessCombineStaticStr {
    pub fn new() -> Self {
        Self { }
    }
}

fn append_and_clear(output: &mut Vec<TokenTree>, current_str: &mut String) {
    if current_str.len() > 0 {
        let inner = TokenStream::from_iter(vec![ TokenTree::Literal(proc_macro2::Literal::string(current_str.as_str())) ]);
        output.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)));
        output.push(TokenTree::Punct(proc_macro2::Punct::new(';', proc_macro2::Spacing::Alone)));
        current_str.clear();
    }
}

fn is_ident_with_name(add_to_output: bool, name: &str, output: &mut Vec<TokenTree>, it: &mut std::iter::Peekable<std::slice::Iter<'_, TokenTree>>) -> bool {
    if let Some(token) = it.peek() {
        match token {
            TokenTree::Ident(ident) if ident.to_string().as_str() == name => {
                if add_to_output {
                    output.push(it.next().unwrap().clone());
                } else {
                    it.next().unwrap();
                }
                return true;
            },
            _ => return false,
        }
    } else {
        return false;
    }
}

fn is_write_html_str(add_to_output: bool, output: &mut Vec<TokenTree>, it: &mut std::iter::Peekable<std::slice::Iter<'_, TokenTree>>) -> bool {
    is_ident_with_name(add_to_output, "write_html_str", output, it)
}

fn is_html_output(add_to_output: bool, output: &mut Vec<TokenTree>, it: &mut std::iter::Peekable<std::slice::Iter<'_, TokenTree>>) -> bool {
    is_ident_with_name(add_to_output, "html_output", output, it)
}

fn is_punct_with_char(add_to_output: bool, c: char, output: &mut Vec<TokenTree>, it: &mut std::iter::Peekable<std::slice::Iter<'_, TokenTree>>) -> bool {
    if let Some(token) = it.peek() {
        match token {
            TokenTree::Punct(punct) if punct.as_char() == c => {
                if add_to_output {
                    output.push(it.next().unwrap().clone());
                } else {
                    it.next().unwrap();
                }
                return true;
            },
            _ => return false,
        }
    } else {
        return false;
    }
}

fn is_group(delimiter: Delimiter, str_output: &mut String, it: &mut std::iter::Peekable<std::slice::Iter<'_, TokenTree>>) -> bool {
    if let Some(token) = it.peek() {
        match token {
            TokenTree::Group(group) if group.delimiter() == delimiter => {
                it.next();
                // next is string literal
                if is_string_literal(str_output, PeekableTokenTree::new(group.stream())) {
                    return true;
                } else {
                    return false;
                }
            },
            _ => return false,
        }
    } else {
        return false;
    }
}

fn is_string_literal(str_output: &mut String, it: PeekableTokenTree) -> bool {
    if let Some(token) = it.peek() {
        match token {
            TokenTree::Literal(_) => {
                let s1 = it.next().unwrap().to_string();
                let s = s1.replace("\\\\", "\\").replace("\\\"", "\"").replace("\"", "");
                str_output.push_str(s.borrow());
                return true;
            },
            _ => return false,
        }
    } else {
        return false;
    }
}

impl IRustProcessor for PostProcessCombineStaticStr {
    fn get_stage_for(&self) -> &str {
        "post"
    }

    fn process_rust(&self, rusthtml: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        let mut current_str = String::new();
        let mut is_first = true;

        let mut it = rusthtml.iter().peekable();
        loop {
            if is_html_output(is_first, &mut output, &mut it) {
                // next char has to be '.'
                if is_punct_with_char(is_first, '.', &mut output, &mut it) {
                    // next ident has to be 'write_html_str'
                    if is_write_html_str(is_first, &mut output, &mut it) {
                        // next char has to be '('
                        if is_group(Delimiter::Parenthesis, &mut current_str, &mut it) {
                            is_first = false;
                            // next char has to be ';'
                            if is_punct_with_char(is_first, ';', &mut output, &mut it) {
                                continue;
                            }
                        }
                    }
                }
            }

            is_first = true;
            append_and_clear(output.as_mut(), &mut current_str);
            if let Some(token) = it.next() {
                output.push(token.clone());
            } else {
                break;
            }
        }
        
        append_and_clear(output.as_mut(), &mut current_str);
        Ok(output)
    }
}