use std::rc::Rc;

use super::{rusthtmlparser_html::IRustHtmlParserHtml, rusthtmlparser_rust::IRustHtmlParserRust, rusthtmlparser_rusthtml::IRustHtmlParserRustOrHtml};

pub trait IRustHtmlParserAll {
    fn get_html_parser(&self) -> Rc<dyn IRustHtmlParserHtml>;
    fn get_rust_parser(&self) -> Rc<dyn IRustHtmlParserRust>;
    fn get_rust_or_html_parser(&self) -> Rc<dyn IRustHtmlParserRustOrHtml>;
}

pub trait IRustHtmlParserAssignSharedParts {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>);
}


pub struct RustHtmlParserAll {
    html_parser: Rc<dyn IRustHtmlParserHtml>,
    rust_parser: Rc<dyn IRustHtmlParserRust>,
    rust_or_html_parser: Rc<dyn IRustHtmlParserRustOrHtml>,
}

impl RustHtmlParserAll {
    pub fn new(
        html_parser: Rc<dyn IRustHtmlParserHtml>,
        rust_parser: Rc<dyn IRustHtmlParserRust>,
        rust_or_html_parser: Rc<dyn IRustHtmlParserRustOrHtml>,
    ) -> Self {
        Self {
            html_parser,
            rust_parser,
            rust_or_html_parser,
        }
    }
}

impl IRustHtmlParserAll for RustHtmlParserAll {
    fn get_html_parser(&self) -> Rc<dyn IRustHtmlParserHtml> {
        self.html_parser.clone()
    }

    fn get_rust_parser(&self) -> Rc<dyn IRustHtmlParserRust> {
        self.rust_parser.clone()
    }

    fn get_rust_or_html_parser(&self) -> Rc<dyn IRustHtmlParserRustOrHtml> {
        self.rust_or_html_parser.clone()
    }
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserAll {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        self.html_parser.assign_shared_parser(parser.clone());
        self.rust_parser.assign_shared_parser(parser.clone());
        self.rust_or_html_parser.assign_shared_parser(parser.clone());
    }
}
