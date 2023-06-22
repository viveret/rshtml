use std::fmt;

use crate::view::rusthtml::rusthtml_error::RustHtmlError;

// used in order to automatically escape strings, but leave HTML strings as is.
#[derive(Clone, Debug)]
pub struct HtmlString {
    pub content: String,
}

impl HtmlString {
    // create a new HtmlString from an unescaped string that should be escaped.
    // this is used for data strings, not HTML strings.
    // content_to_escape: the string that should be escaped.
    pub fn new_data_string(content_to_escape: String) -> Self {
        // println!("Escaping data: {}", content_to_escape);
        HtmlString::new_from_html(html_escape::encode_text(&content_to_escape).as_ref().to_string())
    }

    // create a new HtmlString from an HTML string that should not be escaped.
    // html: the HTML string that should not be escaped.
    pub fn new_from_html(html: String) -> Self {
        Self { content: html }
    }

    // create a new HtmlString from an HTML string that should not be escaped.
    // html: the HTML string that should not be escaped.
    pub fn new_from_html_str(html: &'static str) -> Self {
        Self { content: html.to_string() }
    }

    // create a new empty HtmlString.
    pub fn empty() -> Self {
        Self { content: String::new() }
    }

    pub fn default() -> Self {
        Self::empty()
    }

    // whether or not the HtmlString is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    // get the length of the HtmlString.
    pub fn len(&self) -> usize {
        self.content.len()
    }
}

impl fmt::Display for HtmlString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl From<&HtmlString> for HtmlString {
    fn from(item: &HtmlString) -> Self {
        HtmlString::new_from_html(item.content.clone())
    }
}

impl From<String> for HtmlString {
    fn from(item: String) -> Self {
        HtmlString::new_data_string(item)
    }
}

impl From<&String> for HtmlString {
    fn from(item: &String) -> Self {
        HtmlString::new_data_string(item.clone())
    }
}

impl From<&str> for HtmlString {
    fn from(item: &str) -> Self {
        HtmlString::new_data_string(item.to_string())
    }
}

impl From<Option<String>> for HtmlString {
    fn from(item: Option<String>) -> Self {
        match item {
            Some(x) => HtmlString::new_data_string(x),
            None => HtmlString::empty(),
        }
    }
}

impl From<Option<&String>> for HtmlString {
    fn from(item: Option<&String>) -> Self {
        match item {
            Some(x) => HtmlString::new_data_string(x.clone()),
            None => HtmlString::empty(),
        }
    }
}

impl From<Option<&&str>> for HtmlString {
    fn from(item: Option<&&str>) -> Self {
        match item {
            Some(x) => HtmlString::new_data_string(x.to_string()),
            None => HtmlString::empty(),
        }
    }
}

impl <'a> From<Result<HtmlString, RustHtmlError<'a>>> for HtmlString {
    fn from(item: Result<HtmlString, RustHtmlError>) -> Self {
        match item {
            Ok(HtmlString { content }) => HtmlString::new_from_html(content),
            Err(RustHtmlError(e)) => HtmlString::new_data_string(e.to_string()),
        }
    }
}