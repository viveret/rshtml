use std::io::Read;
use std::rc::Rc;

use proc_macro2::Ident;

use crate::view::rusthtml::peekable_tokentree::IPeekableTokenTree;
use crate::view::rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken};
use crate::view::rusthtml::rusthtml_directive_result::RustHtmlDirectiveResult;
use crate::view::rusthtml::irust_to_rusthtml_converter::IRustToRustHtmlConverter;

use super::irusthtml_directive::IRustHtmlDirective;


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
    fn convert_mdfile_const_directive(identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<(), RustHtmlError<'static>> {
        match parser.convert_views_path_str(identifier.clone(), it, parser.get_context().get_is_raw_tokenstream()) {
            Ok(path) => {
                match std::fs::File::open(path.as_str()) {
                    Ok(mut f) => {
                        let mut buffer = String::new();
                        f.read_to_string(&mut buffer).expect("could not read markdown file");
                        let mdtext = comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default());
                        output.push(RustHtmlToken::HtmlTextNode(mdtext, identifier.span()));
                    },
                    Err(e) => {
                        return Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}', could not open: {:?}", path, e)));
                    }
                }
                Ok(())
            },
            Err(RustHtmlError(e)) => {
                return Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}', could not parse path: {}", identifier, e)));
            }
        }
    }
}

impl IRustHtmlDirective for MarkdownFileConstDirective {
    fn matches(self: &Self, name: &String) -> bool {
        name == "mdfile_const" || name == "markdownfile_const"
    }

    fn execute(self: &Self, identifier: &Ident, parser: Rc<dyn IRustToRustHtmlConverter>, output: &mut Vec<RustHtmlToken>, it: Rc<dyn IPeekableTokenTree>) -> Result<RustHtmlDirectiveResult, RustHtmlError> {
        match Self::convert_mdfile_const_directive(identifier, parser, output, it) {
            Ok(_) => {
                Ok(RustHtmlDirectiveResult::OkContinue)
            },
            Err(RustHtmlError(e)) => {
                Err(RustHtmlError::from_string(format!("cannot read external markdown file '{}': {}", identifier, e)))
            }
        }
    }
}