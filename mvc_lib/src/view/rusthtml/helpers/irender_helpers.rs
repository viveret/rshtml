use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

// Helper functions for RustHtml views.
pub trait IRenderHelpers<'a> {
    // render a section of the view or return an error if it does not exist.
    // section_name: the name of the section to render.
    // returns: the rendered section or an error if it does not exist.
    fn section<'b, 'c, 'd>(self: &Self, section_name: &'b str) -> Result<HtmlString, RustHtmlError<'d>>;

    // render a section of the view (if it exists).
    // section_name: the name of the section to render.
    // returns: the rendered section or an empty string if it does not exist.
    fn section_optional<'b, 'c, 'd>(self: &Self, section_name: &'b str) -> Result<HtmlString, RustHtmlError<'d>>;

    // render the body of the layout view.
    // returns: the rendered body of the layout view or an error.
    fn body<'b>(self: &Self) -> Result<HtmlString, RustHtmlError<'b>>;
}