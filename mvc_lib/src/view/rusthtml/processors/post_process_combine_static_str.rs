use crate::view::rusthtml::{irusthtml_processor::IRustHtmlProcessor, rusthtml_token::RustHtmlToken};



pub struct PostProcessCombineStaticStr {
}

impl PostProcessCombineStaticStr {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl IRustHtmlProcessor for PostProcessCombineStaticStr {
    fn get_stage_for(&self) -> &str {
        "post"
    }

    fn process_rusthtml(&self, rusthtml: &Vec<crate::view::rusthtml::rusthtml_token::RustHtmlToken>) -> Result<Vec<crate::view::rusthtml::rusthtml_token::RustHtmlToken>, crate::view::rusthtml::rusthtml_error::RustHtmlError> {
        let mut output = vec![];
        let mut current_str = String::new();
        
        fn append_and_clear(output: &mut Vec<crate::view::rusthtml::rusthtml_token::RustHtmlToken>, current_str: &mut String) {
            if current_str.len() > 0 {
                output.push(RustHtmlToken::AppendToHtml(
                    vec![RustHtmlToken::Literal(None, Some(current_str.clone()))]
                ));
                current_str.clear();
            }
        }

        for token in rusthtml {
            match token {
                crate::view::rusthtml::rusthtml_token::RustHtmlToken::AppendToHtml(s) => {
                    if let Some(crate::view::rusthtml::rusthtml_token::RustHtmlToken::Literal(l, s)) = s.first() {
                        if let Some(l) = l {
                            current_str.push_str(l.to_string().as_str());
                            continue;
                        } else if let Some(s) = s {
                            current_str.push_str(s);
                            continue;
                        } else {
                            panic!("Literal and string are both None");
                        }
                    }
                    append_and_clear(output.as_mut(), &mut current_str);
                    output.push(token.clone());
                },
                _ => {
                    append_and_clear(output.as_mut(), &mut current_str);
                    output.push(token.clone());
                }
            }
        }
        append_and_clear(output.as_mut(), &mut current_str);
        Ok(output)
    }
}