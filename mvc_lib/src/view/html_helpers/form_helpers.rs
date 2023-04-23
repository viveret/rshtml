use crate::view::rusthtml::html_string::HtmlString;

pub struct FormHelpers {

}

impl FormHelpers {
    pub fn new() -> Self {
        Self { }
    }

    pub fn submit(self: &Self, text: &str) -> HtmlString {
        HtmlString { content: format!("<button type=\"submit\">{}</button>", html_escape::encode_text(&text)) }
    }
}