use std::any::{Any, TypeId};
use std::error::Error;
use std::result::Result;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use rusthtml::html_string::HtmlString;

use crate::contexts::view_context::IViewContext;

use crate::services::service_collection::IServiceCollection;


pub struct PrecompiledRustHtmlView {
    model_type_name: Option<String>,
    render_fn: Box<dyn Fn() -> Result<Rc<HtmlString>, Rc<RustHtmlError>>>,
    when_compiled: chrono::prelude::Local::now()
    // might add section renderers, the layout name, and "IsBeingRendered" flag
}

impl IView for precompiled_rusthtml_view {
    pub fn get_path(self: &Self) -> String {
        panic!("Path not available for precompiled views");
    }

    pub fn get_raw(self: &Self) -> String {
        panic!("Raw not available for precompiled views");
    }

    // if the view defines a model type, this returns the type id
    pub fn get_model_type_name(self: &Self) -> Option<String> {
        return self.model_type_name;
    }

    // using template, render the view given the current data
    pub fn render(self: &Self, ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError> {
        self.render_fn()
    }
}