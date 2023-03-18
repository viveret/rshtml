use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::contexts::controller_context::ControllerContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::request_context::RequestContext;

use rusthtml::html_string::HtmlString;
use crate::view::iview::IView;
use crate::view::view_renderer::IViewRenderer;


pub trait IViewContext: Send + Sync {
    fn recurse_into_new_context(self: &Self, view: Rc<Box<dyn IView>>) -> Arc<RwLock<dyn IViewContext>>;

    fn write_html(self: &Self, html: &HtmlString);
    fn write_content(self: &Self, content: &String);

    fn get_view_renderer(self: &Self) -> Rc<Box<dyn IViewRenderer>>;
    fn get_viewdata(self: &Self) -> Rc<HashMap<String, String>>;
    fn get_view(self: &Self) -> Rc<Box<dyn IView>>;

    fn get_controller_ctx(self: &Self) -> Rc<RefCell<ControllerContext>>;
    fn get_response_ctx(self: &Self) -> Rc<RefCell<ResponseContext>>;
    fn get_request_ctx(self: &Self) -> Rc<RequestContext>;
}

pub struct ViewContext {
    view: Rc<Box<dyn IView>>,
    viewdata: Rc<HashMap<String, String>>,
    viewmodel: Option<Rc<Box<dyn Any>>>,
    view_renderer: Rc<Box<dyn IViewRenderer>>,
    controller_ctx: Rc<RefCell<ControllerContext>>,
    response_ctx: Rc<RefCell<ResponseContext>>,
    request_ctx: Rc<RequestContext>,
}
unsafe impl Send for ViewContext {}
unsafe impl Sync for ViewContext {}


impl ViewContext {
    pub fn new(
                view: Rc<Box<dyn IView>>,
                viewmodel: Option<Rc<Box<dyn Any>>>,
                view_renderer: Rc<Box<dyn IViewRenderer>>,
                controller_ctx: Rc<RefCell<ControllerContext>>,
                response_ctx: Rc<RefCell<ResponseContext>>,
                request_ctx: Rc<RequestContext>) -> Self {
        Self {
            viewdata: Rc::new(HashMap::new()),
            view: view,
            viewmodel: viewmodel,
            view_renderer: view_renderer,
            controller_ctx: controller_ctx,
            response_ctx: response_ctx,
            request_ctx: request_ctx,
        }
    }

    // pub fn as_box(self: Self) -> Box<dyn IViewContext> {
    //     Box::new(self)
    // }
}

impl IViewContext for ViewContext {
    fn recurse_into_new_context(self: &Self, view: Rc<Box<dyn IView>>) -> Arc<RwLock<dyn IViewContext>> {
        Arc::new(RwLock::new(Self::new(
            view,
            self.viewmodel.clone(),
            self.view_renderer.clone(),
            self.controller_ctx.clone(),
            self.response_ctx.clone(),
            self.request_ctx.clone()
        )))
    }

    fn write_html(self: &Self, _html: &HtmlString) {

    }

    fn write_content(self: &Self, _content: &String) {

    }

    fn get_view_renderer(self: &Self) -> Rc<Box<dyn IViewRenderer>> {
        self.view_renderer.clone()
    }

    fn get_viewdata(self: &Self) -> Rc<HashMap<String, String>> {
        self.viewdata.clone()
    }

    fn get_view(self: &Self) -> Rc<Box<dyn IView>> {
        self.view.clone()
    }

    fn get_controller_ctx(self: &Self) -> Rc<RefCell<ControllerContext>> {
        self.controller_ctx.clone()
    }

    fn get_response_ctx(self: &Self) -> Rc<RefCell<ResponseContext>> {
        self.response_ctx.clone()
    }

    fn get_request_ctx(self: &Self) -> Rc<RequestContext> {
        self.request_ctx.clone()
    }
}