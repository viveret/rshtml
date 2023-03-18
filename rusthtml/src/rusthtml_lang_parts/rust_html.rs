use crate::rusthtml_error::RustHtmlError;

use crate::rusthtml_lang_part::RustHtmlLangPart;

pub struct RustHtml {

}
impl RustHtmlLangPart for RustHtml {
    fn convert_tokenstream_to_rusthtmltokens(self: &Self) -> Result<bool, RustHtmlError<'static>> {
        Ok(true)
    }

    fn convert_rusthtmltokens_to_tokenstream(self: &Self) -> Result<bool, RustHtmlError<'static>> {
        Ok(true)
    }
}