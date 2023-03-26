use std::any::{Any, TypeId};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::contexts::controller_context::ControllerContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::request_context::RequestContext;

use crate::view::rusthtml::html_string::HtmlString;
use crate::view::iview::IView;
use crate::view::view_renderer::IViewRenderer;


pub trait IViewContext: Send + Sync {
    fn recurse_into_new_context(self: &Self, view: Rc<dyn IView>) -> Box<dyn IViewContext>;

    fn write_html(self: &Self, html: HtmlString);
    fn write_html_str(self: &Self, html: &str);
    fn write_content(self: &Self, content: String);
    fn collect_html(self: &Self) -> HtmlString;

    fn get_view_renderer(self: &Self) -> Rc<dyn IViewRenderer>;
    fn get_viewdata(self: &Self) -> Rc<HashMap<String, String>>;
    fn get_viewmodel(self: &Self) -> Rc<Option<Box<dyn Any>>>;
    fn get_view(self: &Self) -> Rc<dyn IView>;

    fn get_controller_ctx(self: &Self) -> Rc<RefCell<ControllerContext>>;
    fn get_response_ctx(self: &Self) -> Rc<RefCell<ResponseContext>>;
    fn get_request_ctx(self: &Self) -> Rc<RequestContext>;
}

pub struct ViewContext {
    view: Rc<dyn IView>,
    viewdata: Rc<HashMap<String, String>>,
    viewmodel: Rc<Option<Box<dyn Any>>>,
    view_renderer: Rc<dyn IViewRenderer>,
    controller_ctx: Rc<RefCell<ControllerContext>>,
    response_ctx: Rc<RefCell<ResponseContext>>,
    request_ctx: Rc<RequestContext>,
    html_buffer: RefCell<String>,
}
unsafe impl Send for ViewContext {}
unsafe impl Sync for ViewContext {}


impl ViewContext {
    pub fn new(
                view: Rc<dyn IView>,
                viewmodel: Rc<Option<Box<dyn Any>>>,
                view_renderer: Rc<dyn IViewRenderer>,
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
            html_buffer: RefCell::new(String::new()),
        }
    }
}

impl IViewContext for ViewContext {
    fn recurse_into_new_context(self: &Self, view: Rc<dyn IView>) -> Box<dyn IViewContext> {
        Box::new(Self::new(
            view,
            self.viewmodel.clone(),
            self.view_renderer.clone(),
            self.controller_ctx.clone(),
            self.response_ctx.clone(),
            self.request_ctx.clone()
        ))
    }

    fn write_html(self: &Self, html: HtmlString) {
        self.write_html_str(html.content.as_str());
    }

    fn write_html_str(self: &Self, html: &str) {
        self.html_buffer.borrow_mut().push_str(html);
    }

    fn write_content(self: &Self, content: String) {
        self.write_html(HtmlString::new_data_string(content))
    }

    fn collect_html(self: &Self) -> HtmlString {
        HtmlString::new_from_html(self.html_buffer.borrow().clone())
    }

    fn get_view_renderer(self: &Self) -> Rc<dyn IViewRenderer> {
        self.view_renderer.clone()
    }

    fn get_viewdata(self: &Self) -> Rc<HashMap<String, String>> {
        self.viewdata.clone()
    }

    fn get_viewmodel(self: &Self) -> Rc<Option<Box<dyn Any>>> {
        self.viewmodel.clone()
    }

    fn get_view(self: &Self) -> Rc<dyn IView> {
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