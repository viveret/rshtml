use proc_macro2::{Ident, Span, TokenTree, TokenStream};

use super::attribute::AstAttribute;
use super::property::AstProperty;


// this is a method of a struct or a static function.
pub(crate) struct AstMethod {
    pub attributes: Vec<AstAttribute>,
    pub vis: Option<Ident>,
    pub name: Ident,
    pub generics: Vec<TokenTree>,
    pub return_type: Vec<TokenTree>,
    pub args: Vec<AstProperty>
}

impl AstMethod {
    pub(crate) fn new(
        attributes: Vec<AstAttribute>,
        method_visibility: Option<Ident>,
        method_name: Ident,
        method_generics: Vec<TokenTree>,
        method_args: Vec<AstProperty>,
        method_return_type: Vec<TokenTree>
    ) -> Self {
        Self {
            attributes: attributes,
            vis: method_visibility,
            name: method_name,
            generics: method_generics,
            return_type: method_return_type,
            args: method_args
        }
    }

    pub fn finalize(&self) -> TokenStream {
        let attributes = TokenStream::from_iter(self.attributes.iter().flat_map(|x| x.finalize()));
        let name = self.name.to_string();
        // let return_type = TokenStream::from_iter(self.return_type.clone().into_iter());
        // Box::new(TypeInfo::of::<#return_type>()),
        let return_type_tokens = if self.return_type.len() > 0 {
            let return_type = TokenStream::from_iter(self.return_type.clone().into_iter());
            quote::quote! {
                Some(Box::new(TypeInfo::of::<#return_type>()))
            }
        } else {
            quote::quote! { None }
        };
        let vis = self.vis.clone().unwrap_or(Ident::new("private", Span::call_site())).to_string();
        let parameters = TokenStream::from_iter(self.args.iter().flat_map(|x| x.finalize()));
        quote::quote! {
            Rc::new(ReflectedMethod::new(
                vec![#attributes],
                #vis.to_string(),
                #name.to_string(),
                vec![],
                vec![#parameters],
                #return_type_tokens,
            )),
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{}{}{}{}{} -> {}",
            self.attributes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "),
            self.vis.as_ref().map(|x| format!("{} ", x.to_string())).unwrap_or("".to_string()),
            self.name.to_string(),
            self.generics.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
            self.args.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
            self.return_type.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
        )
    }

    // fn safe_return_type(&self) -> Vec<TokenTree> {
    //     if self.return_type.len() == 0 {
    //         vec![TokenTree::Group(Group::new(proc_macro2::Delimiter::Parenthesis, TokenStream::new()))]
    //     } else {
    //         self.return_type.clone()
    //     }
    // }
}