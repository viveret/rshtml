use std::any::{Any, TypeId};
use std::error::Error;
use std::result::Result;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::contexts::view_context::IViewContext;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::IServiceCollection;


pub trait IView {
    // relative to root "views" folder
    fn get_path(self: &Self) -> String;

    // raw rust + HTML template data
    fn get_raw(self: &Self) -> String;

    // if the view defines a model type, this returns the type id
    fn get_model_type_name(self: &Self) -> Option<String>;

    // using template, render the view given the current data
    fn render(self: &Self, ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError>;
    
    // fn render_borrowed(self: &Self, ctx: Rc<dyn IViewContext>, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError>;

    // might add section renderers, the layout name, and "IsBeingRendered" flag
}