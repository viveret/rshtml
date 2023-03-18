use crate::rusthtml_error::RustHtmlError;

pub trait RustHtmlLangPart {
    fn convert_tokenstream_to_rusthtmltokens(self: &Self) -> Result<bool, RustHtmlError<'static>>;
    fn convert_rusthtmltokens_to_tokenstream(self: &Self) -> Result<bool, RustHtmlError<'static>>;
}