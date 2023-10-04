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
        self.real_context.get_is_raw_tokenstream()
    }

    fn get_should_panic_or_return_error(self: &Self) -> bool {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_should_panic_or_return_error));
        self.real_context.get_should_panic_or_return_error()
    }

    fn get_model_type_name(self: &Self) -> String {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_model_type_name));
        self.real_context.get_model_type_name()
    }

    fn get_model_type(self: &Self) -> Vec<TokenTree> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_model_type));
        self.real_context.get_model_type()
    }

    fn set_model_type(self: &Self, value: Option<Vec<TokenTree>>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_model_type));
        self.real_context.set_model_type(value);
    }

    fn try_get_param_string(self: &Self, key: &str) -> Option<String> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::try_get_param_string));
        self.real_context.try_get_param_string(key)
    }

    fn get_param_string(self: &Self, key: &str) -> Result<String, RustHtmlError> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_param_string));
        self.real_context.get_param_string(key)
    }

    fn get_functions_section(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_functions_section));
        self.real_context.get_functions_section()
    }

    fn get_struct_section(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_struct_section));
        self.real_context.get_struct_section()
    }

    fn get_impl_section(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_impl_section));
        self.real_context.get_impl_section()
    }

    fn get_model_ident(self: &Self) -> Option<TokenStream> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_model_ident));
        self.real_context.get_model_ident()
    }

    fn htmltag_scope_stack_push(self: &Self, s: String) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::htmltag_scope_stack_push), s));
        self.real_context.htmltag_scope_stack_push(s);
    }

    fn mut_punct_scope_stack(self: &Self) -> std::cell::RefMut<Vec<char>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_punct_scope_stack));
        self.real_context.mut_punct_scope_stack()
    }

    fn mut_use_statements(self: &Self) -> std::cell::RefMut<Vec<TokenStream>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_use_statements));
        self.real_context.mut_use_statements()
    }

    fn mut_inject_statements(self: &Self) -> std::cell::RefMut<Vec<TokenStream>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_inject_statements));
        self.real_context.mut_inject_statements()
    }

    fn get_inject_statements_stream(self: &Self) -> proc_macro2::TokenStream {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_inject_statements_stream));
        self.real_context.get_inject_statements_stream()
    }

    fn mut_params(self: &Self) -> std::cell::RefMut<std::collections::HashMap<String, String>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::mut_params));
        self.real_context.mut_params()
    }

    fn get_environment_name(self: &Self) -> String {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_environment_name));
        self.real_context.get_environment_name()
    }

    fn get_raw(&self) -> String {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_raw));
        self.real_context.get_raw()
    }

    fn set_raw(self: &Self, value: String) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::set_raw), value));
        self.real_context.set_raw(value);
    }

    fn get_section(self: &Self, name: &String) -> Option<TokenStream> {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::get_section), name));
        self.real_context.get_section(name)
    }

    fn set_section(self: &Self, name: String, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log(format!("{}({})", nameof_member_fn!(Self::set_section), name));
        self.real_context.set_section(name, value);
    }

    fn set_functions_section(self: &Self, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_functions_section));
        self.real_context.set_functions_section(value);
    }

    fn set_impl_section(self: &Self, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_impl_section));
        self.real_context.set_impl_section(value);
    }

    fn set_struct_section(self: &Self, value: Option<TokenStream>) {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::set_struct_section));
        self.real_context.set_struct_section(value);
    }

    fn get_directives(self: &Self) -> Vec<Rc<dyn IRustHtmlDirective>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_directives));
        self.real_context.get_directives()
    }

    fn try_get_directive(self: &Self, name: String) -> Option<Rc<dyn IRustHtmlDirective>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::try_get_directive));
        self.real_context.try_get_directive(name)
    }

    fn get_tag_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlTagParsed>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_tag_parsed_handler));
        self.real_context.get_tag_parsed_handler()
    }

    fn get_node_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlNodeParsed>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_node_parsed_handler));
        self.real_context.get_node_parsed_handler()
    }

    fn get_preprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_preprocessors));
        self.real_context.get_preprocessors()
    }

    fn get_postprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_postprocessors));
        self.real_context.get_postprocessors()
    }

    fn get_rust_preprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_rust_preprocessors));
        self.real_context.get_rust_preprocessors()
    }

    fn get_rust_postprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>> {
        self.add_operation_to_ooo_log_str(nameof_member_fn!(Self::get_rust_postprocessors));
        self.real_context.get_rust_postprocessors()
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