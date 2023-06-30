use proc_macro2::TokenTree;

use crate::view::rusthtml::{irust_processor::IRustProcessor, rusthtml_error::RustHtmlError};


// combine static strings into one literal
pub struct PostProcessCombineStaticStr {
}

impl PostProcessCombineStaticStr {
    pub fn new() -> Self {
        Self {
        }
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

        for token in rusthtml {
            match token {
                TokenTree::Ident(ident) if ident.to_string().as_str() == "" => {
                    if let Some(crate::view::rusthtml::rusthtml_token::RustHtmlToken::Literal(l, s)) = s.first() {
                        if let Some(l) = l {
                            current_str.push_str(l.to_string().as_str());
                            continue;
                        } else if let Some(s) = s {
                            current_str.push_str(s);
                            continue;
                        } else {
                            panic!("Literal and string are both None");
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
        }
        append_and_clear(output.as_mut(), &mut current_str);
        Ok(output)
    }

    // fn process_rusthtml(&self, rusthtml: &Vec<crate::view::rusthtml::rusthtml_token::RustHtmlToken>) -> Result<Vec<crate::view::rusthtml::rusthtml_token::RustHtmlToken>, crate::view::rusthtml::rusthtml_error::RustHtmlError> {
    //     let mut output = vec![];
    //     let mut current_str = String::new();
        
    //     fn append_and_clear(output: &mut Vec<crate::view::rusthtml::rusthtml_token::RustHtmlToken>, current_str: &mut String) {
    //         if current_str.len() > 0 {
    //             output.push(RustHtmlToken::AppendToHtml(
    //                 vec![RustHtmlToken::Literal(None, Some(current_str.clone()))]
    //             ));
    //             current_str.clear();
    //         }
    //     }

    //     for token in rusthtml {
    //         match token {
    //             crate::view::rusthtml::rusthtml_token::RustHtmlToken::AppendToHtml(s) => {
    //                 if let Some(crate::view::rusthtml::rusthtml_token::RustHtmlToken::Literal(l, s)) = s.first() {
    //                     if let Some(l) = l {
    //                         current_str.push_str(l.to_string().as_str());
    //                         continue;
    //                     } else if let Some(s) = s {
    //                         current_str.push_str(s);
    //                         continue;
    //                     } else {
    //                         panic!("Literal and string are both None");
    //                     }
    //                 }
    //                 append_and_clear(output.as_mut(), &mut current_str);
    //                 output.push(token.clone());
    //             },
    //             _ => {
    //                 append_and_clear(output.as_mut(), &mut current_str);
    //                 output.push(token.clone());
    //             }
    //         }
    //     }
    //     append_and_clear(output.as_mut(), &mut current_str);
    //     Ok(output)
    // }
}