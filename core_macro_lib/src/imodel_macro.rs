use proc_macro2::TokenStream;
use quote::quote;


pub(crate) fn impl_imodel(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote! {
        impl IModel for #name {
            fn get_properties(&self) -> HashMap<String, Rc<dyn IModelProperty>> {
                Self::reflected_properties().into_iter().map(|x| (x.get_name(), x)).collect::<HashMap<String, Rc<dyn IModelProperty>>>()
            }
        
            fn get_property(&self, name: &str) -> Option<Rc<dyn IModelProperty>> {
                self.get_properties().get(name).cloned()
            }
        
            fn get_methods(&self) -> std::collections::HashMap<String, Rc<dyn IModelMethod>> {
                Self::reflected_methods().into_iter().map(|x| (x.get_name(), x)).collect::<HashMap<String, Rc<dyn IModelMethod>>>()
            }
        
            fn get_method(&self, name: &str) -> Option<Rc<dyn IModelMethod>> {
                self.get_methods().get(name).cloned()
            }
        
            fn get_type_info(&self) -> Box<TypeInfo> {
                Box::new(TypeInfo::of::<Self>())
            }
        
            fn get_underlying_value(&self) -> &dyn std::any::Any {
                self // default for IModel is itself
            }
        
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        
            fn to_string(&self) -> String {
                self.get_type_info().to_string()
            }
        }
    }
}
