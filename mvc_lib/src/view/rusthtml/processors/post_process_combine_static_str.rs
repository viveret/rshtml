use proc_macro2::{TokenTree, Delimiter, Group, TokenStream, Ident, Punct};

use crate::view::rusthtml::peekable_tokentree::{PeekableTokenTree, IPeekableTokenTree};
use crate::view::rusthtml::rusthtml_error::RustHtmlError;
use crate::view::rusthtml::irust_processor::IRustProcessor;


// combine static strings into one literal
pub struct PostProcessCombineStaticStr { }

impl PostProcessCombineStaticStr {
    pub fn new() -> Self {
        Self { }
    }

    pub fn try_html_output_write_html_str_with_string_literal_arg_and_semicolon(
        is_first: &mut bool,
        current_str: &mut String,
        output: &mut Vec<TokenTree>,
        it: &PeekableTokenTree,
    ) -> bool {
        if let Some(html_output) = Self::peek_html_output(it) {
            // next char has to be '.'
            if let Some(period) = Self::peek_punct_with_char('.', 1, it) {
                // next ident has to be 'write_html_str'
                if let Some(write_html) = Self::peek_write_html(it) {
                    // next has to be '('
                    if let Some(s) = Self::peek_group_with_string_literal_arg(Delimiter::Parenthesis, it) {
                        Self::handle_string_literal(is_first, current_str, output, html_output, period, write_html, s, it);
                        true
                    // } else if Self::peek_group_with_ident_expression_arg(Delimiter::Parenthesis, it) {
                    //     Self::append_and_clear(output, current_str);
                    //     false
                    } else {
                        // panic!("next group is not group with string literal or identifier expression");
                        // println!("next group is not group with string literal or identifier expression");
                        false
                    }
                } else {
                    panic!("next ident is not 'write_html_str', was {:?}", it.peek());
                }
            } else {
                panic!("next char is not '.', was {:?}", it.peek());
            }
        } else {
            // println!("next ident is not 'html_output', was {:?}", it.peek());
            false
        }
    }

    pub fn handle_string_literal(
        is_first: &mut bool,
        current_str: &mut String,
        output: &mut Vec<TokenTree>,
        html_output: (TokenTree, Ident),
        period: (TokenTree, Punct),
        write_html: (TokenTree, Ident),
        s: String,
        it: &PeekableTokenTree,
    ) {
        current_str.push_str(s.as_str());

        if *is_first {
            *is_first = false;
            output.push(html_output.0.clone());
            output.push(period.0.clone());
            output.push(write_html.0.clone());
        }
        
        // next char has to be ';'
        if Self::peek_punct_with_char(';', 4, it).is_some() {
            // skip all
            it.next();
            it.next();
            it.next();
            it.next();
            it.next();
        } else {
            panic!("next char is not ';', was {:?}", it.peek());
        }
    }

    pub fn append_and_clear(output: &mut Vec<TokenTree>, current_str: &mut String) {
        if current_str.len() > 0 {
            let inner = TokenStream::from_iter(vec![ TokenTree::Literal(proc_macro2::Literal::string(current_str.as_str())) ]);
            output.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)));
            output.push(TokenTree::Punct(Punct::new(';', proc_macro2::Spacing::Alone)));
            current_str.clear();
        }
    }

    pub fn peek_ident_with_name(name: &str, peek_nth: usize, it: &PeekableTokenTree) -> Option<(TokenTree, Ident)> {
        if let Some(token) = it.peek_nth(peek_nth) {
            match &token {
                TokenTree::Ident(ident) if ident.to_string().as_str() == name => {
                    Some((token.clone(), ident.clone()))
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn peek_write_html(it: &PeekableTokenTree) -> Option<(TokenTree, Ident)> {
        for ident_name in ["write_html_str", "write_html"].iter() {
            if let Some(write_html) = Self::peek_ident_with_name(ident_name, 2, it) {
                return Some(write_html);
            }
        }
        None
    }

    pub fn peek_html_output(it: &PeekableTokenTree) -> Option<(TokenTree, Ident)> {
        Self::peek_ident_with_name("html_output", 0, it)
    }

    pub fn peek_punct_with_char(c: char, peek_nth: usize, it: &PeekableTokenTree) -> Option<(TokenTree, Punct)> {
        if let Some(token) = it.peek_nth(peek_nth) {
            match &token {
                TokenTree::Punct(punct) if punct.as_char() == c => {
                    Some((token.clone(), punct.clone()))
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn peek_group(delimiter: Delimiter, peek_nth: usize, it: &PeekableTokenTree) -> Option<Group> {
        if let Some(token) = it.peek_nth(peek_nth) {
            match token {
                TokenTree::Group(group) if group.delimiter() == delimiter => {
                    Some(group.clone())
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn peek_group_with_string_literal_arg(delimiter: Delimiter, it: &PeekableTokenTree) -> Option<String> {
        if let Some(group) = Self::peek_group(delimiter, 3, it) {
            // next is string literal
            if let Some(s) = Self::is_string_literal(PeekableTokenTree::new(group.stream()).next().as_ref()) {
                // println!("peek_group_with_string_literal_arg: {:?}", s);
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn peek_group_with_ident_expression_arg(delimiter: Delimiter, it: &PeekableTokenTree) -> bool {
        if let Some(group) = Self::peek_group(delimiter, 3, it) {
            // next is identifier expression
            if Self::is_ident_expression(&PeekableTokenTree::new(group.stream())) {
                true
            } else {
                // println!("peek_group_with_ident_expression_arg: {:?}", it.peek());
                false
            }
        } else {
            // println!("peek_group_with_ident_expression_arg: {:?}", it.peek());
            false
        }
    }

    pub fn is_ident_expression(it: &PeekableTokenTree) -> bool {
        loop {
            if let Some(token) = it.next() {
                match &token {
                    TokenTree::Ident(_) => {
                        continue;
                    },
                    TokenTree::Punct(punct) if punct.as_char() == '.' => {
                        continue;
                    },
                    TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                        if Self::is_ident_expression(&PeekableTokenTree::new(group.stream())) {
                            continue;
                        } else {
                            println!("is_ident_expression: {:?}", token);
                            return false;
                        }
                    },
                    _ => {
                        println!("is_ident_expression: {:?}", token);
                        return false;
                    },
                }
            } else {
                break;
            }
        }
        true
    }

    pub fn is_string_literal(it: Option<&TokenTree>) -> Option<String> {
        if let Some(token) = it {
            match token {
                TokenTree::Literal(literal) => {
                    let s = literal.to_string();
                    Some(snailquote::unescape(&s).unwrap())
                },
                TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                    // check if contents are string literal or group with string literal
                    Self::is_string_literal(PeekableTokenTree::new(group.stream()).next().as_ref())
                },
                _ => None,
            }
        } else {
            None
        }
    }
}

impl IRustProcessor for PostProcessCombineStaticStr {
    fn get_stage_for(&self) -> &str {
        "post"
    }

    fn process_rust(&self, rusthtml: &Vec<TokenTree>) -> Result<Vec<TokenTree>, RustHtmlError> {
        let mut output = vec![];
        let mut current_str = String::new();
        let mut is_first = true;

        let it = PeekableTokenTree::from_vec(rusthtml);
        loop {
            if Self::try_html_output_write_html_str_with_string_literal_arg_and_semicolon(
                &mut is_first,
                &mut current_str,
                &mut output,
                &it
            ) {
            } else if let Some(group) = Self::peek_group(Delimiter::Brace, 0, &it) {
                // recurse
                it.next();
                let inner = self.process_rust(&group.stream().into_iter().collect::<Vec<TokenTree>>())?;
                // println!("inner: {:?}", inner);
                output.push(TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(inner))));
            } else if let Some(token) = it.next() {
                Self::append_and_clear(&mut output, &mut current_str);
                is_first = true;
                output.push(token.clone());
            } else {
                break;
            }
        }
        
        Self::append_and_clear(&mut output, &mut current_str);
        Ok(output)
    }

    fn get_type(&self) -> &str {
        nameof::name_of_type!(PostProcessCombineStaticStr)
    }
}