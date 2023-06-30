use super::rusthtml_error::RustHtmlError;
use super::rusthtml_token::RustHtmlToken;


pub trait IRustHtmlProcessor {
    fn get_stage_for(&self) -> &str;
    fn process_rusthtml(&self, rusthtml: &Vec<RustHtmlToken>) -> Result<Vec<RustHtmlToken>, RustHtmlError>;
}