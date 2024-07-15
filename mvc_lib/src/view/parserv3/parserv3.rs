use std::rc::Rc;

use core_lib::asyncly::icancellation_token::ICancellationToken;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::view::rusthtml::directives::model_directive::ModelDirective;
use crate::view::rusthtml::directives::name_directive::NameDirective;
use crate::view::rusthtml::directives::viewstart_directive::ViewStartDirective;
use crate::view::rusthtml::irusthtml_parser_context::IRustHtmlParserContext;
use crate::view::rusthtml::parser_parts::peekable_tokentree::{IPeekableTokenTree, StreamPeekableTokenTree};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::converters::converter_directives::ConverterDirectives;
use super::converters::iconverter_output::{ConverterOutput, IConverterOutput};
use super::converters::iconverter_middle::{ConverterMiddle, IConverterMiddle};
use super::converters::iconverter_input::{ConverterInput, IConverterInput};
use super::converters::iparserv3_rust_parser::{ParserV3RustParser, IParserV3RustParser};

use crate::view::rusthtml::directives::use_directive::UseDirective;

pub trait IParserV3 {
    fn get_converter_in(&self) -> Rc<dyn IConverterInput>;
    fn get_converter_middle(&self) -> Rc<dyn IConverterMiddle>;
    fn get_converter_out(&self) -> Rc<dyn IConverterOutput>;

    // other parts
    fn get_rust_parser(&self) -> Rc<dyn IParserV3RustParser>;
    fn get_converter_directives(&self) -> Rc<dyn IConverterMiddle>;

    fn expand(&self,
        input: Rc<dyn IPeekableTokenTree>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError>;

    fn expand_tokentree(&self,
        input: TokenStream,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<TokenStream, RustHtmlError>;
}

pub struct ParserV3 {
    pub converter_in: Rc<dyn IConverterInput>,
    pub converter_middle: Rc<dyn IConverterMiddle>,
    pub converter_out: Rc<dyn IConverterOutput>,
    pub converter_directives: Rc<dyn IConverterMiddle>,
    pub rust_parser: Rc<dyn IParserV3RustParser>
}

impl ParserV3 {
    pub fn new(
        converter_in: Rc<dyn IConverterInput>,
        converter_middle: Rc<dyn IConverterMiddle>,
        converter_out: Rc<dyn IConverterOutput>,
        converter_directives: Rc<dyn IConverterMiddle>,
        rust_parser: Rc<dyn IParserV3RustParser>
    ) -> Self {
        Self {
            converter_in,
            converter_middle,
            converter_out,
            converter_directives,
            rust_parser
        }
    }

    pub fn new_default() -> Rc<dyn IParserV3> {
        let x = Rc::new(Self {
            converter_in: Rc::new(ConverterInput::new()),
            converter_middle: Rc::new(ConverterMiddle::new()),
            converter_out: Rc::new(ConverterOutput::new()),
            converter_directives: Rc::new(ConverterDirectives::new()),
            rust_parser: Rc::new(ParserV3RustParser::new())
        });
        x.get_converter_middle().set_parser(x.clone());
        x.get_converter_directives().set_parser(x.clone());
        x
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
    
    fn get_converter_directives(&self) -> Rc<dyn IConverterMiddle> {
        self.converter_directives.clone()
    }

    fn get_rust_parser(&self) -> Rc<dyn IParserV3RustParser> {
        self.rust_parser.clone()
    }
    
    fn expand(&self,
        input: Rc<dyn IPeekableTokenTree>,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<Rc<dyn IPeekableTokenTree>, RustHtmlError> {
        let mut input = self.converter_in.convert(input);
        input = self.converter_middle.convert(input, context, ct)?;
        self.converter_out.convert(input)
    }
    
    fn expand_tokentree(&self,
        input: TokenStream,
        context: Rc<dyn IRustHtmlParserContext>,
        ct: Rc<dyn ICancellationToken>
    ) -> Result<TokenStream, RustHtmlError> {
        let input = Rc::new(StreamPeekableTokenTree::new(input));
        let mut input = self.converter_in.convert(input);
        input = self.converter_middle.convert(input, context, ct)?;
        let output = self.converter_out.convert(input)?;
        Ok(output.to_stream())
    }
}