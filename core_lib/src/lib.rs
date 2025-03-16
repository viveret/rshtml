pub mod assert;
pub mod asyncly;
pub mod sys;



// this is used by the RustHtmlParserContextLog struct to implement the IRustHtmlParserContext trait
#[macro_export]
macro_rules! impl_with_logging {
    ($trait_name:ident, $struct_name:ident, $real_context:ident, 
        $(fn $method_name:ident(&self $(, $arg_name:ident: $arg_type:ty)*) $(-> $ret_type:ty)?;)+) => {
        impl $struct_name {
            fn add_operation_to_ooo_log_str(&self, operation: &str) -> bool {
                self.order_of_operations.borrow_mut().push(operation.to_string());
                true
            }

            fn add_operation_to_ooo_log(&self, operation: String) {
                self.order_of_operations.borrow_mut().push(operation);
            }

            pub fn get_order_of_operations(&self) -> Vec<String> {
                self.order_of_operations.borrow().clone()
            }
        }

        impl $trait_name for $struct_name {
            $(
                impl_with_logging!(@method_impl $struct_name, $real_context, $method_name, [$($arg_name: $arg_type),*] $(-> $ret_type)?);
            )+
        }
    };

    // Handle methods with return type
    (@method_impl $struct_name:ident, $real_context:ident, $method_name:ident, [$($arg_name:ident: $arg_type:ty),*] -> $ret_type:ty) => {
        fn $method_name(&self, $($arg_name: $arg_type),*) -> $ret_type {
            self.add_operation_to_ooo_log_str(stringify!($method_name));
            self.$real_context.$method_name($($arg_name),*)
        }
    };

    // Handle methods without return type
    (@method_impl $struct_name:ident, $real_context:ident, $method_name:ident, [$($arg_name:ident: $arg_type:ty),*]) => {
        fn $method_name(&self, $($arg_name: $arg_type),*) {
            self.add_operation_to_ooo_log_str(stringify!($method_name));
            self.$real_context.$method_name($($arg_name),*)
        }
    };
}