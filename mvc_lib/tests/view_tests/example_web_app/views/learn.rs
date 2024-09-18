use mvc_lib::view::macro_impl::rusthtml_view_macro_impl;

// #[test]
// pub fn example_web_app_learn__view() {
//     let html = quote::quote! {
//     };

//     let result = rusthtml_view_macro_impl(html);
//     let result_len = result.into_iter().count();

//     assert_ne!(0, result_len)
// }


#[test]
pub fn example_web_app_learn_details_view() {
    let html = quote::quote! {
        @viewstart "learn/_view_start.rs"
        @model crate::view_models::learn::DetailsViewModel
        @name "learn_details"
        @{
            view_context.insert_str("Title", "Learn Rust HTML (rshtml)".to_string());
        }
        
        @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Learn"), None, None).as_str(), "< Back to learning index", None)
        
        <h1>@&view_context.get_str("Title")</h1>
    
        // @htmlfile "home/index.html"
        // @rshtmlfile "home/index.rshtml"
        @mdfile_nocache &model.path.clone()
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}

#[test]
pub fn example_web_app_learn_index_view() {
    let html = quote::quote! {
        @viewstart "learn/_view_start.rs"
        @model crate::view_models::learn::IndexViewModel
        @name "learn_index"
        @{
            view_context.insert_str("Title", "Learn Rust HTML (rshtml)".to_string());
        }
        
        <h1>@view_context.get_str("Title")</h1>

        <ul>
        @for doc_name in model.learn_docs.iter() {
            // doc_name as text and as href id
            let href = url.url_action(false, Some(false), None, Some("details"), Some("Learn"), None, Some(&RouteValuesBuilder::build_area(doc_name)));
            <li>
                @html.link(href.as_str(), doc_name, None)
            </li>
        }
        </ul>
        
        @mdfile_nocache "docs/learn/README.md"
    };

    let result = rusthtml_view_macro_impl(html);
    let result_len = result.into_iter().count();

    assert_ne!(0, result_len)
}