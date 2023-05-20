use crate::view::rusthtml::html_string::HtmlString;

// form helpers for generating HTML forms.
pub struct FormHelpers {

}

impl FormHelpers {
    pub fn new() -> Self {
        Self { }
    }

    // create a submit button with the given text for a form.
    // text: the text to display on the button.
    // returns: the submit button HTML.
    pub fn submit(self: &Self, text: &str) -> HtmlString {
        HtmlString { content: format!("<button type=\"submit\">{}</button>", html_escape::encode_text(&text)) }
    }
}