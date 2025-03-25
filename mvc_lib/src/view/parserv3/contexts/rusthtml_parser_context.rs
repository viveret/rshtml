// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use core_lib::sys::call_tracker::CallstackTracker;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

use crate::view::parserv3::converters::inode_parsed::IHtmlNodeParsed;
use crate::view::parserv3::converters::irust_processor::IRustProcessor;
use crate::view::parserv3::converters::irusthtml_processor::IRustHtmlProcessor;
use crate::view::parserv3::converters::itag_parsed::IHtmlTagParsed;
use crate::view::parserv3::converters::rusthtml_parser_sub_processors::RustHtmlParserSubProcessors;
use crate::view::parserv3::directives::else_directive::ElseDirective;
use crate::view::parserv3::directives::else_if_directive::ElseIfDirective;
use crate::view::parserv3::directives::for_directive::ForDirective;
use crate::view::parserv3::directives::html_directive::HtmlDirective;
use crate::view::parserv3::directives::htmlfile_directive::HtmlFileDirective;
use crate::view::parserv3::directives::if_directive::IfDirective;
use crate::view::parserv3::directives::inject_directive::InjectDirective;
use crate::view::parserv3::directives::irusthtml_directive::IRustHtmlDirective;
use crate::view::parserv3::directives::lang_directive::LangDirective;
use crate::view::parserv3::directives::let_directive::LetDirective;
use crate::view::parserv3::directives::markdown_directive::MarkdownDirective;
use crate::view::parserv3::directives::markdownfile_const_directive::MarkdownFileConstDirective;
use crate::view::parserv3::directives::markdownfile_nocache_directive::MarkdownFileNoCacheDirective;
use crate::view::parserv3::directives::model_directive::ModelDirective;
use crate::view::parserv3::directives::name_directive::NameDirective;
use crate::view::parserv3::directives::rusthtmlfile_directive::RustHtmlFileDirective;
use crate::view::parserv3::directives::rusthtmlfile_nocache_directive::RustHtmlFileNoCacheDirective;
use crate::view::parserv3::directives::section_functions_directive::FunctionsSectionDirective;
use crate::view::parserv3::directives::section_impl_directive::ImplSectionDirective;
use crate::view::parserv3::directives::section_struct_directive::StructSectionDirective;
use crate::view::parserv3::directives::use_directive::UseDirective;
use crate::view::parserv3::directives::viewstart_directive::ViewStartDirective;
use crate::view::parserv3::directives::while_directive::WhileDirective;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::rusthtml_token::RustHtmlToken;

use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::rusthtml_output_buffer::{RustHtmlTokenBuffer, RustHtmlTokenBufferBuffer};

pub struct RustHtmlParserContext {
    // the current stack trace.
    pub call_stack: CallstackTracker,

    // whether or not the RustHtml code is raw tokenstream.
    pub is_raw_tokenstream: bool,

    // whether or not to panic or return an error when an error occurs.
    pub should_panic_or_return_error: bool,

    // the current scope stack for punctuation.
    pub punctuation_scope_stack: RefCell<Vec<char>>,
    // the current scope stack for HTML tags.
    pub htmltag_scope_stack: RefCell<Vec<String>>,
    // the current scope stack for parsing HTML tags
    pub htmltag_parse_scope_stack: RefCell<Vec<Rc<dyn IHtmlTagParseContext>>>,

    // current parameters in the global scope used while parsing.
    pub params: RefCell<HashMap<String, String>>,
    // sections labeled used while parsing.
    pub sections: RefCell<HashMap<String, TokenStream>>,
    // the functions section of the RustHtml code.
    pub functions_section: RefCell<Option<TokenStream>>,
    // the struct section of the RustHtml code.
    pub struct_section: RefCell<Option<TokenStream>>,
    // the impl section of the RustHtml code.
    pub impl_section: RefCell<Option<TokenStream>>,
    // the model type of the RustHtml code.
    pub model_type: RefCell<Option<Vec<TokenTree>>>,
    // the use statements automatically included in the output code.
    pub implicit_use_statements: TokenStream,
    // the use statements of the RustHtml code.
    pub use_statements: RefCell<Vec<TokenStream>>,
    // the inject statements of the RustHtml code.
    pub inject_statements: RefCell<Vec<TokenStream>>,

    // the raw RustHtml code.
    pub raw: RefCell<String>,

    // whether or not the RustHtml code has included a view start.
    pub has_included_view_start: RefCell<bool>,

    // the name of the environment while parsing and "compiling" the RustHtml code.
    pub environment_name: String,

    // the directives available to the parser.
    pub directives: Vec<Rc<dyn IRustHtmlDirective>>,

    pub sub_processors: RustHtmlParserSubProcessors,

    // stack of the current processing state of the parser.
    // this is calculated by taking the hash of the stream / vec of token trees.
    // if the hash is the same, then the processing state is the same,
    // if the hash is different, then the processing state is different,
    // and if the hash is repeated, then the processing state is in a recursive loop or no longer simplifiable.
    pub rusthtml_processing_state_stack: RefCell<Vec<u32>>,
    pub rust_processing_state_stack: RefCell<Vec<u32>>,

    // the stack of whether or not the parser is in HTML mode.
    pub is_in_html_mode_stack: RefCell<Vec<bool>>,

    // the stack of the output buffer where the RustHtml code is being written to.
    pub output_buffer_stack: RustHtmlTokenBufferBuffer,
}

impl RustHtmlParserContext {
    // creates a new RustHtmlParser.
    // should_panic_or_return_error: whether or not to panic or return an error when an error occurs.
    // environment_name: the name of the environment to use.
    // returns: a new RustHtmlParser.
    pub fn new(
        is_raw_tokenstream: bool,
        should_panic_or_return_error: bool,
        environment_name: String,
    ) -> Self {
        Self {
            call_stack: CallstackTracker::new(),
            is_raw_tokenstream: is_raw_tokenstream,
            should_panic_or_return_error: should_panic_or_return_error,
            htmltag_scope_stack: RefCell::new(vec![]),
            htmltag_parse_scope_stack: RefCell::new(vec![]),
            punctuation_scope_stack: RefCell::new(vec![]),
            params: RefCell::new(HashMap::new()),
            sections: RefCell::new(HashMap::new()),
            functions_section: RefCell::new(None),
            struct_section: RefCell::new(None),
            impl_section: RefCell::new(None),
            model_type: RefCell::new(None),
            implicit_use_statements: quote! {
                use as_any::Downcast;
                use std::any::Any;
                use std::borrow::Cow;
                use std::cell::RefCell;
                use std::collections::HashMap;
                use std::error::Error;
                use std::rc::Rc;
                use std::io::Read;
                use std::ops::Deref;
                use std::sync::{Arc, RwLock};

                use chrono::{DateTime, TimeZone, Utc};
                use proc_macro2::TokenStream;

                use core_macro_lib::{ * };
                
                use mvc_lib::core::type_info::TypeInfo;
                use mvc_lib::core::html_buffer::IHtmlBuffer;
                use mvc_lib::core::html_buffer::HtmlBuffer;
                use mvc_lib::contexts::controller_context::IControllerContext;
                use mvc_lib::contexts::view_context::IViewContext;
                use mvc_lib::model_binder::imodel::IModel;
                use mvc_lib::model_binder::imodel::AnyIModel;
                use mvc_lib::services::service_scope::ServiceScope;
                use mvc_lib::services::service_descriptor::ServiceDescriptor;
                use mvc_lib::services::service_collection::IServiceCollection;
                use mvc_lib::services::service_collection::ServiceCollection;
                use mvc_lib::view::rusthtml::helpers::ihtml_helpers::IHtmlHelpers;
                use mvc_lib::view::rusthtml::helpers::html_helpers::HtmlHelpers;
                use mvc_lib::view::rusthtml::helpers::irender_helpers::IRenderHelpers;
                use mvc_lib::view::rusthtml::helpers::render_helpers::RenderHelpers;
                use mvc_lib::view::rusthtml::html_string::HtmlString;
                use mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError;
                use mvc_lib::view::iview::IView;
                use mvc_lib::routing::iurl_helpers::IUrlHelpers;
                use mvc_lib::routing::url_helpers::UrlHelpers;
                use mvc_lib::routing::route_values_builder::RouteValuesBuilder;
                use mvc_lib::services::service_collection::ServiceCollectionExtensions;
            },
            use_statements: RefCell::new(vec![]),
            inject_statements: RefCell::new(vec![
                quote::quote!{
                    let render = RenderHelpers::new(view_context, services);
                    // let html = HtmlHelpers::<#model_type>::new(view_context, services);
                    let url = UrlHelpers::new(view_context, services);
                }.into(),
            ]),
            raw: RefCell::new(String::new()),
            has_included_view_start: RefCell::new(false),
            environment_name: environment_name,
            directives: vec![
                // Low level language constructs.
                Rc::new(LetDirective::new()),
                Rc::new(ForDirective::new()),
                Rc::new(WhileDirective::new()),
                Rc::new(IfDirective::new()),
                Rc::new(ElseIfDirective::new()),
                Rc::new(ElseDirective::new()),
                Rc::new(UseDirective::new()),

                // Higher level language constructs.
                // Rc::new(AwaitDirective::new()),

                // directives for this view or RustHtml code.
                Rc::new(LangDirective::new()),
                Rc::new(ModelDirective::new()),
                Rc::new(NameDirective::new()),
                Rc::new(ViewStartDirective::new()),
                Rc::new(InjectDirective::new()),

                // html directives.
                // Rc::new(HtmlFormDirective::new()),

                // sections for this view that are not for rendering.
                Rc::new(FunctionsSectionDirective::new()),
                Rc::new(StructSectionDirective::new()),
                Rc::new(ImplSectionDirective::new()),

                // Style and script directives.
                // Rc::new(CssDirective::new()),
                // Rc::new(LessDirective::new()),
                // Rc::new(JsDirective::new()),
                // Rc::new(TsDirective::new()),
                
                // External formats and files (e.g. Markdown, HTML, RustHtml, etc.)
                Rc::new(HtmlDirective::new()),
                Rc::new(HtmlFileDirective::new()),
                Rc::new(RustHtmlFileDirective::new()),
                Rc::new(RustHtmlFileNoCacheDirective::new()),
                Rc::new(MarkdownDirective::new()),
                Rc::new(MarkdownFileConstDirective::new()),
                Rc::new(MarkdownFileNoCacheDirective::new()),
            ],
            sub_processors: RustHtmlParserSubProcessors::new(),
            rusthtml_processing_state_stack: RefCell::new(vec![]),
            rust_processing_state_stack: RefCell::new(vec![]),
            is_in_html_mode_stack: RefCell::new(vec![]),
            output_buffer_stack: RustHtmlTokenBufferBuffer::new(),
        }
    }

    pub fn is_ok(&self) -> bool {
        true
    }
}

impl IRustHtmlParserContext for RustHtmlParserContext {
    fn get_model_type_name(&self) -> String {
        let mut s = String::new();
        for type_part in self.get_model_type() {
            s.push_str(&type_part.to_string());
        }
        s
    }

    fn get_model_type_stream(&self) -> TokenStream {
        TokenStream::from_iter(self.get_model_type())
    }

    fn get_model_type(&self) -> Vec<TokenTree> {
        self.model_type.borrow().clone().unwrap_or(vec![])
    }

    // try to get a parameter value as a string.
    // key: the key of the parameter.
    fn try_get_param_string(&self, key: &str) -> Option<String> {
        match self.params.borrow().get(&key.to_string()) {
            Some(str_val) => {
                let s = snailquote::unescape(str_val).expect("couldn't unescape string");
                Some(s)
            },
            None => {
                None
            }
        }
    }

    fn get_param_string(&self, key: &str) -> Result<String, RustHtmlError> {
        match self.params.borrow().get(&key.to_string()) {
            Some(str_val) => {
                let s = snailquote::unescape(str_val).expect("couldn't unescape string");
                Ok(s)
            },
            None => {
                return Err(RustHtmlError::from_string(
                    format!("missing param '@{}' in rusthtml (keys: {})", key, self.params.borrow().keys().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
                ));
            }
        }
    }

    fn get_functions_section(&self) -> Option<TokenStream> {
        if let Some(has_functions) = self.functions_section.borrow().as_ref() {
            Some(has_functions.clone())
        } else {
            None
        }
    }

    fn get_struct_section(&self) -> Option<TokenStream> {
        if let Some(has_struct) = self.struct_section.borrow().as_ref() {
            Some(has_struct.clone())
        } else {
            None
        }
    }

    fn get_impl_section(&self) -> Option<TokenStream> {
        if let Some(has_impl) = self.impl_section.borrow().as_ref() {
            Some(has_impl.clone())
        } else {
            None
        }
    }

    fn get_model_ident(&self) -> Option<TokenStream> {
        if let Some(has_model) = self.model_type.borrow().as_ref() {
            Some(TokenStream::from_iter(has_model.clone()))
        } else {
            None
        }
    }

    fn set_model_type(&self, value: Option<Vec<TokenTree>>) {
        *self.model_type.borrow_mut() = value;
    }

    fn htmltag_scope_stack_push(&self, s: String) {
        self.htmltag_scope_stack.borrow_mut().push(s);
    }

    fn htmltag_scope_stack_pop(&self) -> Option<String> {
        self.htmltag_scope_stack.borrow_mut().pop()
    }

    fn mut_punct_scope_stack(&self) -> RefMut<Vec<char>> {
        self.punctuation_scope_stack.borrow_mut()
    }

    fn push_use_statements(&self, rshtml: TokenStream) {
        self.use_statements.borrow_mut().push(rshtml)
    }

    fn get_implicit_use_statements(&self) -> proc_macro2::TokenStream {
        self.implicit_use_statements.clone()
    }

    fn get_use_statements_stream(&self) -> proc_macro2::TokenStream {
        let tokens = 
            self.use_statements.borrow()
                .iter()
                .map(|s| s.clone().into_iter())
                .flatten()
                .collect::<Vec<TokenTree>>();
        proc_macro2::TokenStream::from_iter(
            tokens
        )
    }

    fn mut_params(&self) -> RefMut<HashMap<String, String>> {
        self.params.borrow_mut()
    }

    fn get_environment_name(&self) -> String {
        self.environment_name.clone()
    }

    fn get_raw(&self) -> String {
        self.raw.borrow().clone()
    }

    fn set_raw(&self, value: String) {
        *self.raw.borrow_mut() = value;
    }

    fn set_functions_section(&self, value: Option<TokenStream>) {
        *self.functions_section.borrow_mut() = value;
    }

    fn set_impl_section(&self, value: Option<TokenStream>) {
        *self.impl_section.borrow_mut() = value;
    }

    fn set_struct_section(&self, value: Option<TokenStream>) {
        *self.struct_section.borrow_mut() = value;
    }

    fn get_directives(&self) -> Vec<Rc<dyn IRustHtmlDirective>> {
        self.directives.clone()
    }

    fn try_get_directive(&self, name: String) -> Option<Rc<dyn IRustHtmlDirective>> {
        let x = self.directives
            .iter()
            .filter(|x| x.matches(&name))
            .take(1)
            .cloned()
            .collect::<Vec<Rc<dyn IRustHtmlDirective>>>();

        let x = x.get(0);

        if let Some(x) = x {
            Some(x.clone())
        } else {
            // println!("try_get_directive not found: {}", name);
            None
        }
    }

    fn get_is_raw_tokenstream(&self) -> bool {
        self.is_raw_tokenstream
    }

    fn get_tag_parsed_handler(&self) -> Vec<Rc<dyn IHtmlTagParsed>> {
        self.sub_processors.tag_parsed_handlers.clone()
    }

    fn get_node_parsed_handler(&self) -> Vec<Rc<dyn IHtmlNodeParsed>> {
        self.sub_processors.node_parsed_handlers.clone()
    }

    fn get_section(&self, name: &String) -> Option<TokenStream> {
        self.sections.borrow().get(name).cloned()
    }

    fn set_section(&self, name: String, value: Option<TokenStream>) {
        if let Some(v) = value {
            self.sections.borrow_mut().insert(name, v);
        } else {
            self.sections.borrow_mut().remove(&name);
        }
    }

    fn push_inject_statements(&self, rust: TokenStream) {
        self.inject_statements.borrow_mut().push(rust);
    }

    fn get_inject_statements_stream(&self) -> proc_macro2::TokenStream {
        let mut model_based_injections = vec![];

        if let Some(model_type) = self.model_type.borrow().as_ref() {
            if model_type.len() > 0 {
                    let model_type_stream = 
                    proc_macro2::TokenStream::from(
                        model_type.into_iter()
                            .cloned()
                            .collect::<TokenStream>()
                    );
                    model_based_injections.push(quote::quote!{
                        let html = HtmlHelpers::<#model_type_stream>::new(view_context, services);
                    });
            } else {
                panic!("model type must be a single type, not {}: {}", model_type.len(), self.get_model_type_name());
            }
        } else {
            model_based_injections.push(quote::quote!{
                let html = HtmlHelpers::<AnyIModel>::new(view_context, services);
            });
        }

        model_based_injections.push(
            proc_macro2::TokenStream::from(
                TokenStream::from_iter(
                    self.inject_statements.borrow()
                        .iter()
                        .cloned()
                        .map(|s| s.into_iter())
                        .flatten()
                )
            )
        );
        
        proc_macro2::TokenStream::from_iter(
            model_based_injections
                .iter()
                .cloned()
                .map(|s| s.into_iter())
                .flatten()
        )
    }

    fn get_preprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.sub_processors.preprocessors.clone()
    }

    fn get_postprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.sub_processors.postprocessors.clone()
    }

    fn get_rust_preprocessors(&self) -> Vec<Rc<dyn IRustProcessor>> {
        self.sub_processors.rust_preprocessors.clone()
    }

    fn get_rust_postprocessors(&self) -> Vec<Rc<dyn IRustProcessor>> {
        self.sub_processors.rust_postprocessors.clone()
    }

    fn get_call_stack(&self) -> &CallstackTracker {
        &self.call_stack
    }
    
    fn get_max_call_stack_count(&self) -> usize {
        30
    }

    fn check_call_stack_count(&self) -> Result<(), RustHtmlError> {
        if self.call_stack.len() > self.get_max_call_stack_count() {
            let callstack = self.call_stack.to_string();
            return Err(RustHtmlError::from_string(format!("call stack count is greater than the max call stack count of {}. call stack: {}", self.get_max_call_stack_count(), callstack)));
        }
        Ok(())
    }

    fn push_html_tag_parse_context(&self, tag: Rc<dyn IHtmlTagParseContext>) {
        self.htmltag_parse_scope_stack.borrow_mut().push(tag);
    }

    fn get_is_in_html_mode(&self) -> bool {
        self.is_in_html_mode_stack.borrow().last().unwrap_or(&true).clone()
    }

    fn push_is_in_html_mode(&self, v: bool) {
        self.is_in_html_mode_stack.borrow_mut().push(v);
    }

    fn pop_is_in_html_mode(&self) -> bool {
        self.is_in_html_mode_stack.borrow_mut().pop().unwrap_or(true)
    }

    fn push_output_buffer(&self, buffer: RustHtmlTokenBuffer) {
        // self.output_buffer_stack.borrow_mut().push(buffer);
        self.output_buffer_stack.push(buffer);
    }

    fn pop_output_buffer(&self) -> Option<RustHtmlTokenBuffer> {
        // if self.output_buffer_stack.borrow().len() > 1 {
        //     self.output_buffer_stack.borrow_mut().pop()
        // } else {
        //     None
        // }
        self.output_buffer_stack.pop()
    }

    fn get_output_buffer(&self) -> Option<RustHtmlTokenBuffer> {
        // self.output_buffer_stack.borrow().last().cloned()
        self.output_buffer_stack.last()
    }

    fn push_output_token(&self, token: RustHtmlToken) -> Result<(), RustHtmlError> {
        if let Some(buffer) = self.get_output_buffer() {
            // buffer.borrow_mut().push(token);
            buffer.push(token);
            Ok(())
        } else {
            Err(RustHtmlError::from_string(format!("no output buffer to push token ({:?}) to", token)))
        }
    }

    fn push_output_tokens(&self, token: &[RustHtmlToken]) -> Result<(), RustHtmlError> {
        if let Some(buffer) = self.get_output_buffer() {
            // buffer.borrow_mut().extend(token.iter().cloned());
            // let items: Vec<RustHtmlToken> = token.iter().cloned().collect();
            buffer.extend_buffer(&buffer);//items.as_slice());
            Ok(())
        } else {
            Err(RustHtmlError::from_string(format!("no output buffer to push tokens ({:?}) to", token)))
        }
    }
    
    fn log_error(&self, error: RustHtmlError) {
        // self.log.push(error.to_string());
    }
    
    fn log_info(&self, info: String) {
        // todo!()
    }
    
    fn insert_params(&self, key: String, value: String) {
        self.params.borrow_mut().insert(key, value);
    }
}


pub struct MockRustHtmlParserContext {

}

impl MockRustHtmlParserContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRustHtmlParserContext for MockRustHtmlParserContext {
    fn get_call_stack(&self) -> &CallstackTracker {
        todo!()
    }

    fn get_max_call_stack_count(&self) -> usize {
        0
    }

    fn check_call_stack_count(&self) -> Result<(), RustHtmlError> {
        Ok(())
    }

    fn get_is_in_html_mode(&self) -> bool {
        false
    }

    fn push_is_in_html_mode(&self, v: bool) {
        
    }

    fn pop_is_in_html_mode(&self) -> bool {
        false
    }

    fn push_output_buffer(&self, buffer: RustHtmlTokenBuffer) {
        
    }

    fn pop_output_buffer(&self) -> Option<RustHtmlTokenBuffer> {
        None
    }

    fn get_output_buffer(&self) -> Option<RustHtmlTokenBuffer> {
        None
    }

    fn push_output_token(&self, token: RustHtmlToken) -> Result<(), RustHtmlError> {
        Ok(())
    }

    fn push_output_tokens(&self, token: &[RustHtmlToken]) -> Result<(), RustHtmlError> {
        Ok(())
    }

    fn get_is_raw_tokenstream(&self) -> bool {
        false
    }

    fn get_model_type_name(&self) -> String {
        String::new()
    }

    fn get_model_type_stream(&self) -> TokenStream {
        TokenStream::new()
    }

    fn get_model_type(&self) -> Vec<TokenTree> {
        vec![]
    }

    fn set_model_type(&self, value: Option<Vec<TokenTree>>) {
        
    }

    fn try_get_param_string(&self, key: &str) -> Option<String> {
        None
    }

    fn get_param_string(&self, key: &str) -> Result<String, RustHtmlError> {
        Ok(String::new())
    }

    fn get_functions_section(&self) -> Option<TokenStream> {
        None
    }

    fn get_struct_section(&self) -> Option<TokenStream> {
        None
    }

    fn get_impl_section(&self) -> Option<TokenStream> {
        None
    }

    fn get_model_ident(&self) -> Option<TokenStream> {
        None
    }

    fn htmltag_scope_stack_push(&self, s: String) {
    }

    fn htmltag_scope_stack_pop(&self) -> Option<String> {
        None
    }

    fn mut_punct_scope_stack(&self) -> RefMut<Vec<char>> {
        todo!()
    }

    fn push_use_statements(&self, rust: TokenStream) {
    }

    fn get_implicit_use_statements(&self) -> proc_macro2::TokenStream {
        TokenStream::new()
    }

    fn get_use_statements_stream(&self) -> proc_macro2::TokenStream {
        TokenStream::new()
    }

    fn push_inject_statements(&self, rust: TokenStream) {
    }

    fn get_inject_statements_stream(&self) -> proc_macro2::TokenStream {
        TokenStream::new()
    }

    fn mut_params(&self) -> RefMut<HashMap<String, String>> {
        todo!()
    }

    fn insert_params(&self, key: String, value: String) {
        
    }

    fn get_environment_name(&self) -> String {
        String::new()
    }

    fn get_raw(&self) -> String {
        String::new()
    }

    fn set_raw(&self, value: String) {
        
    }

    fn get_section(&self, name: &String) -> Option<TokenStream> {
        None
    }

    fn set_section(&self, name: String, value: Option<TokenStream>) {
    }

    fn set_functions_section(&self, value: Option<TokenStream>) {
    }

    fn set_impl_section(&self, value: Option<TokenStream>) {
    }

    fn set_struct_section(&self, value: Option<TokenStream>) {
    }

    fn get_directives(&self) -> Vec<Rc<dyn IRustHtmlDirective>> {
        vec![]
    }

    fn try_get_directive(&self, name: String) -> Option<Rc<dyn IRustHtmlDirective>> {
        None
    }

    fn get_tag_parsed_handler(&self) -> Vec<Rc<dyn IHtmlTagParsed>> {
        vec![]
    }

    fn get_node_parsed_handler(&self) -> Vec<Rc<dyn IHtmlNodeParsed>> {
        vec![]
    }

    fn get_preprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        vec![]
    }

    fn get_postprocessors(&self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        vec![]
    }

    fn get_rust_preprocessors(&self) -> Vec<Rc<dyn IRustProcessor>> {
        vec![]
    }

    fn get_rust_postprocessors(&self) -> Vec<Rc<dyn IRustProcessor>> {
        vec![]
    }

    fn push_html_tag_parse_context(&self, tag_parse_ctx: Rc<dyn IHtmlTagParseContext>) {
        
    }

    fn log_error(&self, error: RustHtmlError) {
        
    }

    fn log_info(&self, info: String) {
        
    }
}