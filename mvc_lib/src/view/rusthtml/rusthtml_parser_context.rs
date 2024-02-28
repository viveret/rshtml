// based on https://github.com/bodil/typed-html/blob/master/macros/src/lexer.rs
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use core_lib::sys::call_tracker::CallstackTracker;
use proc_macro2::{TokenStream, TokenTree};

use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::processors::post_process_flatten_group_none_delimiter::PostProcessFlattenGroupNoneDelimiter;

use super::directives::else_directive::ElseDirective;
use super::directives::else_if_directive::ElseIfDirective;
use super::directives::for_directive::ForDirective;
use super::directives::html_directive::HtmlDirective;
use super::directives::htmlfile_directive::HtmlFileDirective;
use super::directives::if_directive::IfDirective;
use super::directives::inject_directive::InjectDirective;
use super::directives::irusthtml_directive::IRustHtmlDirective;
use super::directives::lang_directive::LangDirective;
use super::directives::let_directive::LetDirective;
use super::directives::markdown_directive::MarkdownDirective;
use super::directives::markdownfile_const_directive::MarkdownFileConstDirective;
use super::directives::markdownfile_nocache_directive::MarkdownFileNoCacheDirective;
use super::directives::model_directive::ModelDirective;
use super::directives::name_directive::NameDirective;
use super::directives::rusthtmlfile_directive::RustHtmlFileDirective;
use super::directives::rusthtmlfile_nocache_directive::RustHtmlFileNoCacheDirective;
use super::directives::section_functions_directive::FunctionsSectionDirective;
use super::directives::section_impl_directive::ImplSectionDirective;
use super::directives::section_struct_directive::StructSectionDirective;
use super::directives::use_directive::UseDirective;
use super::directives::viewstart_directive::ViewStartDirective;
use super::directives::while_directive::WhileDirective;
use super::ihtml_tag_parse_context::IHtmlTagParseContext;
use super::irust_processor::IRustProcessor;
use super::irusthtml_parser_context::IRustHtmlParserContext;
use super::irusthtml_processor::IRustHtmlProcessor;
use super::node_helpers::environment_node::EnvironmentHtmlNodeParsed;
use super::node_helpers::inode_parsed::IHtmlNodeParsed;
use super::parsers::rusthtmlparser_all::IRustHtmlParserAll;
use super::processors::post_process_combine_static_str::PostProcessCombineStaticStr;
use super::rusthtml_token::RustHtmlToken;
use super::tag_helpers::environment_tag::EnvironmentHtmlTagParsed;
use super::tag_helpers::itag_parsed::IHtmlTagParsed;

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
    // tag parsed handlers.
    pub tag_parsed_handlers: Vec<Rc<dyn IHtmlTagParsed>>,
    // node parsed handlers.
    pub node_parsed_handlers: Vec<Rc<dyn IHtmlNodeParsed>>,

    // preprocessors available to the parser.
    pub preprocessors: Vec<Rc<dyn IRustHtmlProcessor>>,
    // postprocessors available to the parser.
    pub postprocessors: Vec<Rc<dyn IRustHtmlProcessor>>,

    // preprocessors available to the parser.
    pub rust_preprocessors: Vec<Rc<dyn IRustProcessor>>,
    // postprocessors available to the parser.
    pub rust_postprocessors: Vec<Rc<dyn IRustProcessor>>,

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
    pub output_buffer_stack: RefCell<Vec<Rc<RefCell<Vec<RustHtmlToken>>>>>,
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
            use_statements: RefCell::new(vec![
                quote::quote!{
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
                    use mvc_lib::services::service_collection::IServiceCollection;
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
                }.into(),
            ]),
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
            tag_parsed_handlers: vec![
                Rc::new(EnvironmentHtmlTagParsed::new()),
                // Rc::new(DoctypeTagParsed::new()),
            ],
            node_parsed_handlers: vec![
                Rc::new(EnvironmentHtmlNodeParsed::new()),
                // Rc::new(DoctypeNodeParsed::new()),
                // Rc::new(CommentNodeParsed::new()),
                // Rc::new(TextNodeParsed::new()),
                // Rc::new(WhitespaceNodeParsed::new()),
            ],
            preprocessors: vec![],
            postprocessors: vec![
                // Rc::new(PostProcessCombineStaticStr::new()),
            ],
            rust_preprocessors: vec![],
            rust_postprocessors: vec![
                Rc::new(PostProcessFlattenGroupNoneDelimiter::new()),
                Rc::new(PostProcessCombineStaticStr::new()),
            ],
            rusthtml_processing_state_stack: RefCell::new(vec![]),
            rust_processing_state_stack: RefCell::new(vec![]),
            is_in_html_mode_stack: RefCell::new(vec![]),
            output_buffer_stack: RefCell::new(vec![
                Rc::new(RefCell::new(vec![])), // root output buffer
            ]),
        }
    }

    pub fn is_ok(&self) -> bool {
        true
    }
}

impl IRustHtmlParserContext for RustHtmlParserContext {
    fn get_model_type_name(self: &Self) -> String {
        let mut s = String::new();
        for type_part in self.get_model_type() {
            s.push_str(&type_part.to_string());
        }
        s
    }

    fn get_model_type(self: &Self) -> Vec<TokenTree> {
        self.model_type.borrow().clone().unwrap_or(vec![])
    }

    // try to get a parameter value as a string.
    // key: the key of the parameter.
    fn try_get_param_string(self: &Self, key: &str) -> Option<String> {
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

    fn get_param_string(self: &Self, key: &str) -> Result<String, RustHtmlError> {
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

    fn get_functions_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_functions) = self.functions_section.borrow().as_ref() {
            Some(has_functions.clone())
        } else {
            None
        }
    }

    fn get_struct_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_struct) = self.struct_section.borrow().as_ref() {
            Some(has_struct.clone())
        } else {
            None
        }
    }

    fn get_impl_section(self: &Self) -> Option<TokenStream> {
        if let Some(has_impl) = self.impl_section.borrow().as_ref() {
            Some(has_impl.clone())
        } else {
            None
        }
    }

    fn get_model_ident(self: &Self) -> Option<TokenStream> {
        if let Some(has_model) = self.model_type.borrow().as_ref() {
            Some(TokenStream::from_iter(has_model.clone()))
        } else {
            None
        }
    }

    fn set_model_type(self: &Self, value: Option<Vec<TokenTree>>) {
        *self.model_type.borrow_mut() = value;
    }

    fn htmltag_scope_stack_push(self: &Self, s: String) {
        self.htmltag_scope_stack.borrow_mut().push(s);
    }

    fn htmltag_scope_stack_pop(self: &Self) -> Option<String> {
        self.htmltag_scope_stack.borrow_mut().pop()
    }

    fn mut_punct_scope_stack(self: &Self) -> RefMut<Vec<char>> {
        self.punctuation_scope_stack.borrow_mut()
    }

    fn push_use_statements(self: &Self, rshtml: TokenStream) {
        self.use_statements.borrow_mut().push(rshtml)
    }

    fn get_use_statements_stream(self: &Self) -> proc_macro2::TokenStream {
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

    fn mut_params(self: &Self) -> RefMut<HashMap<String, String>> {
        self.params.borrow_mut()
    }

    fn get_environment_name(self: &Self) -> String {
        self.environment_name.clone()
    }

    fn get_raw(&self) -> String {
        self.raw.borrow().clone()
    }

    fn set_raw(self: &Self, value: String) {
        *self.raw.borrow_mut() = value;
    }

    fn set_functions_section(self: &Self, value: Option<TokenStream>) {
        *self.functions_section.borrow_mut() = value;
    }

    fn set_impl_section(self: &Self, value: Option<TokenStream>) {
        *self.impl_section.borrow_mut() = value;
    }

    fn set_struct_section(self: &Self, value: Option<TokenStream>) {
        *self.struct_section.borrow_mut() = value;
    }

    fn get_directives(self: &Self) -> Vec<Rc<dyn IRustHtmlDirective>> {
        self.directives.clone()
    }

    fn try_get_directive(self: &Self, name: String) -> Option<Rc<dyn IRustHtmlDirective>> {
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

    fn get_is_raw_tokenstream(self: &Self) -> bool {
        self.is_raw_tokenstream
    }

    fn get_tag_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlTagParsed>> {
        self.tag_parsed_handlers.clone()
    }

    fn get_node_parsed_handler(self: &Self) -> Vec<Rc<dyn IHtmlNodeParsed>> {
        self.node_parsed_handlers.clone()
    }

    fn get_section(self: &Self, name: &String) -> Option<TokenStream> {
        self.sections.borrow().get(name).cloned()
    }

    fn set_section(self: &Self, name: String, value: Option<TokenStream>) {
        if let Some(v) = value {
            self.sections.borrow_mut().insert(name, v);
        } else {
            self.sections.borrow_mut().remove(&name);
        }
    }

    fn push_inject_statements(self: &Self, rust: TokenStream) {
        self.inject_statements.borrow_mut().push(rust);
    }

    fn get_inject_statements_stream(self: &Self) -> proc_macro2::TokenStream {
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

    fn get_preprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.preprocessors.clone()
    }

    fn get_postprocessors(self: &Self) -> Vec<Rc<dyn IRustHtmlProcessor>> {
        self.postprocessors.clone()
    }

    fn get_rust_preprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>> {
        self.rust_preprocessors.clone()
    }

    fn get_rust_postprocessors(self: &Self) -> Vec<Rc<dyn IRustProcessor>> {
        self.rust_postprocessors.clone()
    }

    fn add_operation_to_ooo_log(self: &Self, _operation: String) {
        // dont do anything
    }

    fn get_ooo(self: &Self) -> Vec<String> {
        vec![]
    }

    fn push_inject_statements_rshtml(self: &Self, rshtml: Vec<RustHtmlToken>, parser: Rc<dyn IRustHtmlParserAll>, ctx: Rc<dyn IRustHtmlParserContext>, ct: Rc<dyn ICancellationToken>) {
        let rust = parser.get_converter_out().convert_vec(rshtml, ctx, ct).unwrap();
        self.push_inject_statements(TokenStream::from_iter(rust));
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

    fn push_html_tag_parse_context(self: &Self, tag: Rc<dyn IHtmlTagParseContext>) {
        self.htmltag_parse_scope_stack.borrow_mut().push(tag);
    }

    fn get_is_in_html_mode(&self) -> bool {
        self.is_in_html_mode_stack.borrow().last().unwrap_or(&false).clone()
    }

    fn push_is_in_html_mode(&self, v: bool) {
        self.is_in_html_mode_stack.borrow_mut().push(v);
    }

    fn pop_is_in_html_mode(&self) -> bool {
        self.is_in_html_mode_stack.borrow_mut().pop().unwrap_or(false)
    }

    fn push_output_buffer(&self, buffer: Rc<RefCell<Vec<RustHtmlToken>>>) {
        self.output_buffer_stack.borrow_mut().push(buffer);
    }

    fn pop_output_buffer(&self) -> Option<Rc<RefCell<Vec<RustHtmlToken>>>> {
        if self.output_buffer_stack.borrow().len() > 1 {
            self.output_buffer_stack.borrow_mut().pop()
        } else {
            None
        }
    }

    fn get_output_buffer(&self) -> Option<Rc<RefCell<Vec<RustHtmlToken>>>> {
        self.output_buffer_stack.borrow().last().cloned()
    }

    fn push_output_token(&self, token: RustHtmlToken) -> Result<(), RustHtmlError<'static>> {
        if let Some(buffer) = self.get_output_buffer() {
            buffer.borrow_mut().push(token);
            Ok(())
        } else {
            Err(RustHtmlError::from_string(format!("no output buffer to push token ({:?}) to", token)))
        }
    }

    fn push_output_tokens(&self, token: &[RustHtmlToken]) -> Result<(), RustHtmlError<'static>> {
        if let Some(buffer) = self.get_output_buffer() {
            buffer.borrow_mut().extend(token.iter().cloned());
            Ok(())
        } else {
            Err(RustHtmlError::from_string(format!("no output buffer to push tokens ({:?}) to", token)))
        }
    }
}