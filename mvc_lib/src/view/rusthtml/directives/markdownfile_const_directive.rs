use std::rc::Rc;

use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parsers::rusthtmlparser_all::IRustHtmlParserAll;
use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;

use super::irusthtml_directive::IRustHtmlDirective;
use super::markdownfile_nocache_directive::MarkdownFileNoCacheDirective;


// The "mdfile" directive is used to render markdown from a file.
pub struct MarkdownFileConstDirective {}

impl MarkdownFileConstDirective {
    pub fn new() -> Self {
        Self {}
    }

    // read and convert a Markdown file directly to RustHtml tokens.
    // identifier: the identifier to convert.
    // output: the destination for the RustHtml tokens.
    // it: the iterator to use.
    // returns: nothing or an error.
    pub fn convert_mdfile_const_directive(ctx: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        MarkdownFileNoCacheDirective::convert_mdfile_nocache_directive(ctx, identifier, ident_token, parser, output, it)
    }
}

impl IRustHtmlDirective for MarkdownFileConstDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_const" || name == "markdownfile_const"
    }

    fn execute(self: &Self, context: Rc<dyn IRustHtmlParserContext>, identifier: &Ident, ident_token: &TokenTree, parser: Rc<dyn IRustHtmlParserAll>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match Self::convert_mdfile_const_directive(context, identifier, ident_token, parser, output, it) {
            Ok(_) => {
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file const '{}': {}", identifier, e)))
            }
        }
    }
}