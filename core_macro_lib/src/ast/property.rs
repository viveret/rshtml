use proc_macro2::{TokenTree, Ident, Punct, TokenStream};


// this is a property of a struct or a parameter of a method or a function.
pub(crate) struct AstProperty {
    pub visibility: Option<Ident>,
    pub name: Ident,
    pub colon: Punct,
    pub value: Vec<TokenTree>,
}

impl AstProperty {
    pub(crate) fn new(
        visibility: Option<Ident>,
        name: Ident,
        colon: Punct,
        value: Vec<TokenTree>
    ) -> Self {
        Self {
            visibility,
            name,
            colon,
            value,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{} {}",
            self.visibility.as_ref().map(|x| format!(" {}", x.to_string())).unwrap_or("".to_string()),
            self.name.to_string(),
            self.colon.to_string(),
            self.value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
        )
    }

    pub(crate) fn finalize(&self) -> Vec<TokenTree> {
        let name = self.name.to_string();
        let value = TokenStream::from_iter(self.value.clone().into_iter());
        quote::quote! {
            Rc::new(ReflectedProperty::new(
                #name.to_string(),
                Box::new(TypeInfo::of::<#value>()),
            )),
        }.into_iter().collect::<Vec<TokenTree>>()
    }
}