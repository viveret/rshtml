extern crate proc_macro;
extern crate proc_macro2;
extern crate mvc_lib;

use mvc_lib::view::macro_impl::rusthtml_macro_impl;
use mvc_lib::view::macro_impl::rusthtml_view_macro_impl;
use proc_macro2::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenTree;
use quote::quote;


#[proc_macro]
pub fn rusthtml_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rusthtml_macro_impl(input.into()).into()
}

// puts render function into a structure with additional functionality and information
#[proc_macro]
pub fn rusthtml_view_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rusthtml_view_macro_impl(input.into()).into()
}


fn rc_controller_action_impl(new_fn: &Ident, input: TokenStream) -> TokenStream {
    let action_name = match input.into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident.clone(),
        _ => panic!("expected ident"),
    };
    quote! {
        Rc::new(
            ControllerActionMemberFn::#new_fn(
                vec![],
                None,
                action_name_to_path(
                    IControllerExtensions::get_name_ref(self),
                    nameof_member_fn!(Self::#action_name)
                ),
                nameof_member_fn!(Self::#action_name).into(),
                IControllerExtensions::get_name(self).into(),
                self.get_route_area(),
                Box::new(Self::#action_name)
            )
        )
    }.into()
}

#[proc_macro]
pub fn rc_controller_action(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rc_controller_action_impl(&Ident::new("new_not_validated", proc_macro2::Span::call_site()), input.into()).into()
}

#[proc_macro]
pub fn rc_controller_action_validate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rc_controller_action_impl(&Ident::new("new_validated", proc_macro2::Span::call_site()), input.into()).into()
}

#[proc_macro]
pub fn rc_controller_action_validate_typed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    rc_controller_action_impl(&Ident::new("new_validated_typed", proc_macro2::Span::call_site()), input.into()).into()
}
