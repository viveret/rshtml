use proc_macro2::TokenTree;

use super::rusthtml_error::RustHtmlError;


pub trait IRustProcessor {
    fn get_stage_for(&self) -> &str;
    fn process_rust(&self, rusthtml: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError>;
}