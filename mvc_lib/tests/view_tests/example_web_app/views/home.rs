use mvc_lib::view::macro_impl::rusthtml_view_macro_impl;

// #[test]
// pub fn example_web_app_home__view() {
//     let html = quote::quote! {
//     };

//     let result = rusthtml_view_macro_impl(html);
//     let result_len = result.into_iter().count();

//     assert_ne!(0, result_len)
// }



#[test]
pub fn example_web_app_home_index_view() {
    let html = quote::quote! {
        @viewstart "home/view_start.rs"
        @name "home_index"
        @{
            view_context.insert_str("Title", "Rust HTML (rshtml) Home".to_string());
        }
        
        <h1>@view_context.get_str("Title")</h1>

        @mdfile_const "README.md"
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}

#[test]
pub fn example_web_app_home_view_start_view() {
    let html = quote::quote! {
        @name "home_view_start"
        @{
            // this code is executed before every view in this folder
            // unless the view uses @viewstart null or @viewstart ""
            view_context.insert_str("Layout", "shared/_layout.rs".to_string());
        }
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}
