// This enum is used to return from a directive function to indicate if
// - the directive should continue,
// - if it should break, and
// - whether or not to append the HTML.
#[derive(Debug, PartialEq)]
pub enum RustHtmlDirectiveResult {
    // the directive was parsed successfully and should continue
    OkContinue,
    // the directive was parsed successfully, should break, and should append the HTML
    OkBreakAppendHtml,
    // the directive was parsed successfully and should break
    OkBreak,
}