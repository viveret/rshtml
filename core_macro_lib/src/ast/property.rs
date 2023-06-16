use proc_macro2::{TokenTree, Ident, Punct, TokenStream};

use super::attribute::AstAttribute;


// this is a property of a struct or a parameter of a method or a function.
pub(crate) struct AstProperty {
    pub attributes: Vec<AstAttribute>,
    pub visibility: Option<Ident>,
    pub name_ampersand: Option<Punct>,
    pub name: Ident,
    pub colon: Option<Punct>,
    pub return_type: Vec<TokenTree>,
}

impl AstProperty {
    pub(crate) fn new(
        attributes: Vec<AstAttribute>,
        visibility: Option<Ident>,
        name_ampersand: Option<Punct>,
        name: Ident,
        colon: Option<Punct>,
        return_type: Vec<TokenTree>
    ) -> Self {
        Self {
            attributes,
            visibility,
            name_ampersand,
            name,
            colon,
            return_type,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{} {}",
            self.attributes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "),
            self.visibility.as_ref().map(|x| format!(" {}", x.to_string())).unwrap_or("".to_string()),
            self.name_ampersand.as_ref().map(|x| x.to_string()).unwrap_or(String::new()),
            self.name.to_string(),
            self.colon.as_ref().map(|x| x.to_string()).unwrap_or(String::new()),
            self.return_type.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
        )
    }

    pub(crate) fn finalize(&self) -> Vec<TokenTree> {
        let has_name_ampersand = self.name_ampersand.is_some();
        let name = self.name.to_string();
        let return_type_tokens = if self.return_type.len() > 0 {
            let return_type = TokenStream::from_iter(self.return_type.clone().into_iter());
            quote::quote! {
                Some(Box::new(TypeInfo::of::<#return_type>()))
            }
        } else {
            quote::quote! { None }
        };
        let attribute_tokens = self.attributes.iter().flat_map(|x| {
            let tokens = x.finalize();
            tokens
        }).collect::<Vec<TokenTree>>();
        let attributes = TokenStream::from_iter(attribute_tokens.into_iter());
        quote::quote! {
            Rc::new(ReflectedProperty::new(
                vec![#attributes],
                #has_name_ampersand,
                #name.to_string(),
                #return_type_tokens,
            )),
        }.into_iter().collect::<Vec<TokenTree>>()
    }
}