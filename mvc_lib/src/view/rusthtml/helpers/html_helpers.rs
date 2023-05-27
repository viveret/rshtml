use std::borrow::Cow;
use std::collections::HashMap;

use crate::core::html_buffer::{HtmlBuffer, IHtmlBuffer};
use crate::services::service_collection::IServiceCollection;
use crate::contexts::view_context::IViewContext;
use crate::view::rusthtml::html_string::HtmlString;

use super::ihtml_helpers::IHtmlHelpers;


// helpers for HTML views
pub struct HtmlHelpers<'a> {
    view_context: &'a dyn IViewContext,
    services: &'a dyn IServiceCollection
}

impl <'a> HtmlHelpers<'a> {
    pub fn new(view_context: &'a dyn IViewContext, services: &'a dyn IServiceCollection) -> Self {
        Self {
            view_context: view_context,
            services: services
        }
    }
}

impl <'a> IHtmlHelpers<'a> for HtmlHelpers<'a> {
    fn form<'b, F>(self: &Self, method: http::method::Method, action: Cow<'b, str>, html_attrs: Option<&HashMap<String, String>>, inner_render_fn: F) -> HtmlString where F: Fn() -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
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
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: format!("<button type=\"submit\" {}>{}</button>", html_attrs_str, html_escape::encode_text(&text)) }
    }

    fn input(self: &Self, name: &str, input_type: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
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
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: 
            format!("<input type=\"hidden\" name=\"{}\" value=\"{}\" {}/>", 
                html_escape::encode_text(&name), 
                html_escape::encode_text(&value),
                html_attrs_str,
            ) 
        }
    }

    fn checkbox(self: &Self, name: &str, checked: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: 
            format!("<input type=\"checkbox\" name=\"{}\" {} {}/>", 
                html_escape::encode_text(&name), 
                if checked { "checked" } else { "" },
                html_attrs_str,
            ) 
        }
    }

    fn textarea(self: &Self, name: &str, value: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: 
            format!("<textarea name=\"{}\" {}>{}</textarea>", 
            html_escape::encode_text(&name), 
            html_attrs_str,
            html_escape::encode_text(&value)) 
        }
    }

    fn label(self: &Self, for_name: &str, text: &str, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: 
            format!("<label for=\"{}\" {}>{}</label>", 
                html_escape::encode_text(&for_name),
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }

    fn select(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        let mut html = format!("<select name=\"{}\" {}>", html_escape::encode_text(&name), html_attrs_str);
        for option in options {
            html.push_str(&self.option(&option.0, &option.1, false, None).content);
        }
        html = format!("{}</select>", html);
        HtmlString { content: html }
    }

    fn select_multiple(self: &Self, name: &str, options: Vec<(String, String)>, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        let mut html = format!("<select name=\"{}\" multiple {}>", html_escape::encode_text(&name), html_attrs_str);
        for option in options {
            html.push_str(&self.option(&option.0, &option.1, false, None).content);
        }
        html = format!("{}</select>", html);
        HtmlString { content: html }
    }

    fn option(self: &Self, value: &str, text: &str, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
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
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: 
            format!("<option value=\"{}\" selected {} {}>{}</option>", 
                html_escape::encode_text(&value),
                if disabled { "disabled" } else { "" },
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }

    fn option_group(self: &Self, label: &str, options: Vec<(String, String)>, disabled: bool, html_attrs: Option<&HashMap<String, String>>) -> HtmlString {
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
                        let new_value = html_attrs_first.get(key).unwrap().to_string() + " " + &value;
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
        self.append_html_attrs_into_first(copy_first, html_attrs_second).unwrap().clone()
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
        let html_attrs_str = self.html_attrs_to_string(html_attrs);
        HtmlString { content: 
            format!("<a href=\"{}\" {}>{}</a>", 
                html_escape::encode_text(&href),
                html_attrs_str,
                html_escape::encode_text(&text)
            )
        }
    }
}