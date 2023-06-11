use proc_macro2::{Ident, TokenTree, TokenStream};



pub(crate) struct AstMethodSimple {
    pub name: String,
    pub return_type: String,
    pub args: String,
}

pub(crate) struct AstMethod {
    pub vis: Option<Ident>,
    pub name: Ident,
    pub return_type: Vec<TokenTree>,
    pub args: Vec<(Ident, Vec<TokenTree>)>
}

impl AstMethod {
    pub(crate) fn new(
        method_visibility: Option<proc_macro2::Ident>,
        method_name: proc_macro2::Ident,
        method_generics: Vec<proc_macro2::TokenTree>,
        method_args: Vec<(proc_macro2::Ident, Vec<proc_macro2::TokenTree>)>,
        method_return_type: Vec<proc_macro2::TokenTree>
    ) -> Self {
        Self {
            vis: method_visibility,
            name: method_name,
            return_type: method_return_type,
            args: method_args
        }
    }

    pub(crate) fn finalize(&self) -> TokenStream {
        let name = self.name.to_string();
        let value = TokenStream::from_iter(self.return_type.clone().into_iter());
        let vis = self.vis.clone().unwrap_or(Ident::new("private", proc_macro2::Span::call_site())).to_string();
        let parameters = TokenStream::from_iter(self.args.iter().map(|x| {
            let name = x.0.to_string();
            let value = TokenStream::from_iter(x.1.clone().into_iter());
            quote::quote! {
                Rc::new(ReflectedProperty::new(
                    #name.to_string(),
                    Box::new(TypeInfo::of::<#value>()),
                )),
            }
        }));
        quote::quote! {
            Rc::new(ReflectedMethod::new(
                #name.to_string(),
                #vis.to_string(),
                vec![#parameters],
                Box::new(TypeInfo::of::<#value>()),
            )),
        }
    }
}