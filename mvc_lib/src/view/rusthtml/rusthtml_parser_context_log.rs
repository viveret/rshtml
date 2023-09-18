use std::rc::Rc;
use std::cell::RefCell;

use core_macro_lib::nameof_member_fn;
use proc_macro2::{TokenTree, TokenStream};

use super::directives::irusthtml_directive::IRustHtmlDirective;
use super::irust_processor::IRustProcessor;
use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::irusthtml_processor::IRustHtmlProcessor;
use super::node_helpers::inode_parsed::IHtmlNodeParsed;
use super::rusthtml_error::RustHtmlError;
use super::tag_helpers::itag_parsed::IHtmlTagParsed;


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

    pub fn add_operation_to_ooo_log_str(&self, operation: &str) -> bool {
        self.order_of_operations.borrow_mut().push(operation.to_string());
        true
    }
}

impl IRustHtmlParserContext for RustHtmlParserContextLog {
    fn get_is_raw_tokenstream(self: &Self) -> bool {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_is_raw_tokenstream));
        false
    }

    fn get_should_panic_or_return_error(self: &Self) -> bool {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_should_panic_or_return_error));
        false
    }

    fn get_model_type_name(self: &Self) -> String {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_model_type_name));
        "test".to_string()
    }

    fn get_model_type(self: &Self) -> Vec<TokenTree> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_model_type));
        vec![]
    }

    fn set_model_type(self: &Self, value: Option<Vec<TokenTree>>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_model_type));
    }

    fn try_get_param_string(self: &Self, key: &str) -> Option<String> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::try_get_param_string));
        None
    }

    fn get_param_string(self: &Self, key: &str) -> Result<String, RustHtmlError> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_param_string));
        Ok("test".to_string())
    }

    fn get_functions_section(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_functions_section));
        None
    }

    fn get_struct_section(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_struct_section));
        None
    }

    fn get_impl_section(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_impl_section));
        None
    }

    fn get_model_ident(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_model_ident));
        None
    }

    fn htmltag_scope_stack_push(self: &Self, s: String) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::htmltag_scope_stack_push), s));
        self.real_context.htmltag_scope_stack_push(s);
    }

    fn mut_punct_scope_stack(self: &Self) -> std::cell::RefMut<Vec<char>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_punct_scope_stack));
        todo!()
    }

    fn mut_use_statements(self: &Self) -> std::cell::RefMut<Vec<TokenStream>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_use_statements));
        todo!()
    }

    fn mut_inject_statements(self: &Self) -> std::cell::RefMut<Vec<TokenStream>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_inject_statements));
        todo!()
    }

    fn get_inject_statements_stream(self: &Self) -> proc_macro2::TokenStream {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_inject_statements_stream));
        proc_macro2::TokenStream::new()
    }

    fn mut_params(self: &Self) -> std::cell::RefMut<std::collections::HashMap<String, String>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_params));
        todo!()
    }

    fn get_environment_name(self: &Self) -> String {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_environment_name));
        "test".to_string()
    }

    fn get_raw(&self) -> String {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_raw));
        "test".to_string()
    }

    fn set_raw(self: &Self, value: String) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::set_raw), value));
    }

    fn get_section(self: &Self, name: &String) -> Option<TokenStream> {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::get_section), name));
        None
    }

    fn set_section(self: &Self, name: String, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::set_section), name));
    }

    fn set_functions_section(self: &Self, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_functions_section));
    }

    fn set_impl_section(self: &Self, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_impl_section));
    }

    fn set_struct_section(self: &Self, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_struct_section));
    }

    fn get_directives(self: &Self) -> Vec<Rc<dyn IRustHtmlDirective>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_directives));
        vec![]
    }

    fn try_get_directive(self: &Self, name: String) -> Option<Rc<dyn IRustHtmlDirective>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::try_get_directive));
        None
    }

    fn get_tag_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlTagParsed>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_tag_parsed_handler));
        vec![]
    }

    fn get_node_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlNodeParsed>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_node_parsed_handler));
        vec![]
    }

    fn get_preprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_preprocessors));
        vec![]
    }

    fn get_postprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_postprocessors));
        vec![]
    }

    fn get_rust_preprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_rust_preprocessors));
        vec![]
    }

    fn get_rust_postprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_rust_postprocessors));
        vec![]
    }

    fn htmltag_scope_stack_pop(self: &Self) -> Option<String> {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::htmltag_scope_stack_pop), "test"));
        self.real_context.htmltag_scope_stack_pop()
    }

    fn add_operation_to_ooo_log(self: &Self, operation: String) {
        self.order_of_operations.borrow_mut().push(operation);
    }

    fn get_ooo(self: &Self) -> Vec<String> {
        self.order_of_operations.borrow().clone()
    }
}