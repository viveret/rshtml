use std::rc::Rc;

use core_lib::asyncly::cancellation_token::CancellationToken;
use mvc_lib::view::rusthtml::rusthtml_parser_context::RustHtmlParserContext;
use mvc_lib::view::rusthtml::parser_parts::peekable_tokentree::StreamPeekableTokenTree;
use mvc_lib::view::rusthtml::parser_parts::rusthtmlparser_html::{RustHtmlParserHtml, IRustHtmlParserHtml};


#[test]
pub fn test_rusthtmlparserhtml_constructor() {
    let _ = RustHtmlParserHtml::new();
}

#[test]
pub fn test_rusthtmlparserhtml_parse_html() {
    let parser = RustHtmlParserHtml::new();

    let html = quote::quote! {
        <div>
            <p>test</p>
        </div>
    };
    let it = Rc::new(StreamPeekableTokenTree::new(html));

    let ct = Rc::new(CancellationToken::new());
    let ctx = Rc::new(RustHtmlParserContext::new(false, false, "test".to_string()));
    let tokens = parser.parse_html(ctx, it, ct).unwrap();

    assert_eq!(tokens.len(), 4);
}