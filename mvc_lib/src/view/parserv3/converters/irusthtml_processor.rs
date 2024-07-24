use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;


pub trait IRustHtmlProcessor {
    fn get_stage_for(&self) -> &str;
    fn process_rusthtml(&self, rusthtml: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
}