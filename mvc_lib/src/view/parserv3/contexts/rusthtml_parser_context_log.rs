use std::rc::Rc;
use std::cell::RefCell;

use core_lib::{asyncly::icancellation_token::ICancellationToken, impl_with_logging, sys::call_tracker::CallstackTracker};
use proc_macro2::{TokenStream, TokenTree};

use crate::view::{parserv3::{converters::{inode_parsed::IHtmlNodeParsed, irust_processor::IRustProcessor, irusthtml_processor::IRustHtmlProcessor, itag_parsed::IHtmlTagParsed}, directives::irusthtml_directive::IRustHtmlDirective}, rusthtml::{rusthtml_error::RustHtmlError, rusthtml_token::RustHtmlToken}};

use super::{ihtml_tag_parse_context::IHtmlTagParseContext, irusthtml_parser_context::IRustHtmlParserContext, rusthtml_output_buffer::RustHtmlTokenBuffer};


// Usage of the macro to define the struct and implement the trait with logging
pub struct RustHtmlParserContextLog {
    order_of_operations: RefCell<Vec<String>>,
    real_context: Rc<dyn IRustHtmlParserContext>,
}

impl RustHtmlParserContextLog {
    pub fn new(real_context: Rc<dyn IRustHtmlParserContext>) -> Self {
        Self {
            order_of_operations: RefCell::new(vec![]),
            real_context,
        }
    }
}

impl_with_logging!(
    IRustHtmlParserContext,
    RustHtmlParserContextLog,
    real_context,
    fn get_is_raw_tokenstream(&self) -> bool;
    fn get_model_type_name(&self) -> String;
    fn get_model_type_stream(&self) -> TokenStream;
    fn get_model_type(&self) -> Vec<TokenTree>;
    fn set_model_type(&self, value: Option<Vec<TokenTree>>);
    fn try_get_param_string(&self, key: &str) -> Option<String>;
    fn get_param_string(&self, key: &str) -> Result<String, RustHtmlError>;
    fn get_functions_section(&self) -> Option<TokenStream>;
    fn get_struct_section(&self) -> Option<TokenStream>;
    fn get_impl_section(&self) -> Option<TokenStream>;
    fn get_model_ident(&self) -> Option<TokenStream>;
    fn htmltag_scope_stack_push(&self, s: String);
    fn mut_punct_scope_stack(&self) -> std::cell::RefMut<Vec<char>>;
    fn push_use_statements(&self, rust: TokenStream);
    fn push_inject_statements(&self, rust: TokenStream);
    fn get_inject_statements_stream(&self) -> proc_macro2::TokenStream;
    fn mut_params(&self) -> std::cell::RefMut<std::collections::HashMap<String, String>>;
    fn get_environment_name(&self) -> String;
    fn get_raw(&self) -> String;
    fn set_raw(&self, value: String);
    fn get_section(&self, name: &String) -> Option<TokenStream>;
    fn set_section(&self, name: String, value: Option<TokenStream>);
    fn set_functions_section(&self, value: Option<TokenStream>);
    fn set_impl_section(&self, value: Option<TokenStream>);
    fn set_struct_section(&self, value: Option<TokenStream>);
    fn get_directives(&self) -> Vec<Rc<dyn IRustHtmlDirective>>;
    fn try_get_directive(&self, name: String) -> Option<Rc<dyn IRustHtmlDirective>>;
    fn get_tag_parsed_handler(&self) -> Vec<Rc<dyn IHtmlTagParsed>>;
    fn get_node_parsed_handler(&self) -> Vec<Rc<dyn IHtmlNodeParsed>>;
    fn get_preprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>>;
    fn get_postprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>>;
    fn get_rust_preprocessors(&self) -> Vec<Rc<dyn IRustProcessor>>;
    fn get_rust_postprocessors(&self) -> Vec<Rc<dyn IRustProcessor>>;
    fn htmltag_scope_stack_pop(&self) -> Option<String>;
    fn push_inject_statements_rshtml(&self, rust: Vec<RustHtmlToken>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>);
    fn get_use_statements_stream(&self) -> proc_macro2::TokenStream;
    fn get_max_call_stack_count(&self) -> usize;
    fn check_call_stack_count(&self) -> Result<(), RustHtmlError>;
    fn get_call_stack(&self) -> &CallstackTracker;
    fn push_html_tag_parse_context(&self, tag: Rc<dyn IHtmlTagParseContext>);
    fn get_is_in_html_mode(&self) -> bool;
    fn push_is_in_html_mode(&self, v: bool);
    fn pop_is_in_html_mode(&self) -> bool;
    fn push_output_buffer(&self, buffer: RustHtmlTokenBuffer);
    fn pop_output_buffer(&self) -> Option<RustHtmlTokenBuffer>;
    fn get_output_buffer(&self) -> Option<RustHtmlTokenBuffer>;
    fn push_output_token(&self, token: RustHtmlToken) -> Result<(), RustHtmlError>;
    fn push_output_tokens(&self, token: &[RustHtmlToken]) -> Result<(), RustHtmlError>;
    fn log_error(&self, error: RustHtmlError);
    fn log_info(&self, info: String);
);