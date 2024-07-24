use std::rc::Rc;

use crate::view::rusthtml::node_helpers::environment_node::EnvironmentHtmlNodeParsed;
use crate::view::rusthtml::tag_helpers::environment_tag::EnvironmentHtmlTagParsed;

use super::inode_parsed::IHtmlNodeParsed;
use super::irusthtml_processor::IRustHtmlProcessor;
use super::itag_parsed::IHtmlTagParsed;
use super::irust_processor::IRustProcessor;

pub struct RustHtmlParserSubProcessors {
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
}

impl RustHtmlParserSubProcessors {
    pub fn new() -> Self {
        Self {
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
                // Rc::new(PostProcessFlattenGroupNoneDelimiter::new()),
                // Rc::new(PostProcessCombineStaticStr::new()),
            ],
        }
    }

    pub fn add_tag_parsed_handler(&mut self, handler: Rc<dyn IHtmlTagParsed>) {
        self.tag_parsed_handlers.push(handler);
    }

    pub fn add_node_parsed_handler(&mut self, handler: Rc<dyn IHtmlNodeParsed>) {
        self.node_parsed_handlers.push(handler);
    }

    pub fn add_preprocessor(&mut self, preprocessor: Rc<dyn IRustHtmlProcessor>) {
        self.preprocessors.push(preprocessor);
    }

    pub fn add_postprocessor(&mut self, postprocessor: Rc<dyn IRustHtmlProcessor>) {
        self.postprocessors.push(postprocessor);
    }

    pub fn add_rust_preprocessor(&mut self, preprocessor: Rc<dyn IRustProcessor>) {
        self.rust_preprocessors.push(preprocessor);
    }

    pub fn add_rust_postprocessor(&mut self, postprocessor: Rc<dyn IRustProcessor>) {
        self.rust_postprocessors.push(postprocessor);
    }
}