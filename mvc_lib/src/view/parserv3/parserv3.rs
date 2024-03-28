use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::converters::iconverter_output::{ConverterOutput, IConverterOutput};
use super::converters::iconverter_middle::{ConverterMiddle, IConverterMiddle};
use super::converters::iconverter_input::{ConverterInput, IConverterInput};

pub trait IParserV3 {
    fn get_converter_in(&self) -> Rc<dyn IConverterInput>;
    fn get_converter_middle(&self) -> Rc<dyn IConverterMiddle>;
    fn get_converter_out(&self) -> Rc<dyn IConverterOutput>;

    fn expand(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableTokenTree>;
    fn expand_with_context(&self,
        input: TokenStream,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<TokenStream, RustHtmlError>;
}

pub struct ParserV3 {
    pub converter_in: Rc<dyn IConverterInput>,
    pub converter_middle: Rc<dyn IConverterMiddle>,
    pub converter_out: Rc<dyn IConverterOutput>,
}

impl ParserV3 {
    pub fn new(
        converter_in: Rc<dyn IConverterInput>,
        converter_middle: Rc<dyn IConverterMiddle>,
        converter_out: Rc<dyn IConverterOutput>
    ) -> Self {
        Self {
            converter_in,
            converter_middle,
            converter_out,
        }
    }

    pub fn new_default() -> Self {
        Self {
            converter_in: Rc::new(ConverterInput::new()),
            converter_middle: Rc::new(ConverterMiddle::new()),
            converter_out: Rc::new(ConverterOutput::new()),
        }
    }
}

impl IParserV3 for ParserV3 {
    fn get_converter_in(&self) -> Rc<dyn IConverterInput> {
        self.converter_in.clone()
    }

    fn get_converter_middle(&self) -> Rc<dyn IConverterMiddle> {
        self.converter_middle.clone()
    }

    fn get_converter_out(&self) -> Rc<dyn IConverterOutput> {
        self.converter_out.clone()
    }
    
    fn expand(&self, input: Rc<dyn IPeekableTokenTree>) -> Rc<dyn IPeekableTokenTree> {
        let mut input = self.converter_in.convert(input);
        input = self.converter_middle.convert(input);
        self.converter_out.convert(input)
    }
    
    fn expand_with_context(&self,
        input: TokenStream,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<TokenStream, RustHtmlError> {
        let input = Rc::new(StreamPeekableTokenTree::new(input));
        let mut input = self.converter_in.convert(input);
        input = self.converter_middle.convert(input);
        let output = self.converter_out.convert(input);
        Ok(output.to_stream())
    }
}