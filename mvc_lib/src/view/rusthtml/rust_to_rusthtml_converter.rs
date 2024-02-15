// // based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
// use std::rc::Rc;
// use std::str::FromStr;

// use core_lib::asyncly::icancellation_token::ICancellationToken;
// use core_macro_lib::nameof_member_fn;
// use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

// use crate::view::rusthtml::rusthtml_token::{RustHtmlToken, RustHtmlIdentAndPunctAndGroupOrLiteral, RustHtmlIdentOrPunctOrGroup };
// use crate::view::rusthtml::rusthtml_error::RustHtmlError;

// use super::html_tag_parse_context::HtmlTagParseContext;
// use super::ihtml_tag_parse_context::IHtmlTagParseContext;
// use super::irust_to_rusthtml_converter::IRustToRustHtmlConverter;
// use super::parser_parts::rusthtmlparser_all::{RustHtmlParserAll, IRustHtmlParserAll};
// use super::parser_parts::peekable_tokentree::StreamPeekableTokenTree;
// use super::parser_parts::peekable_tokentree::IPeekableTokenTree;
// use super::rusthtml_directive_result::RustHtmlDirectiveResult;
// use super::irusthtml_parser_context::IRustHtmlParserContext;


// // this implements the IRustToRustHtml trait.
// #[derive(Clone)]
// pub struct RustToRustHtmlConverter {
//     // the context for the RustHtml parser.
//     pub context: Rc<dyn IRustHtmlParserContext>,
//     pub new_parser: Rc<dyn IRustHtmlParserAll>,
// }

// impl RustToRustHtmlConverter {
//     // create a new instance of the RustToRustHtml parser.
//     // context: the context for the RustHtml parser.
//     pub fn new(context: Rc<dyn IRustHtmlParserContext>) -> Self {
//         Self {
//             context: context,
//             new_parser: RustHtmlParserAll::new_default(),
//         }
//     }
// }

// impl IRustToRustHtmlConverter for RustToRustHtmlConverter {
//     fn get_context(self: &Self) -> Rc<dyn IRustHtmlParserContext> {
//         self.context.clone()
//     }
// }
