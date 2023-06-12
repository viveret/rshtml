use proc_macro2::{TokenTree, Ident, Punct, TokenStream, Group};

use super::attribute::AstAttribute;


// this is a property of a struct or a parameter of a method or a function.
pub(crate) struct AstProperty {
    pub attributes: Vec<AstAttribute>,
    pub visibility: Option<Ident>,
    pub name: Ident,
    pub colon: Option<Punct>,
    pub return_type: Vec<TokenTree>,
}

impl AstProperty {
    pub(crate) fn new(
        attributes: Vec<AstAttribute>,
        visibility: Option<Ident>,
        name: Ident,
        colon: Option<Punct>,
        return_type: Vec<TokenTree>
    ) -> Self {
        Self {
            attributes,
            visibility,
            name,
            colon,
            return_type,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{} {}",
            self.attributes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "),
            self.visibility.as_ref().map(|x| format!(" {}", x.to_string())).unwrap_or("".to_string()),
            self.name.to_string(),
            self.colon.as_ref().map(|x| x.to_string()).unwrap_or(String::new()),
            self.return_type.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
        )
    }

    pub(crate) fn finalize(&self) -> Vec<TokenTree> {
        let name = self.name.to_string();
        let return_type = TokenStream::from_iter(self.safe_return_type().into_iter());
        quote::quote! {
            Rc::new(ReflectedProperty::new(
                #name.to_string(),
                Box::new(TypeInfo::of::<#return_type>()),
            )),
        }.into_iter().collect::<Vec<TokenTree>>()
    }

    fn safe_return_type(&self) -> Vec<TokenTree> {
        if self.return_type.len() == 0 {
            vec![TokenTree::Group(Group::new(proc_macro2::Delimiter::Parenthesis, TokenStream::new()))]
        } else {
            self.return_type.clone()
        }
    }
}