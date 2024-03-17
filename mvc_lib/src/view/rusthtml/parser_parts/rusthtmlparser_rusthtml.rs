use std::{rc::Rc, cell::RefCell};

use proc_macro2::Group;

use super::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use super::rusthtmlparser_all::IRustHtmlParserAll;
use super::rusthtmlparser_all::IRustHtmlParserAssignSharedParts;



pub trait IRustHtmlParserRustOrHtml: IRustHtmlParserAssignSharedParts {
}

pub struct RustHtmlParserRustOrHtml {
    shared_parser: RefCell<Option<Rc<dyn IRustHtmlParserAll>>>,
}

impl RustHtmlParserRustOrHtml {
    pub fn new() -> Self {
        Self {
            shared_parser: RefCell::new(None),
        }
    }

    pub fn foobar_group(&self, t: &Group) -> Rc<dyn IPeekableTokenTree> {
        Rc::new(StreamPeekableTokenTree::new(t.stream()))
    }
}

impl IRustHtmlParserRustOrHtml for RustHtmlParserRustOrHtml {
}

impl IRustHtmlParserAssignSharedParts for RustHtmlParserRustOrHtml {
    fn assign_shared_parser(self: &Self, parser: Rc<dyn IRustHtmlParserAll>) {
        *self.shared_parser.borrow_mut() = Some(parser.clone());
    }
}
