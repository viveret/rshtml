use std::collections::HashMap;

use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::contexts::view_context::IViewContext;
use mvc_lib::view::rusthtml::helpers::html_helpers::HtmlHelpers;
use mvc_lib::view::rusthtml::helpers::ihtml_helpers::IHtmlHelpers;




pub struct CustomHtmlHelpers<'a> {
    html_helpers: HtmlHelpers<'a>,
    view_context: &'a dyn IViewContext,
    services: &'a dyn IServiceCollection
}

impl <'a> CustomHtmlHelpers<'a> {
    pub fn new(view_context: &'a dyn IViewContext, services: &'a dyn IServiceCollection) -> Self {
        Self {
            html_helpers: HtmlHelpers::new(view_context, services),
            view_context: view_context,
            services: services
        }
    }
}

impl <'a> IHtmlHelpers<'a> for CustomHtmlHelpers<'a> {
    fn form<'b, F>(self: &Self, method: http::method::Method, action: std::borrow::Cow<'b, str>, html_attrs: Option<HashMap<String, String>>, route_values: HashMap<String, String>, inner_render_fn: F) -> mvc_lib::view::rusthtml::html_string::HtmlString where F: Fn() -> mvc_lib::view::rusthtml::html_string::HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-form")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(default_html_attrs), html_attrs);
        self.html_helpers.form(method, action, combined_html_attrs, route_values, inner_render_fn)
    }

    fn submit(self: &Self, text: &str, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-btn s-btn__primary")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(default_html_attrs), html_attrs);
        self.html_helpers.submit(text, combined_html_attrs)
    }

    fn input(self: &Self, name: &str, input_type: &str, value: &str, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-input")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(default_html_attrs), html_attrs);
        self.html_helpers.input(name, input_type, value, combined_html_attrs)
    }

    fn hidden(self: &Self, name: &str, value: &str, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn checkbox(self: &Self, name: &str, checked: bool, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn textarea(self: &Self, name: &str, value: &str, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn label(self: &Self, for_name: &str, text: &str, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-label")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(default_html_attrs), html_attrs);
        self.html_helpers.label(for_name, text, combined_html_attrs)
    }

    fn select(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn select_multiple(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn option(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn option_selected(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn option_group(self: &Self, label: &str, options: Vec<(String, String)>, disabled: bool, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        todo!()
    }

    fn append_html_attrs_into_first(self: &Self, html_attrs_first: Option<HashMap<String, String>>, html_attrs_second: Option<HashMap<String, String>>) -> Option<HashMap<String, String>> {
        self.html_helpers.append_html_attrs_into_first(html_attrs_first, html_attrs_second)
    }

    fn append_html_attrs_into_new(self: &Self, html_attrs_first: Option<HashMap<String, String>>, html_attrs_second: Option<HashMap<String, String>>) -> HashMap<String, String> {
        self.html_helpers.append_html_attrs_into_new(html_attrs_first, html_attrs_second)
    }

    fn html_attrs_str_to_string(self: &Self, html_attrs: Option<HashMap<&str, &str>>) -> Option<HashMap<String, String>> {
        self.html_helpers.html_attrs_str_to_string(html_attrs)
    }

    fn html_attrs_array_tuple_str_to_string(self: &Self, html_attrs: &[(&str, &str)]) -> HashMap<String, String> {
        self.html_helpers.html_attrs_array_tuple_str_to_string(html_attrs)
    }

    fn html_attrs_to_string(self: &Self, html_attrs: Option<HashMap<String, String>>) -> String {
        self.html_helpers.html_attrs_to_string(html_attrs)
    }

    fn link<'b>(self: &Self, href: &'b str, text: &'b str, html_attrs: Option<HashMap<String, String>>) -> mvc_lib::view::rusthtml::html_string::HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-link")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(default_html_attrs), html_attrs);
        self.html_helpers.link(href, text, combined_html_attrs)
    }
}