use std::str::FromStr;
use std::{rc::Rc, cell::RefCell};

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTrackerScope;
use core_macro_lib::{callstack_tracker_scope_and_assert, nameof_member_fn};
use proc_macro2::{Delimiter, Literal, TokenStream};
use proc_macro2::{TokenTree, Group, Ident, Punct};

use crate::view::rusthtml::html_tag_parse_context::HtmlTagParseContext;
use crate::view::rusthtml::ihtml_tag_parse_context::IHtmlTagParseContext;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup, RustHtmlToken};

use super::peekable_rusthtmltoken::{IPeekableRustHtmlToken, VecPeekableRustHtmlToken};
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
