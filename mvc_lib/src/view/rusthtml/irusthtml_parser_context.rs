// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTracker;
use proc_macro2::{TokenStream, TokenTree};

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use super::directives::irusthtml_directive::IRustHtmlDirective;
use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irust_processor::IRustProcessor;
use super::irusthtml_processor::IRustHtmlProcessor;
use super::node_helpers::inode_parsed::IHtmlNodeParsed;
use super::parser_parts::rusthtmlparser_all::IRustHtmlParserAll;
use super::rusthtml_parser_context::RustHtmlParserContext;
use super::rusthtml_token::RustHtmlToken;
use super::tag_helpers::itag_parsed::IHtmlTagParsed;


// this is the main parser context for the RustHtml language.
// it is used to parse the RustHtml language into a RustHtmlToken stream of RustHtml tokens
// as well as work with the RustHtml stream more easily.
pub trait IRustHtmlParserContext {
    // gets the current stack trace.
    fn get_call_stack(&self) -> &CallstackTracker;
    // gets the max stack trace count allowed before throwing an error.
    fn get_max_call_stack_count(&self) -> usize;
    // return an error if the stack trace count is greater than the max stack count.
    fn check_call_stack_count(&self) -> Result<(), RustHtmlError>;

    // get if the parser is in a tag (true) or in a Rust block (false). True by default and at start of stream.
    fn get_is_in_html_mode(&self) -> bool;
    fn push_is_in_html_mode(&self, v: bool);
    fn pop_is_in_html_mode(&self) -> bool;

    // push a buffer to the buffer stack that is used to store RustHtml tokens.
    fn push_output_buffer(&self, buffer: Rc<RefCell<Vec<RustHtmlToken>>>);
    // pop a buffer from the buffer stack that is used to store RustHtml tokens.
    fn pop_output_buffer(&self) -> Option<Rc<RefCell<Vec<RustHtmlToken>>>>;
    // get the current buffer from the buffer stack that is used to store RustHtml tokens.
    fn get_output_buffer(&self) -> Option<Rc<RefCell<Vec<RustHtmlToken>>>>;

    // push to output buffer / stream
    fn push_output_token(&self, token: RustHtmlToken);
    // push vec to output buffer / stream
    fn push_output_tokens(&self, token: &[RustHtmlToken]);
    
    // whether or not the RustHtml code is raw tokenstream.
    fn get_is_raw_tokenstream(self: &Self) -> bool;
    // get the model type name as a string.
    fn get_model_type_name(self: &Self) -> String;
    // get the model type as a RustHtmlToken.
    fn get_model_type(self: &Self) -> TokenStream;
    // set the model type as a RustHtmlToken.
    fn set_model_type(self: &Self, value: Option<Vec<RustHtmlToken>>, parser: Rc<dyn IRustHtmlParserAll>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>);
    // try to get a parameter value as a string.
    fn try_get_param_string(self: &Self, key: &str) -> Option<String>;
    // get a parameter value as a string.
    // key: the key of the parameter.
    fn get_param_string(self: &Self, key: &str) -> Result<String, RustHtmlError>;
    // get the functions section as a token stream.
    fn get_functions_section(self: &Self) -> Option<TokenStream>;
    // get the struct section as a token stream.
    fn get_struct_section(self: &Self) -> Option<TokenStream>;
    // get the impl section as a token stream.
    fn get_impl_section(self: &Self) -> Option<TokenStream>;
    // get the model ident as a token stream.
    // fn get_model_ident(self: &Self) -> Option<TokenStream>;
    // push a scope to the HTML tag scope stack.
    fn htmltag_scope_stack_push(self: &Self, s: String);
    // pop a scope from the HTML tag scope stack.
    fn htmltag_scope_stack_pop(self: &Self) -> Option<String>;
    // get the current punctuation scope stack.
    fn mut_punct_scope_stack(self: &Self) -> RefMut<Vec<char>>;
    // get the use statements as mutable.
    fn push_use_statements(self: &Self, rust: TokenStream);
    // get the use statements as a single token stream.
    fn get_use_statements_stream(self: &Self) -> proc_macro2::TokenStream;
    // push the inject statements to a list of statements to be injected into the view.
    fn push_inject_statements(self: &Self, rust: TokenStream);
    // push the inject statements to a list of statements to be injected into the view, using RustHtmlToken.
    fn push_inject_statements_rshtml(self: &Self, rust: Vec<RustHtmlToken>, parser: Rc<dyn IRustHtmlParserAll>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>);
    // get the inject statements as a token stream.
    fn get_inject_statements_stream(self: &Self) -> proc_macro2::TokenStream;
    // get the params as mutable.
    fn mut_params(self: &Self) -> RefMut<HashMap<String, String>>;
    // get the environment name.
    fn get_environment_name(self: &Self) -> String;
    // get the raw RustHtml code.
    fn get_raw(&self) -> String;
    // set the raw RustHtml code.
    fn set_raw(self: &Self, value: String);
    // get a labeled section
    fn get_section(self: &Self, name: &String) -> Option<TokenStream>;
    // set a labeled section
    fn set_section(self: &Self, name: String, value: Option<TokenStream>);
    // set the functions section as a token stream.
    fn set_functions_section(self: &Self, value: Option<TokenStream>);
    // set the impl section as a token stream.
    fn set_impl_section(self: &Self, value: Option<TokenStream>);
    // set the struct section as a token stream.
    fn set_struct_section(self: &Self, value: Option<TokenStream>);
    // get the directives available to the parser.
    fn get_directives(self: &Self) -> Vec<Rc<dyn IRustHtmlDirective>>;
    // get the directive with the specified name.
    fn try_get_directive(self: &Self, name: String) -> Option<Rc<dyn IRustHtmlDirective>>;
    // get tag parsed handlers.
    fn get_tag_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlTagParsed>>;
    // get node parsed handlers.
    fn get_node_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlNodeParsed>>;
    // get the preprocessors available to the parser.
    fn get_preprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>>;
    // get the postprocessors available to the parser.
    fn get_postprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>>;
    // get the rust tokentree preprocessors available to the parser.
    fn get_rust_preprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>>;
    // get the rust tokentree postprocessors available to the parser.
    fn get_rust_postprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>>;

    // resolve a full path to a view using different directories.
    // fn resolve_views_path_string(self: &Self, path: &str) -> Option<String>;

    fn push_html_tag_parse_context(self: &Self, tag_parse_ctx: Rc<dyn IHtmlTagParseContext>);

    fn add_operation_to_ooo_log(self: &Self, operation: String);
    fn get_ooo(self: &Self) -> Vec<String>;
}
