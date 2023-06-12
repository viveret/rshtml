use proc_macro2::{Ident, Punct, Group, TokenTree};


// this is used to represent an attribute in the ast, not in the final code.
pub(crate) struct AstAttribute {
    pub start_punct: Punct,
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

    pub fn finalize(&self) -> Vec<TokenTree> {
        let name = self.name.to_string();
        let content = self.content.as_ref().map(|x| x.to_string()).unwrap_or("".to_string());
        quote::quote! {
            Rc::new(ReflectedAttribute::new(
                #name.to_string(),
                #content.to_string(),
            )),
        }.into_iter().collect::<Vec<TokenTree>>()
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