use crate::view::rusthtml::html_string::HtmlString;

// form helpers for generating HTML forms.
// these helpers are used in the view to generate HTML forms and form elements.
// they are used in the view like this:
// @using (HtmlHelpers form = FormHelpers::new())
// {
//      @form::label("Name", "Name")
//      @form::input("Name", "")
//      <br />
//      @form::label("Age", "Age")
//      @form::input("Age", "")
//      <br />
//      @form::submit("Submit")
// }
pub struct FormHelpers {}

impl FormHelpers {
    // create a new FormHelpers.
    pub fn new() -> Self {
        Self { }
    }

    // create a submit button with the given text for a form.
    // text: the text to display on the button.
    // returns: the submit button HTML.
    pub fn submit(self: &Self, text: &str) -> HtmlString {
        HtmlString { content: format!("<button type=\"submit\">{}</button>", html_escape::encode_text(&text)) }
    }

    // create an input with the given name and value for a form.
    // name: the name of the input.
    // value: the value of the input.
    // returns: the input HTML.
    pub fn input(self: &Self, name: &str, value: &str) -> HtmlString {
        HtmlString { content: format!("<input type=\"text\" name=\"{}\" value=\"{}\" />", html_escape::encode_text(&name), html_escape::encode_text(&value)) }
    }

    // create a hidden input with the given name and value for a form.
    // name: the name of the input.
    // value: the value of the input.
    // returns: the input HTML.
    pub fn hidden(self: &Self, name: &str, value: &str) -> HtmlString {
        HtmlString { content: format!("<input type=\"hidden\" name=\"{}\" value=\"{}\" />", html_escape::encode_text(&name), html_escape::encode_text(&value)) }
    }

    // create a checkbox input with the given name and checked value for a form.
    // name: the name of the input.
    // checked: whether or not the checkbox is checked.
    // returns: the input HTML.
    pub fn checkbox(self: &Self, name: &str, checked: bool) -> HtmlString {
        HtmlString { content: format!("<input type=\"checkbox\" name=\"{}\" {} />", html_escape::encode_text(&name), if checked { "checked" } else { "" }) }
    }

    // create a textarea with the given name and value for a form.
    // name: the name of the textarea.
    // value: the value of the textarea.
    // returns: the textarea HTML.
    pub fn textarea(self: &Self, name: &str, value: &str) -> HtmlString {
        HtmlString { content: format!("<textarea name=\"{}\">{}</textarea>", html_escape::encode_text(&name), html_escape::encode_text(&value)) }
    }

    // create a label with the given for name and text for a form.
    // for_name: the for name of the label.
    // text: the text of the label.
    // returns: the label HTML.
    pub fn label(self: &Self, for_name: &str, text: &str) -> HtmlString {
        HtmlString { content: format!("<label for=\"{}\">{}</label>", html_escape::encode_text(&for_name), html_escape::encode_text(&text)) }
    }

    // create a label with the given for name and text for a form.
    // name: the name of the select.
    // options: the options for the select.
    // returns: the select HTML.
    pub fn select(self: &Self, name: &str, options: Vec<(String, String)>) -> HtmlString {
        let mut html = format!("<select name=\"{}\">", html_escape::encode_text(&name));
        for option in options {
            html = format!("{}<option value=\"{}\">{}</option>", html, html_escape::encode_text(&option.0), html_escape::encode_text(&option.1));
        }
        html = format!("{}</select>", html);
        HtmlString { content: html }
    }

    // create a select with the given name and options for a form that allows multiple selections.
    // name: the name of the select.
    // options: the options for the select.
    // returns: the select HTML.
    pub fn select_multiple(self: &Self, name: &str, options: Vec<(String, String)>) -> HtmlString {
        let mut html = format!("<select name=\"{}\" multiple>", html_escape::encode_text(&name));
        for option in options {
            html = format!("{}<option value=\"{}\">{}</option>", html, html_escape::encode_text(&option.0), html_escape::encode_text(&option.1));
        }
        html = format!("{}</select>", html);
        HtmlString { content: html }
    }

    // create a select option with the given value and text for a form.
    // value: the value of the option.
    // text: the text of the option.
    // disabled: whether or not the option is disabled.
    // returns: the option HTML.
    pub fn option(self: &Self, value: &str, text: &str, disabled: bool) -> HtmlString {
        HtmlString { content: format!("<option value=\"{}\" {}>{}</option>", html_escape::encode_text(&value), if disabled { "disabled" } else { "" }, html_escape::encode_text(&text)) }
    }

    // create a select option with the given value and text for a form that is selected.
    // value: the value of the option.
    // text: the text of the option.
    // disabled: whether or not the option is disabled.
    // returns: the option HTML.
    pub fn option_selected(self: &Self, value: &str, text: &str, disabled: bool) -> HtmlString {
        HtmlString { content: format!("<option value=\"{}\" selected {}>{}</option>", html_escape::encode_text(&value), if disabled { "disabled" } else { "" }, html_escape::encode_text(&text)) }
    }

    // create a select option group with the given label and options for a form.
    // label: the label of the option group.
    // options: the options for the option group.
    // disabled: whether or not the option group is disabled.
    // returns: the option group HTML.
    pub fn option_group(self: &Self, label: &str, options: Vec<(String, String)>, disabled: bool) -> HtmlString {
        let mut html = format!("<optgroup label=\"{}\">", html_escape::encode_text(&label));
        for option in options {
            html = format!("{}<option value=\"{}\" {}>{}</option>", html, html_escape::encode_text(&option.0), if disabled { "disabled" } else { "" }, html_escape::encode_text(&option.1));
        }
        html = format!("{}</optgroup>", html);
        HtmlString { content: html }
    }
}