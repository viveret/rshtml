use std::iter::Peekable;
use std::rc::Rc;

use proc_macro::{ Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::core::panic_or_return_error::PanicOrReturnError;
use crate::view::rusthtml::rusthtml_token::{RustHtmlToken };
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::view::rusthtml::peekable_tokentree::PeekableTokenTree;
use super::irusthtml_to_rust_converter::IRustHtmlToRustConverter;
use super::rust_to_rusthtml_converter::RustToRustHtmlConverter;
use super::rusthtml_parser_context::{IRustHtmlParserContext, RustHtmlParserContext};
use super::rusthtml_to_rust_converter::RustHtmlToRustConverter;
use super::irust_to_rusthtml_converter::IRustToRustHtmlConverter;


// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs


// this is the main parser for the RustHtml language.
// it is used to parse the RustHtml language into a RustHtmlToken stream of RustHtml tokens
// as well as work with the RustHtml stream more easily.
pub struct RustHtmlParser {
    pub parse_context: Rc<dyn IRustHtmlParserContext>,
    pub parser: Rc<dyn IRustToRustHtmlConverter>,
    pub converter: Rc<dyn IRustHtmlToRustConverter>,
}

impl RustHtmlParser {
    // creates a new RustHtmlParser.
    // should_panic_or_return_error: whether or not to panic or return an error when an error occurs.
    // environment_name: the name of the environment to use.
    // returns: a new RustHtmlParser.
    pub fn new(should_panic_or_return_error: bool, environment_name: String) -> Self {
        let parse_context = Rc::new(RustHtmlParserContext::new(true, should_panic_or_return_error, environment_name));
        Self {
            parse_context: parse_context.clone(),
            parser: Rc::new(RustToRustHtmlConverter::new(parse_context.clone())),
            converter: Rc::new(RustHtmlToRustConverter::new(parse_context)),
        }
    }

    // expand a token stream from a token stream. this is used for compiling the RustHtml code into Rust code.
    // input: the tokens to expand.
    // returns: the expanded tokens.
    pub fn expand_tokenstream(self: &Self, input: TokenStream) -> Result<TokenStream, RustHtmlError> {
        let it = Rc::new(PeekableTokenTree::new(input));

        let rusthtml_tokens_for_view = self.parser.parse_tokenstream_to_rusthtmltokens(true, it, false)?;

        // prefix with _view_start
        let rusthtml_tokens = match self.parse_context.get_param_string("viewstart") {
            Ok(view_start_path) => {
                // try different paths to resolve to existing file.
                // let folder = String::new();
                // let final_path = self.parser.resolve_views_path_str(&view_start_path)?;

                let mut view_start_tokens = vec![];
                self.parser.expand_external_tokenstream(&view_start_path, &mut view_start_tokens)?;

                view_start_tokens.iter().chain(rusthtml_tokens_for_view.iter()).cloned().collect()
            },
            _ => {
                rusthtml_tokens_for_view
            }
        };

        let rust_output = self.converter.parse_rusthtmltokens_to_plain_rust(&rusthtml_tokens)?;
        self.parse_context.set_raw(self.display_as_code(&mut rust_output.iter().cloned().peekable()));
        Ok(TokenStream::from_iter(rust_output))
    }

    // print a token stream as code.
    // rust_output: the token stream to print.
    // returns: nothing.
    pub fn print_as_code(self: &Self, rust_output: &Vec<TokenTree>) {
        println!("{}", self.display_as_code(&mut rust_output.iter().cloned().peekable()));
    }

    // display a token stream as code and return it as a string.
    // rust_output: the token stream to display.
    // returns: the token stream as a string.
    pub fn display_as_code(self: &Self, rust_output: &mut Peekable<impl Iterator<Item=TokenTree>>) -> String {
        let mut s = String::new();
        for token in rust_output {
            if let TokenTree::Group(group) = token {
                let delimiter = group.delimiter();
                s.push_str(self.parser.get_opening_delim(delimiter));
                s.push_str(&self.display_as_code(&mut group.stream().into_iter().peekable()));
                s.push_str(self.parser.get_closing_delim(delimiter));
            } else {
                s.push_str(&token.to_string());
            }
        }
        s
    }

    // panic or return an error. if should_panic_or_return_error is true, then panic. otherwise, return an error.
    // message: the error message.
    // returns: an error with the message.
    pub fn panic_or_return_error<'a, T>(self: &Self, message: String) -> Result<T, RustHtmlError<'a>> {
        return PanicOrReturnError::panic_or_return_error(self.parse_context.get_should_panic_or_return_error(), message);
    }

    // parse a token stream to a syn AST.
    // input: the token stream to parse.
    // returns: the AST or an error.
    pub fn parse_to_ast(self: &Self, input: TokenStream) -> Result<syn::Item, RustHtmlError> {
        let ts = self.expand_tokenstream(input)?;
        let ast = syn::parse(ts).unwrap();
        Ok(ast)
    }

    // assert that the next token is a punct. if it is, return nothing. otherwise, return the unexpected token.
    // c: the punct to expect.
    // it: the iterator to use.
    // returns: nothing or the unexpected token.
    pub fn expect_punct(self: &Self, c: char, it: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(), Option<TokenTree>> {
        if let Some(actual_c_token) = it.peek() {
            match actual_c_token { 
                TokenTree::Punct(punct) => {
                    let actual_c = punct.as_char();
                    if actual_c == c {
                        Ok(())
                    } else {
                        Err(Some(actual_c_token.clone()))
                    }
                },
                _ => Err(Some(actual_c_token.clone()))
            }
        } else {
            Err(None)
        }
    }

    // insert a self. before an identifier.
    // output: the destination for the RustHtml tokens.
    pub fn insert_self_dot(self: &Self, output: &mut Vec<RustHtmlToken<Ident, Punct, Literal>>) {
        output.push(RustHtmlToken::Identifier(Ident::new("self", Span::call_site())));
        output.push(RustHtmlToken::ReservedChar('.', Punct::new('.', Spacing::Alone)));
    }
}
