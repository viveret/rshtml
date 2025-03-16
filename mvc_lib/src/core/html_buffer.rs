use std::cell::RefCell;

use crate::view::rusthtml::html_string::HtmlString;




pub trait IHtmlBuffer {
    // write html to the view context.
    // html: the html to write.
    fn write_html(&self, html: HtmlString);
    // write html to the view context.
    // html: the html to write.
    fn write_html_str(&self, html: &str);
    // write content to the view context.
    // content: the content to write.
    fn write_content(&self, content: String);
    // collect the html that has been written to the view context.
    // returns: the html that has been written to the view context.
    fn collect_html(&self) -> HtmlString;
}

pub struct HtmlBuffer {
    buffer: RefCell<String>,
}

impl HtmlBuffer {
    // create a new instance of the html buffer.
    pub fn new() -> Self {
        Self {
            buffer: RefCell::new(String::new()),
        }
    }
}

impl IHtmlBuffer for HtmlBuffer {
    fn write_html(&self, html: HtmlString) {
        self.write_html_str(html.content.as_str());
    }

    fn write_html_str(&self, html: &str) {
        self.buffer.borrow_mut().push_str(html);
    }

    fn write_content(&self, content: String) {
        self.write_html(HtmlString::new_data_string(content))
    }

    fn collect_html(&self) -> HtmlString {
        HtmlString::new_from_html(self.buffer.borrow().clone())
    }
}