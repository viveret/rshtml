use std::collections::HashMap;
use std::marker::PhantomData;

use mvc_lib::model_binder::imodel::IModel;
use mvc_lib::services::service_collection::IServiceCollection;
use mvc_lib::contexts::view_context::IViewContext;
use mvc_lib::view::rusthtml::helpers::html_helpers::HtmlHelpers;
use mvc_lib::view::rusthtml::helpers::ihtml_helpers::IHtmlHelpers;
use mvc_lib::view::rusthtml::html_string::HtmlString;


// this is the custom html helpers for the site. this is where you can add your own custom html helpers or override the default ones.
pub struct CustomHtmlHelpers<'a, TModel: 'static + IModel> {
    html_helpers: HtmlHelpers<'a, TModel>,
    _view_context: &'a dyn IViewContext,
    _services: &'a dyn IServiceCollection,
    x: PhantomData<TModel>
}

impl <'a, TModel: 'static + IModel> CustomHtmlHelpers<'a, TModel> {
    pub fn new(view_context: &'a dyn IViewContext, services: &'a dyn IServiceCollection) -> Self {
        Self {
            html_helpers: HtmlHelpers::new(view_context, services),
            _view_context: view_context,
            _services: services,
            x: PhantomData {}
        }
    }
}

impl <'a, TModel: 'static + IModel> IHtmlHelpers<'a, TModel> for CustomHtmlHelpers<'a, TModel> {
    fn form<'b, F>(self: &Self, method: http::method::Method, action: std::borrow::Cow<'b, str>, html_attrs: Option<&HashMap<String, String>>, inner_render_fn: F) -> HtmlString where F: Fn() -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-form")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::form::<F>(&self.html_helpers, method, action, combined_html_attrs.as_ref(), inner_render_fn)
    }

    fn submit(self: &Self, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-btn s-btn__primary")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::submit(&self.html_helpers, text, combined_html_attrs.as_ref())
    }

    fn input(self: &Self, name: &str, input_type: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-input")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::input(&self.html_helpers, name, input_type, value, combined_html_attrs.as_ref())
    }

    fn hidden(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.hidden(name, value, html_attrs)
    }

    fn checkbox(self: &Self, name: &str, checked: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.checkbox(name, checked, html_attrs)
    }

    fn textarea(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.textarea(name, value, html_attrs)
    }

    fn label(self: &Self, for_name: &str, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-label")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::label(&self.html_helpers, for_name, text, combined_html_attrs.as_ref())
    }

    fn select(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.select(name, options, html_attrs)
    }

    fn select_multiple(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.select_multiple(name, options, html_attrs)
    }

    fn option(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.option(value, text, disabled, html_attrs)
    }

    fn option_selected(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.option_selected(value, text, disabled, html_attrs)
    }

    fn option_group(self: &Self, label: &str, options: Vec<(String, String)>, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.option_group(label, options, disabled, html_attrs)
    }

    fn append_html_attrs_into_first(self: &Self, html_attrs_first: Option<&HashMap<String, String>>, html_attrs_second: Option<&HashMap<String, String>>) -> Option<HashMap<String, String>> {
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::append_html_attrs_into_first(&self.html_helpers, html_attrs_first, html_attrs_second)
    }

    fn append_html_attrs_into_new(self: &Self, html_attrs_first: Option<&HashMap<String, String>>, html_attrs_second: Option<&HashMap<String, String>>) -> HashMap<String, String> {
        self.html_helpers.append_html_attrs_into_new(html_attrs_first, html_attrs_second)
        // <CustomHtmlHelpers<'_> as IHtmlHelpers<'_, TModel>>::html_attrs_array_tuple_str_to_string(&self.html_helpers, html_attrs_first, html_attrs_second)
    }

    fn html_attrs_str_to_string(self: &Self, html_attrs: Option<&HashMap<&str, &str>>) -> Option<HashMap<String, String>> {
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_str_to_string(&self.html_helpers, html_attrs)
    }

    fn html_attrs_array_tuple_str_to_string(self: &Self, html_attrs: &[(&str, &str)]) -> HashMap<String, String> {
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_array_tuple_str_to_string(&self.html_helpers, html_attrs)
    }

    fn html_attrs_to_string(self: &Self, html_attrs: Option<&HashMap<String, String>>) -> String {
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(&self.html_helpers, html_attrs)
    }

    fn link<'b>(self: &Self, href: &'b str, text: &'b str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-link")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::link(&self.html_helpers, href, text, combined_html_attrs.as_ref())
    }

    fn input_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), input_type: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-input")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::input_for(&self.html_helpers, expr, input_type, combined_html_attrs.as_ref())
    }

    fn label_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let default_html_attrs = self.html_attrs_array_tuple_str_to_string(&[("class", "s-label")]);
        let combined_html_attrs = self.append_html_attrs_into_first(Some(&default_html_attrs), html_attrs);
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::label_for(&self.html_helpers, expr, combined_html_attrs.as_ref())
    }

    fn hidden_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.hidden_for(expr, html_attrs)
    }

    fn checkbox_for<TFn: 'static + Fn(&TModel) -> bool>(self: &Self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.checkbox_for(expr, html_attrs)
    }

    fn textarea_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.textarea_for(expr, html_attrs)
    }

    fn select_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.select_for(expr, options, html_attrs)
    }

    fn select_multiple_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.select_multiple_for(expr, options, html_attrs)
    }

    fn option_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.option_for(expr, disabled, html_attrs)
    }

    fn option_selected_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        self.html_helpers.option_selected_for(expr, disabled, html_attrs)
    }

    fn validation_summary(self: &Self) -> HtmlString {
        self.html_helpers.validation_summary()
    }
}