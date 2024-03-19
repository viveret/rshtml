use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;

use as_any::Downcast;

use crate::attributes::display_name_attribute::DisplayNameAttribute;
use crate::core::html_buffer::{HtmlBuffer, IHtmlBuffer};
use crate::core::type_info::TypeInfo;
use crate::model_binder::imodel::{IModel, AnyIModel};
use crate::model_binder::model_validation_result::ModelValidationResult;
use crate::services::service_collection::IServiceCollection;
use crate::contexts::view_context::IViewContext;
use crate::view::rusthtml::html_string::HtmlString;

use super::ihtml_helpers::IHtmlHelpers;


// helpers for HTML views
pub struct HtmlHelpers<'a, TModel: 'static + IModel> {
    view_context: &'a dyn IViewContext,
    _services: &'a dyn IServiceCollection,
    x: PhantomData<TModel>,
}

impl <'a, TModel: 'static + IModel> HtmlHelpers<'a, TModel> {
    pub fn new(view_context: &'a dyn IViewContext, services: &'a dyn IServiceCollection) -> Self {
        Self {
            view_context: view_context,
            _services: services,
            x: PhantomData {},
        }
    }
}

impl <'a, TModel: 'static + IModel> IHtmlHelpers<'a, TModel> for HtmlHelpers<'a, TModel> {
    fn form<'b, F>(self: &Self, method: http::method::Method, action: Cow<'b, str>, html_attrs: Option<&HashMap<String, String>>, inner_render_fn: F) -> HtmlString where F: Fn() -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        let html_output = HtmlBuffer::new();
        html_output.write_html_str(
            format!("<form method=\"{}\" action=\"{}\" {}>", 
                html_escape::encode_text(method.as_str()),
                html_escape::encode_text(&action),
                html_attrs_str
            ).as_str()
        );
        html_output.write_html(inner_render_fn());
        html_output.write_html_str("</form>");
        html_output.collect_html()
    }

    fn submit(self: &Self, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: format!("<button type=\"submit\" {}>{}</button>", html_attrs_str, html_escape::encode_text(&text)) }
    }

    fn input(self: &Self, name: &str, input_type: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<input type=\"{}\" name=\"{}\" value=\"{}\" {}/>", 
                html_escape::encode_text(&input_type), 
                html_escape::encode_text(&name), 
                html_escape::encode_text(&value),
                html_attrs_str,
            )
        }
    }

    fn hidden(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<input type=\"hidden\" name=\"{}\" value=\"{}\" {}/>", 
                html_escape::encode_text(&name), 
                html_escape::encode_text(&value),
                html_attrs_str,
            ) 
        }
    }

    fn checkbox(self: &Self, name: &str, checked: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<input type=\"checkbox\" name=\"{}\" {} {}/>", 
                html_escape::encode_text(&name), 
                if checked { "checked" } else { "" },
                html_attrs_str,
            ) 
        }
    }

    fn textarea(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<textarea name=\"{}\" {}>{}</textarea>", 
            html_escape::encode_text(&name), 
            html_attrs_str,
            html_escape::encode_text(&value)) 
        }
    }

    fn label(self: &Self, for_name: &str, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<label for=\"{}\" {}>{}</label>", 
                html_escape::encode_text(&for_name),
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }

    fn select(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        let mut html = format!("<select name=\"{}\" {}>", html_escape::encode_text(&name), html_attrs_str);
        for option in options {
            html.push_str(&<HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::option(self, &option.0, &option.1, false, None).content);
        }
        html = format!("{}</select>", html);
        HtmlString { content: html }
    }

    fn select_multiple(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        let mut html = format!("<select name=\"{}\" multiple {}>", html_escape::encode_text(&name), html_attrs_str);
        for option in options {
            html.push_str(&<HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::option(self, &option.0, &option.1, false, None).content);
        }
        html = format!("{}</select>", html);
        HtmlString { content: html }
    }

    fn option(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<option value=\"{}\" {} {}>{}</option>", 
                html_escape::encode_text(&value),
                if disabled { "disabled" } else { "" },
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }

    fn option_selected(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<option value=\"{}\" selected {} {}>{}</option>", 
                html_escape::encode_text(&value),
                if disabled { "disabled" } else { "" },
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }

    fn option_group(self: &Self, label: &str, options: Vec<(String, String)>, disabled: bool, _html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let mut html = format!("<optgroup label=\"{}\">", html_escape::encode_text(&label));
        for option in options {
            html = format!("{}<option value=\"{}\" {}>{}</option>", html, html_escape::encode_text(&option.0), if disabled { "disabled" } else { "" }, html_escape::encode_text(&option.1));
        }
        html = format!("{}</optgroup>", html);
        HtmlString { content: html }
    }

    fn append_html_attrs_into_first(self: &Self, html_attrs_first: Option<&HashMap<String, String>>, html_attrs_second: Option<&HashMap<String, String>>) -> Option<HashMap<String, String>> {
        if let Some(html_attrs_first) = html_attrs_first {
            let mut new_first = html_attrs_first.clone();
            if let Some(html_attrs_second) = html_attrs_second {
                for (key, value) in html_attrs_second {
                    if html_attrs_first.contains_key(key.as_str()) {
                        let new_value = html_attrs_first.get(key).expect("append_html_attrs_into_first error").to_string() + " " + &value;
                        new_first.insert(key.clone(), new_value);
                    } else {
                        new_first.insert(key.clone(), value.clone());
                    }
                }
            }
            Some(new_first)
        } else {
            html_attrs_second.cloned()
        }
    }

    fn append_html_attrs_into_new(self: &Self, html_attrs_first: Option<&HashMap<String, String>>, html_attrs_second: Option<&HashMap<String, String>>) -> HashMap<String, String> {
        let copy_first = html_attrs_first.clone();
        <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::append_html_attrs_into_first(self, copy_first, html_attrs_second).expect("append_html_attrs_into_first").clone()
    }

    fn html_attrs_str_to_string(self: &Self, html_attrs: Option<&HashMap<&str, &str>>) -> Option<HashMap<String, String>> {
        if let Some(html_attrs) = html_attrs {
            Some(html_attrs.iter().map(|x| (x.0.to_string().clone(), x.1.to_string().clone())).collect())
        } else {
            None
        }
    }

    fn html_attrs_array_tuple_str_to_string(self: &Self, html_attrs: &[(&str, &str)]) -> HashMap<String, String> {
        html_attrs.iter().map(|x| (x.0.to_string(), x.1.to_string())).collect()
    }

    fn html_attrs_to_string(self: &Self, html_attrs: Option<&HashMap<String, String>>) -> String {
        if let Some(html_attrs) = html_attrs {
            let mut html = String::new();
            for (key, value) in html_attrs {
                html.push_str(
                    format!("{}=\"{}\" ", 
                        html_escape::encode_double_quoted_attribute(&key),
                        html_escape::encode_double_quoted_attribute(&value)
                    ).as_str()
                );
            }
            html
        } else {
            String::new()
        }
    }

    fn link<'b>(self: &Self, href: &'b str, text: &'b str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = <HtmlHelpers<'_, TModel> as IHtmlHelpers<'_, TModel>>::html_attrs_to_string(self, html_attrs);
        HtmlString { content: 
            format!("<a href=\"{}\" {}>{}</a>", 
                html_escape::encode_text(&href),
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }

    fn input_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), input_type: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        if let Some(viewmodel) = self.view_context.get_viewmodel() {
            // first check if it is anyimodel
            let r = if let Some(viewmodel) = viewmodel.downcast_ref::<AnyIModel>() {
                (expr.0)(viewmodel.get_underlying_value().downcast_ref::<TModel>().expect("input_for error").clone())
            } else if let Some(viewmodel) = viewmodel.as_any().downcast_ref::<TModel>() {
                (expr.0)(viewmodel)
            } else {
                panic!("viewmodel is not AnyIModel or TModel ({:?}) (it is {:?})", TypeId::of::<TModel>(), viewmodel.get_type_info());
            };
            let value = r.to_string();
            self.input(property_name.as_str(), input_type, value.as_str(), html_attrs)
        } else {
            self.input(property_name.as_str(), input_type, "", html_attrs)
        }
    }

    fn hidden_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        if let Some(viewmodel) = self.view_context.get_viewmodel() {
            // first check if it is anyimodel
            let r = if let Some(viewmodel) = viewmodel.downcast_ref::<AnyIModel>() {
                (expr.0)(viewmodel.get_underlying_value().downcast_ref::<TModel>().expect("hidden_for error"))
            } else if let Some(viewmodel) = viewmodel.as_any().downcast_ref::<TModel>() {
                (expr.0)(viewmodel)
            } else {
                panic!("viewmodel is not AnyIModel or TModel ({:?}) (it is {:?})", TypeId::of::<TModel>(), viewmodel.get_type_info());
            };
            let value = r.to_string();
            self.hidden(property_name.as_str(), value.as_str(), html_attrs)
        } else {
            self.hidden(property_name.as_str(), "", html_attrs)
        }
    }

    fn checkbox_for<TFn: 'static + Fn(&TModel) -> bool>(&self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        if let Some(viewmodel) = self.view_context.get_viewmodel() {
            // first check if it is anyimodel
            let value = if let Some(viewmodel) = viewmodel.downcast_ref::<AnyIModel>() {
                (expr.0)(viewmodel.get_underlying_value().downcast_ref::<TModel>().expect("checkbox_for error"))
            } else if let Some(viewmodel) = viewmodel.as_any().downcast_ref::<TModel>() {
                (expr.0)(viewmodel)
            } else {
                panic!("viewmodel is not AnyIModel or TModel ({:?}) (it is {:?})", TypeId::of::<TModel>(), viewmodel.get_type_info());
            };
            self.checkbox(property_name.as_str(), value, html_attrs)
        } else {
            self.checkbox(property_name.as_str(), false, html_attrs)
        }
    }

    fn textarea_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        if let Some(viewmodel) = self.view_context.get_viewmodel() {
            // first check if it is anyimodel
            let value = if let Some(viewmodel) = viewmodel.downcast_ref::<AnyIModel>() {
                (expr.0)(viewmodel.get_underlying_value().downcast_ref::<TModel>().expect("textarea_for error"))
            } else if let Some(viewmodel) = viewmodel.as_any().downcast_ref::<TModel>() {
                (expr.0)(viewmodel)
            } else {
                panic!("viewmodel is not AnyIModel or TModel ({:?}) (it is {:?})", TypeId::of::<TModel>(), viewmodel.get_type_info());
            };
            self.textarea(property_name.as_str(), value.to_string().as_str(), html_attrs)
        } else {
            self.textarea(property_name.as_str(), "", html_attrs)
        }
    }

    fn label_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(self: &Self, expr: (TFn, proc_macro2::TokenStream), html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        // first get property name
        let property_name = extract_property_name(expr.1);

        // then check for attribute on viewmodel if it exists
        if let Some(viewmodel) = self.view_context.get_viewmodel().as_ref() {
            let property = viewmodel.get_property(&property_name);
            if let Some(property) = property {
                let attribute = property.get_attribute(&TypeInfo::of::<DisplayNameAttribute>());
                if let Some(attribute) = attribute {
                    let label = attribute.get_name();
                    return self.label(property_name.as_str(), label.as_str(), html_attrs);
                }
            }
        }

        // else return the property name
        self.label(property_name.as_str(), property_name.as_str(), html_attrs)
    }

    fn select_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        self.select(property_name.as_str(), options, html_attrs)
    }

    fn select_multiple_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        self.select_multiple(property_name.as_str(), options, html_attrs)
    }

    fn option_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        self.option(property_name.as_str(), property_name.as_str(), disabled, html_attrs)
    }

    fn option_selected_for<TProperty: 'static + ToString, TFn: 'static + Fn(&TModel) -> TProperty>(&self, expr: (TFn, proc_macro2::TokenStream), disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let property_name = extract_property_name(expr.1);
        self.option_selected(property_name.as_str(), property_name.as_str(), disabled, html_attrs)
    }

    fn validation_summary(self: &Self) -> HtmlString {
        if let Some(result) = self.view_context.get_request_context().get_model_validation_result() {
            match result {
                ModelValidationResult::Ok(_) |
                ModelValidationResult::OkNone => {
                    HtmlString::empty()
                },
                ModelValidationResult::PropertyError(_, property_name, error) => {
                    HtmlString::new_data_string(format!("{}: {}", property_name, error.to_string()))
                },
                ModelValidationResult::MultipleErrors(_, errors) => {
                    let mut html = String::new();
                    for error in errors {
                        html.push_str(&format!("{}: {}<br/>", error.0, error.1.to_string()));
                    }
                    HtmlString::new_data_string(html)
                },
                ModelValidationResult::ModelError(_, e) => {
                    HtmlString::new_data_string(e.to_string())
                },
                ModelValidationResult::OtherError(e) => {
                    HtmlString::new_data_string(e.to_string())
                },
            }
        } else {
            HtmlString::empty()
        }
    }
}

fn extract_property_name(expr: proc_macro2::TokenStream) -> String {
    expr.into_iter().last().expect("expr.into_iter().last()").to_string()
}