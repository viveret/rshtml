use std::fmt;

use crate::view::rusthtml::rusthtml_error::RustHtmlError;

// used in order to automatically escape strings, but leave HTML strings as is
#[derive(Clone, Debug)]
pub struct HtmlString {
    pub content: String,
}

impl HtmlString {
    pub fn new_data_string(content_to_escape: String) -> Self {
        // println!("Escaping data: {}", content_to_escape);
        HtmlString::new_from_html(html_escape::encode_text(&content_to_escape).as_ref().to_string())
    }
    
    // pub fn new_from_eval(val: eval::Value) -> Self {
    //     match val {
    //         eval::Value::String(s) => {
    //             // println!("new from eval string: {}", s.to_string());
    //             HtmlString::new_data_string(s.to_string())
    //         },
    //         eval::Value::Number(val_as_number) => {
    //             HtmlString::new_from_html(format!("{:?}", val_as_number))
    //         },
    //         eval::Value::Bool(val_as_bool) => {
    //             HtmlString::new_from_html(format!("{:?}", val_as_bool))
    //         },
    //         eval::Value::Array(val_as_array) => {
    //             HtmlString::new_from_html(format!("{:?}", val_as_array))
    //         },
    //         eval::Value::Object(val_as_object) => {
    //             if val_as_object.contains_key("html") && val_as_object.len() == 1 {
    //                 HtmlString::new_from_html(val_as_object.get("html").unwrap().as_str().unwrap().to_string())
    //             } else {
    //                 HtmlString::new_from_html(format!("{:?}", val_as_object))
    //             }
    //         },
    //         eval::Value::Null => {
    //             HtmlString::empty()
    //         },
    //     }
    // }

    pub fn new_from_html(html: String) -> Self {
        Self { content: html }
    }

    pub fn new_from_html_str(html: &'static str) -> Self {
        Self { content: html.to_string() }
    }

    pub fn empty() -> Self {
        Self { content: "".to_string() }
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