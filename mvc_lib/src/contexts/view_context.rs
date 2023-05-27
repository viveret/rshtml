use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::irequest_context::IRequestContext;

use crate::view::rusthtml::html_string::HtmlString;
use crate::view::iview::IView;
use crate::view::view_renderer::IViewRenderer;

// this trait represents a view context which is used to render a view.
// a view context is created for each view that is rendered.
pub trait IViewContext: Send + Sync {
    // create a new view context for a view that is rendered within the current view context.
    // this is used for rendering layouts and partial views.
    // view: the view to render.
    // returns: a new view context.
    fn recurse_into_new_context(self: &Self, view: Rc<dyn IView>) -> Box<dyn IViewContext>;

    // get the view renderer for the view context.
    fn get_view_renderer(self: &Self) -> Rc<dyn IViewRenderer>;
    // get the context data for the view context.
    fn get_ctx_data(self: &Self) -> Rc<RefCell<HashMap<String, Box<dyn Any>>>>;
    // get the view data for the view context.
    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, String>>>;
    // get the view model for the view context.
    fn get_viewmodel(self: &Self) -> Rc<Option<Box<dyn Any>>>;
    // get the view for the view context.
    fn get_view(self: &Self) -> Rc<dyn IView>;
    // get the view for the view context as a reference.
    fn get_view_as_ref(self: &Self) -> &dyn IView;

    // get the controller context for the view context.
    fn get_controller_ctx(self: &Self) -> Rc<ControllerContext>;
    // get the response context for the view context.
    fn get_response_ctx(self: &Self) -> Rc<ResponseContext>;
    // get the request context for the view context.
    fn get_request_context(self: &Self) -> Rc<dyn IRequestContext>;

    // get a string from the view data or the controller context.
    fn get_string(self: &Self, key: String) -> String;
    // get a string from the view data or the controller context.
    fn get_str(self: &Self, key: &str) -> String;
    
    // insert a string into the view data.
    fn insert_string(self: &Self, key: String, value: String) -> String;
    // insert a string into the view data.
    fn insert_str(self: &Self, key: &str, value: String) -> String;

    // clone the view context for a layout view.
    fn clone_for_layout(self: &Self, layout_view: Rc<dyn IView>) -> Box<dyn IViewContext>;
}

// this struct implements IViewContext.
pub struct ViewContext {
    // the view to render.
    view: Rc<dyn IView>,
    // the context data for the view context.
    ctxdata: Rc<RefCell<HashMap<String, Box<dyn Any>>>>,
    // the view data for the view context.
    viewdata: Rc<RefCell<HashMap<String, String>>>,
    // the view model for the view context.
    viewmodel: Rc<Option<Box<dyn Any>>>,
    // the view renderer for the view context.
    view_renderer: Rc<dyn IViewRenderer>,
    // the controller context for the view context.
    controller_ctx: Rc<ControllerContext>,
    // the response context for the view context.
    response_ctx: Rc<ResponseContext>,
    // the request context for the view context.
    request_ctx: Rc<dyn IRequestContext>,
}
unsafe impl Send for ViewContext {}
unsafe impl Sync for ViewContext {}


impl ViewContext {
    // create a new ViewContext struct.
    // view: the view to render.
    // viewmodel: the view model for the view context.
    // view_renderer: the view renderer for the view context.
    // controller_ctx: the controller context for the view context.
    // response_ctx: the response context for the view context.
    // returns: a new ViewContext struct.
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

    fn get_request_context(self: &Self) -> Rc<dyn IRequestContext> {
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