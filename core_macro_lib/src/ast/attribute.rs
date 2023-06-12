use proc_macro2::{Ident, Punct, Group};


// this is used to represent an attribute in the ast, not in the final code.
pub(crate) struct AstAttribute {
    start_punct: Punct,
    pub name: Ident,
    pub content: Option<Group>,
}

impl AstAttribute {
    pub(crate) fn new(start_punct: Punct, name: Ident, content: Option<Group>) -> AstAttribute {
        Self {
            start_punct,
            name,
            content,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.start_punct.to_string(),
            self.name.to_string(),
            self.content.as_ref().map(|x| x.to_string()).unwrap_or("".to_string())
        )
    }
}