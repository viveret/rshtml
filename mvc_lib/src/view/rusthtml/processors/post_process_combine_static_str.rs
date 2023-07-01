use proc_macro2::TokenTree;

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::irust_processor::IRustProcessor;


// combine static strings into one literal
pub struct PostProcessCombineStaticStr { }

impl PostProcessCombineStaticStr {
    pub fn new() -> Self {
        Self { }
    }
}

impl IRustProcessor for PostProcessCombineStaticStr {
    fn get_stage_for(&self) -> &str {
        "post"
    }

    fn process_rust(&self, rusthtml: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        let mut current_str = String::new();
        
        fn append_and_clear(output: &mut Vec<TokenTree>, current_str: &mut String) {
            if current_str.len() > 0 {
                output.extend_from_slice(quote::quote! {}.into_iter().collect::<Vec<TokenTree>>().as_slice());
                current_str.clear();
            }
        }

        let mut it = rusthtml.iter().peekable();
        loop {
            if let Some(token) = it.next() {
                match token {
                    TokenTree::Ident(ident) if ident.to_string().as_str() == "html_output" => {
                        // next char has to be '.'
                        if let Some(period_token) = it.peek() {
                            match period_token {
                                TokenTree::Punct(punct) if punct.as_char() == '.' => {
                                    it.next();

                                    // next ident has to be 'write_html_str'
                                    if let Some(write_html_str_token) = it.peek() {
                                        match write_html_str_token {
                                            TokenTree::Ident(ident) if ident.to_string().as_str() == "write_html_str" => {
                                                it.next();

                                                // next char has to be '('
                                                if let Some(open_paren_token) = it.peek() {
                                                    match open_paren_token {
                                                        TokenTree::Punct(punct) if punct.as_char() == '(' => {
                                                            it.next();

                                                            // next is string literal
                                                            if let Some(string_literal_token) = it.peek() {
                                                                match string_literal_token {
                                                                    TokenTree::Literal(literal) => {
                                                                        current_str.push_str(literal.to_string().as_str());
                                                                        it.next();

                                                                        // next char has to be ')'
                                                                        if let Some(close_paren_token) = it.peek() {
                                                                            match close_paren_token {
                                                                                TokenTree::Punct(punct) if punct.as_char() == ')' => {
                                                                                    it.next();
                                                                                    continue;
                                                                                },
                                                                                _ => break,
                                                                            }
                                                                        } else {
                                                                            break;
                                                                        }
                                                                    },
                                                                    _ => break,
                                                                }
                                                            } else {
                                                                break;
                                                            }
                                                        },
                                                        _ => break,
                                                    }
                                                } else {
                                                    break;
                                                }
                                            },
                                            _ => break,
                                        }
                                    } else {
                                        break;
                                    }
                                },
                                _ => break,
                            }
                        }
    
                        append_and_clear(output.as_mut(), &mut current_str);
                        output.push(token.clone());
                    },
                    _ => {
                        append_and_clear(output.as_mut(), &mut current_str);
                        output.push(token.clone());
                    }
                }
            } else {
                break;
            }
        }
        
        append_and_clear(output.as_mut(), &mut current_str);
        Ok(output)
    }
}