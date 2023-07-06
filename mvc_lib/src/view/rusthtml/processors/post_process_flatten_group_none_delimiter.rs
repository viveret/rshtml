use proc_macro2::{TokenTree, Delimiter, Group, TokenStream};

use crate::view::rusthtml::{irust_processor::IRustProcessor, rusthtml_error::RustHtmlError};





pub struct PostProcessFlattenGroupNoneDelimiter {

}

impl PostProcessFlattenGroupNoneDelimiter {
    pub fn new() -> Self {
        Self { }
    }
}

impl IRustProcessor for PostProcessFlattenGroupNoneDelimiter {
    fn get_stage_for(&self) -> &str {
        "post"
    }

    fn process_rust(&self, rusthtml: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut group = vec![];
        let mut it = rusthtml.into_iter().peekable();
        loop {
            if let Some(token) = it.next() {
                if let TokenTree::Group(g) = token {
                    match g.delimiter() {
                        Delimiter::None => {
                            group.extend_from_slice(self.process_rust(&g.stream().into_iter().collect::<Vec<TokenTree>>())?.as_slice());
                        },
                        Delimiter::Brace | Delimiter::Bracket => {
                            group.push(TokenTree::Group(Group::new(g.delimiter(), TokenStream::from_iter(self.process_rust(&g.stream().into_iter().collect::<Vec<TokenTree>>())?))));
                        },
                        _ => {
                            group.push(TokenTree::Group(g.clone()));
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(group)
    }
}