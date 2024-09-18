
// can't assert on string output because it could be changed
// without changing underlying logic or behavior, so we are
// just going to test underlying behavior via exposed methods
// pub fn assert_tokentree_stream(stream: proc_macro2::TokenStream, expected: &str)

use core_lib::assert::assert_tokentree::assert_tokentree_stream;
use quote::quote;


#[test]
pub fn test_assert_tokentree_stream_empty() {
    assert_tokentree_stream(quote! {}, quote! {});
}

#[test]
pub fn test_assert_tokentree_stream_single_token() {
    assert_tokentree_stream(quote! { fn }, quote! { fn });
}

#[test]
pub fn test_assert_tokentree_stream_multiple_tokens() {
    assert_tokentree_stream(quote! {
        fn main() {}
    }, quote! {
        fn main() {}
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_variables() {
    assert_tokentree_stream(quote! {
        let x = 42;
    }, quote! {
        let x = 42;
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_expressions() {
    assert_tokentree_stream(quote! {
        let y = x + 1;
    }, quote! {
        let y = x + 1;
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_generics() {
    assert_tokentree_stream(quote! {
        fn add<T>(a: T, b: T) -> T {
            a + b
        }
    }, quote! {
        fn add<T>(a: T, b: T) -> T {
            a + b
        }
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_macro_invocation() {
    assert_tokentree_stream(quote! {
        println!("Hello, {}", name);
    }, quote! {
        println!("Hello, {}", name);
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_function_call() {
    assert_tokentree_stream(quote! {
        let result = calculate(10, 20);
    }, quote! {
        let result = calculate(10, 20);
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_conditional() {
    assert_tokentree_stream(quote! {
        if x > y {
            return x;
        } else {
            return y;
        }
    }, quote! {
        if x > y {
            return x;
        } else {
            return y;
        }
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_nested_blocks() {
    assert_tokentree_stream(quote! {
        {
            let x = 5;
            {
                let y = 10;
                x + y
            }
        }
    }, quote! {
        {
            let x = 5;
            {
                let y = 10;
                x + y
            }
        }
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_module_definition() {
    assert_tokentree_stream(quote! {
        mod my_module {
            pub fn my_function() {
                println!("Hello from module!");
            }
        }
    }, quote! {
        mod my_module {
            pub fn my_function() {
                println!("Hello from module!");
            }
        }
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_struct_definition() {
    assert_tokentree_stream(quote! {
        struct Point {
            x: i32,
            y: i32,
        }
    }, quote! {
        struct Point {
            x: i32,
            y: i32,
        }
    });
}

#[test]
pub fn test_assert_tokentree_stream_with_enum_definition() {
    assert_tokentree_stream(quote! {
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }
    }, quote! {
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }
    });
}

#[test]
#[should_panic(expected = "Token mismatch")]
pub fn test_assert_tokentree_stream_mismatched_function_name() {
    assert_tokentree_stream(quote! {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }, quote! {
        fn subtract(a: i32, b: i32) -> i32 {
            a + b
        }
    });
}

#[test]
#[should_panic(expected = "Token mismatch")]
pub fn test_assert_tokentree_stream_mismatched_variable_name() {
    assert_tokentree_stream(quote! {
        let x = 42;
    }, quote! {
        let y = 42;
    });
}

#[test]
#[should_panic(expected = "Token mismatch")]
pub fn test_assert_tokentree_stream_mismatched_expression() {
    assert_tokentree_stream(quote! {
        let z = x + 1;
    }, quote! {
        let z = x + 2;
    });
}

#[test]
#[should_panic(expected = "Token mismatch")]
pub fn test_assert_tokentree_stream_mismatched_macro_invocation() {
    assert_tokentree_stream(quote! {
        println!("Hello, world!");
    }, quote! {
        println!("Goodbye, world!");
    });
}

#[test]
#[should_panic(expected = "Token mismatch")]
pub fn test_assert_tokentree_stream_mismatched_block_contents() {
    assert_tokentree_stream(quote! {
        {
            let x = 5;
            x + 10
        }
    }, quote! {
        {
            let x = 5;
            x + 20
        }
    });
}

#[test]
#[should_panic(expected = "Left token stream has more tokens than the right.")]
pub fn test_assert_tokentree_stream_extra_tokens_left() {
    assert_tokentree_stream(quote! {
        let x = 42;
        let y = 43;
    }, quote! {
        let x = 42;
    });
}

#[test]
#[should_panic(expected = "Right token stream has more tokens than the left.")]
pub fn test_assert_tokentree_stream_extra_tokens_right() {
    assert_tokentree_stream(quote! {
        let x = 42;
    }, quote! {
        let x = 42;
        let y = 43;
    });
}
