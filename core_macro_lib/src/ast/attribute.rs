use proc_macro2::{Ident, Punct, Group, TokenTree, Span};


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
        let type_name_tokens = if name.chars().nth(0).unwrap().is_uppercase() {
            let type_name = Ident::new(name.as_str(), Span::call_site());
            quote::quote! {
                Some(Box::new(TypeInfo::of::<#type_name>()))
            }
        } else {
            quote::quote! {
                None
            }
        };
        quote::quote! {
            Rc::new(ReflectedAttribute::new(
                #name.to_string(),
                #content.to_string(),
                #type_name_tokens,
            )),
        }.into_iter().collect::<Vec<TokenTree>>()

        // let name = &x.name;
        // let name_str = name.to_string();
        // let value = if let Some(a) = &x.content {
        //     a.to_string()
        // } else {
        //     String::new()
        // };

        // quote::quote! {
        //     Rc::new(ReflectedAttribute::new(
        //         #name_str.to_string(),
        //         #value.to_string(),
        //         None,
        //     )),
        // }.into_iter()
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!(
            "{}[{}{}]",
            self.start_punct.to_string(),
            self.name.to_string(),
            self.content.as_ref().map(|x| x.to_string()).unwrap_or("".to_string())
        )
    }
}