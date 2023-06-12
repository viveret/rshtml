use proc_macro2::{Ident, Span, TokenTree, TokenStream};

use super::property::AstProperty;


pub(crate) struct AstMethod {
    pub vis: Option<Ident>,
    pub name: Ident,
    pub generics: Vec<TokenTree>,
    pub return_type: Vec<TokenTree>,
    pub(crate) args: Vec<AstProperty>
}

impl AstMethod {
    pub(crate) fn new(
        method_visibility: Option<Ident>,
        method_name: Ident,
        method_generics: Vec<TokenTree>,
        method_args: Vec<AstProperty>,
        method_return_type: Vec<TokenTree>
    ) -> Self {
        Self {
            vis: method_visibility,
            name: method_name,
            generics: method_generics,
            return_type: method_return_type,
            args: method_args
        }
    }

    pub fn finalize(&self) -> TokenStream {
        let name = self.name.to_string();
        let value = TokenStream::from_iter(self.return_type.clone().into_iter());
        let vis = self.vis.clone().unwrap_or(Ident::new("private", Span::call_site())).to_string();
        let parameters = TokenStream::from_iter(self.args.iter().map(|x| {
            let name = x.name.to_string();
            let value = TokenStream::from_iter(x.value.clone().into_iter());
            quote::quote! {
                Rc::new(ReflectedProperty::new(
                    #name.to_string(),
                    Box::new(TypeInfo::of::<#value>()),
                )),
            }
        }));
        quote::quote! {
            Rc::new(ReflectedMethod::new(
                vec![],
                #vis.to_string(),
                #name.to_string(),
                vec![],
                vec![#parameters],
                Box::new(TypeInfo::of::<#value>()),
            )),
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{}{}: {}",
            self.name.to_string(),
            self.generics.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
            self.return_type.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
        )
    }
}