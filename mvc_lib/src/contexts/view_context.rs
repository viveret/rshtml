use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::contexts::controller_context::IControllerContext;
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
    fn get_ctx_data(self: &Self) -> Rc<RefCell<HashMap<String, Box<dyn Any>>>>;
    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, String>>>;
    fn get_viewmodel(self: &Self) -> Rc<Option<Box<dyn Any>>>;
    fn get_view(self: &Self) -> Rc<dyn IView>;
    fn get_view_as_ref(self: &Self) -> &dyn IView;

    fn get_controller_ctx(self: &Self) -> Rc<ControllerContext>;
    fn get_response_ctx(self: &Self) -> Rc<ResponseContext>;
    fn get_request_ctx(self: &Self) -> Rc<RequestContext>;

    fn get_string(self: &Self, key: String) -> String;
    fn get_str(self: &Self, key: &str) -> String;
    
    fn insert_string(self: &Self, key: String, value: String) -> String;
    fn insert_str(self: &Self, key: &str, value: String) -> String;

    fn clone_for_layout(self: &Self, layout_view: Rc<dyn IView>) -> Box<dyn IViewContext>;
}

pub struct ViewContext {
    view: Rc<dyn IView>,
    ctxdata: Rc<RefCell<HashMap<String, Box<dyn Any>>>>,
    viewdata: Rc<RefCell<HashMap<String, String>>>,
    viewmodel: Rc<Option<Box<dyn Any>>>,
    view_renderer: Rc<dyn IViewRenderer>,
    controller_ctx: Rc<ControllerContext>,
    response_ctx: Rc<ResponseContext>,
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
                controller_ctx: Rc<ControllerContext>,
                response_ctx: Rc<ResponseContext>
            ) -> Self {
        Self {
            viewdata: Rc::new(RefCell::new(HashMap::new())),
            ctxdata: Rc::new(RefCell::new(HashMap::new())),
            view: view,
            viewmodel: viewmodel,
            view_renderer: view_renderer,
            controller_ctx: controller_ctx.clone(),
            response_ctx: response_ctx,
            request_ctx: controller_ctx.request_context.clone(),
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

    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, String>>> {
        self.viewdata.clone()
    }

    fn get_ctx_data(self: &Self) -> Rc<RefCell<HashMap<String, Box<dyn Any>>>> {
        self.ctxdata.clone()
    }

    fn get_viewmodel(self: &Self) -> Rc<Option<Box<dyn Any>>> {
        self.viewmodel.clone()
    }

    fn get_view(self: &Self) -> Rc<dyn IView> {
        self.view.clone()
    }

    fn get_view_as_ref(self: &Self) -> &dyn IView {
        self.view.as_ref()
    }

    fn get_controller_ctx(self: &Self) -> Rc<ControllerContext> {
        self.controller_ctx.clone()
    }

    fn get_response_ctx(self: &Self) -> Rc<ResponseContext> {
        self.response_ctx.clone()
    }

    fn get_request_ctx(self: &Self) -> Rc<RequestContext> {
        self.request_ctx.clone()
    }

    fn get_string(self: &Self, key: String) -> String {
        match self.get_view_data().as_ref().borrow().get(&key) {
            Some(s) => s.clone(),
            None => self.controller_ctx.get_string(key),
        }
    }

    fn get_str(self: &Self, key: &str) -> String {
        self.get_string(key.to_string())
    }
    
    fn insert_string(self: &Self, key: String, value: String) -> String {
        self.get_view_data().as_ref().borrow_mut().insert(key, value.clone());
        value
    }

    fn insert_str(self: &Self, key: &str, value: String) -> String {
        self.insert_string(key.to_string(), value)
    }

    fn clone_for_layout(self: &Self, layout_view: Rc<dyn IView>) -> Box<dyn IViewContext> {
        let copy = Self::new(layout_view.clone(), self.viewmodel.clone(), self.view_renderer.clone(), self.controller_ctx.clone(), self.response_ctx.clone());
        copy.viewdata.as_ref().replace(self.viewdata.as_ref().borrow().clone());
        Box::new(copy)
    }
}