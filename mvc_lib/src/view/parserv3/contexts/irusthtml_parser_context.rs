// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use proc_macro2::{TokenStream, TokenTree};

use core_lib::sys::call_tracker::CallstackTracker;
use core_lib::asyncly::icancellation_token::ICancellationToken;

use crate::view::parserv3::converters::inode_parsed::IHtmlNodeParsed;
use crate::view::parserv3::converters::irust_processor::IRustProcessor;
use crate::view::parserv3::converters::irusthtml_processor::IRustHtmlProcessor;
use crate::view::parserv3::converters::itag_parsed::IHtmlTagParsed;
use crate::view::parserv3::directives::irusthtml_directive::IRustHtmlDirective;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;
use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::rusthtml_output_buffer::RustHtmlTokenBuffer;


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
    fn push_output_buffer(&self, buffer: RustHtmlTokenBuffer);
    // pop a buffer from the buffer stack that is used to store RustHtml tokens.
    fn pop_output_buffer(&self) -> Option<RustHtmlTokenBuffer>;
    // get the current buffer from the buffer stack that is used to store RustHtml tokens.
    fn get_output_buffer(&self) -> Option<RustHtmlTokenBuffer>;

    // push to output buffer / stream
    fn push_output_token(&self, token: RustHtmlToken) -> Result<(), RustHtmlError>;
    // push vec to output buffer / stream
    fn push_output_tokens(&self, token: &[RustHtmlToken]) -> Result<(), RustHtmlError>;
    
    // whether or not the RustHtml code is raw tokenstream.
    fn get_is_raw_tokenstream(&self) -> bool;
    // get the model type name as a string.
    fn get_model_type_name(&self) -> String;
    // get the model type as a token tree stream.
    fn get_model_type_stream(&self) -> TokenStream;
    // get the model type as a token tree.
    fn get_model_type(&self) -> Vec<TokenTree>;
    // set the model type as a token tree.
    fn set_model_type(&self, value: Option<Vec<TokenTree>>);
    // try to get a parameter value as a string.
    fn try_get_param_string(&self, key: &str) -> Option<String>;
    // get a parameter value as a string.
    // key: the key of the parameter.
    fn get_param_string(&self, key: &str) -> Result<String, RustHtmlError>;
    // get the functions section as a token stream.
    fn get_functions_section(&self) -> Option<TokenStream>;
    // get the struct section as a token stream.
    fn get_struct_section(&self) -> Option<TokenStream>;
    // get the impl section as a token stream.
    fn get_impl_section(&self) -> Option<TokenStream>;
    // get the model ident as a token stream.
    fn get_model_ident(&self) -> Option<TokenStream>;
    // push a scope to the HTML tag scope stack.
    fn htmltag_scope_stack_push(&self, s: String);
    // pop a scope from the HTML tag scope stack.
    fn htmltag_scope_stack_pop(&self) -> Option<String>;
    // get the current punctuation scope stack.
    fn mut_punct_scope_stack(&self) -> RefMut<Vec<char>>;
    // get the use statements as mutable.
    fn push_use_statements(&self, rust: TokenStream);
    
    fn get_implicit_use_statements(&self) -> proc_macro2::TokenStream;
    // get the use statements as a single token stream.
    fn get_use_statements_stream(&self) -> proc_macro2::TokenStream;
    // push the inject statements to a list of statements to be injected into the view.
    fn push_inject_statements(&self, rust: TokenStream);
    // get the inject statements as a token stream.
    fn get_inject_statements_stream(&self) -> proc_macro2::TokenStream;
    // get the params as mutable.
    fn mut_params(&self) -> RefMut<HashMap<String, String>>;
    // insert into the params
    fn insert_params(&self, key: String, value: String);
    // get the environment name.
    fn get_environment_name(&self) -> String;
    // get the raw RustHtml code.
    fn get_raw(&self) -> String;
    // set the raw RustHtml code.
    fn set_raw(&self, value: String);
    // get a labeled section
    fn get_section(&self, name: &String) -> Option<TokenStream>;
    // set a labeled section
    fn set_section(&self, name: String, value: Option<TokenStream>);
    // set the functions section as a token stream.
    fn set_functions_section(&self, value: Option<TokenStream>);
    // set the impl section as a token stream.
    fn set_impl_section(&self, value: Option<TokenStream>);
    // set the struct section as a token stream.
    fn set_struct_section(&self, value: Option<TokenStream>);
    // get the directives available to the parser.
    fn get_directives(&self) -> Vec<Rc<dyn IRustHtmlDirective>>;
    // get the directive with the specified name.
    fn try_get_directive(&self, name: String) -> Option<Rc<dyn IRustHtmlDirective>>;
    // get tag parsed handlers.
    fn get_tag_parsed_handler(&self) -> Vec<Rc<dyn IHtmlTagParsed>>;
    // get node parsed handlers.
    fn get_node_parsed_handler(&self) -> Vec<Rc<dyn IHtmlNodeParsed>>;
    // get the preprocessors available to the parser.
    fn get_preprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>>;
    // get the postprocessors available to the parser.
    fn get_postprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>>;
    // get the rust tokentree preprocessors available to the parser.
    fn get_rust_preprocessors(&self) -> Vec<Rc<dyn IRustProcessor>>;
    // get the rust tokentree postprocessors available to the parser.
    fn get_rust_postprocessors(&self) -> Vec<Rc<dyn IRustProcessor>>;

    // resolve a full path to a view using different directories.
    // fn resolve_views_path_string(&self, path: &str) -> Option<String>;

    fn push_html_tag_parse_context(&self, tag_parse_ctx: Rc<dyn IHtmlTagParseContext>);

    // fn add_operation_to_ooo_log(&self, operation: String);
    // fn get_ooo(&self) -> Vec<String>;

    fn log_error(&self, error: RustHtmlError);
    fn log_info(&self, info: String);
}
