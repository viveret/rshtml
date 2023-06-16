use std::borrow::Cow;
use std::collections::HashMap;

use crate::model_binder::imodel::IModel;
use crate::view::rusthtml::html_string::HtmlString;


// helpers for HTML views
pub trait IHtmlHelpers<'a, TModel: 'static + IModel> {
    // create a link
    // text: the text to display for the link.
    // href: the href for the link.
    // html_attrs: the HTML attributes for the link.
    // returns: the link HTML.
    fn link<'b>(self: &Self, href: &'b str, text: &'b str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a form group.
    fn form<'b, F>(self: &Self, method: http::method::Method, action: Cow<'b, str>, html_attrs: Option<&HashMap<String, String>>, inner_render_fn: F) -> HtmlString where F: Fn() -> HtmlString;

    // create a submit button with the given text for a form.
    // text: the text to display on the button.
    // returns: the submit button HTML.
    fn submit(self: &Self, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create an input with the given name and value for a form.
    // name: the name of the input.
    // value: the value of the input.
    // returns: the input HTML.
    fn input(self: &Self, name: &str, input_type: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a hidden input with the given name and value for a form.
    // name: the name of the input.
    // value: the value of the input.
    // returns: the input HTML.
    fn hidden(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a checkbox input with the given name and checked value for a form.
    // name: the name of the input.
    // checked: whether or not the checkbox is checked.
    // returns: the input HTML.
    fn checkbox(self: &Self, name: &str, checked: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a textarea with the given name and value for a form.
    // name: the name of the textarea.
    // value: the value of the textarea.
    // returns: the textarea HTML.
    fn textarea(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a label with the given for name and text for a form.
    // for_name: the for name of the label.
    // text: the text of the label.
    // returns: the label HTML.
    fn label(self: &Self, for_name: &str, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a label with the given for name and text for a form.
    // name: the name of the select.
    // options: the options for the select.
    // returns: the select HTML.
    fn select(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select with the given name and options for a form that allows multiple selections.
    // name: the name of the select.
    // options: the options for the select.
    // returns: the select HTML.
    fn select_multiple(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select option with the given value and text for a form.
    // value: the value of the option.
    // text: the text of the option.
    // disabled: whether or not the option is disabled.
    // returns: the option HTML.
    fn option(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select option with the given value and text for a form that is selected.
    // value: the value of the option.
    // text: the text of the option.
    // disabled: whether or not the option is disabled.
    // returns: the option HTML.
    fn option_selected(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select option group with the given label and options for a form.
    // label: the label of the option group.
    // options: the options for the option group.
    // disabled: whether or not the option group is disabled.
    // returns: the option group HTML.
    fn option_group(self: &Self, label: &str, options: Vec<(String, String)>, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // merge html attributes into the first set of html attributes.
    // html_attrs_first: the first set of html attributes.
    // html_attrs_second: the second set of html attributes.
    // returns: the first set of html attributes with the second set of html attributes merged into it.
    fn append_html_attrs_into_first(self: &Self, html_attrs_first: Option<&HashMap<String, String>>, html_attrs_second: Option<&HashMap<String, String>>) -> Option<HashMap<String, String>>;

    // merge html attributes into a new set of html attributes.
    // html_attrs_first: the first set of html attributes.
    // html_attrs_second: the second set of html attributes.
    // returns: the new set of html attributes.
    fn append_html_attrs_into_new(self: &Self, html_attrs_first: Option<&HashMap<String, String>>, html_attrs_second: Option<&HashMap<String, String>>) -> HashMap<String, String>;
    
    // convert a map of &str html attributes to a map of String html attributes.
    fn html_attrs_str_to_string(self: &Self, html_attrs: Option<&HashMap<&str, &str>>) -> Option<HashMap<String, String>>;

    // convert an array of &str html attributes to a map of String html attributes.
    fn html_attrs_array_tuple_str_to_string(self: &Self, html_attrs: &[(&str, &str)]) -> HashMap<String, String>;

    // convert a map of String html attributes to a string representation of the html attributes.
    // html_attrs: the map of String html attributes.
    // returns: the string representation of the html attributes.
    fn html_attrs_to_string(self: &Self, html_attrs: Option<&HashMap<String, String>>) -> String;

    // return an HTML string that contains the validation summary.
    fn validation_summary(self: &Self) -> HtmlString;


    // helpers for HTML views that have a model

    // create an input with the given name and value for a form.
    // name: the name of the input.
    // value: the value of the input.
    // returns: the input HTML.
    fn input_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), input_type: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a hidden input with the given name and value for a form.
    // name: the name of the input.
    // value: the value of the input.
    // returns: the input HTML.
    fn hidden_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a checkbox input with the given name and checked value for a form.
    // name: the name of the input.
    // checked: whether or not the checkbox is checked.
    // returns: the input HTML.
    fn checkbox_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a textarea with the given name and value for a form.
    // name: the name of the textarea.
    // value: the value of the textarea.
    // returns: the textarea HTML.
    fn textarea_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a label with the given for name and text for a form.
    // for_name: the for name of the label.
    // text: the text of the label.
    // returns: the label HTML.
    fn label_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a label with the given for name and text for a form.
    // name: the name of the select.
    // options: the options for the select.
    // returns: the select HTML.
    fn select_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select with the given name and options for a form that allows multiple selections.
    // name: the name of the select.
    // options: the options for the select.
    // returns: the select HTML.
    fn select_multiple_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select option with the given value and text for a form.
    // value: the value of the option.
    // text: the text of the option.
    // disabled: whether or not the option is disabled.
    // returns: the option HTML.
    fn option_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;

    // create a select option with the given value and text for a form that is selected.
    // value: the value of the option.
    // text: the text of the option.
    // disabled: whether or not the option is disabled.
    // returns: the option HTML.
    fn option_selected_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString;
}