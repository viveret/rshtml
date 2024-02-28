use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;

use crate::model_binder::iviewmodel::IViewModel;
use crate::view::iview::IView;
use crate::view::view_renderer::IViewRenderer;

use super::iresponse_context::IResponseContext;

// this trait represents a view context which is used to render a view.
// a view context is created for each view that is rendered.
pub trait IViewContext: Send + Sync {
    // get the view renderer for the view context.
    fn get_view_renderer(self: &Self) -> Rc<dyn IViewRenderer>;
    // get the context data for the view context.
    fn get_ctx_data(self: &Self) -> Rc<RefCell<HashMap<String, Box<dyn Any>>>>;
    // get the view data for the view context.
    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, String>>>;
    // get the view model for the view context.
    fn get_viewmodel(self: &Self) -> Option<Rc<dyn IViewModel>>;
    // get the view for the view context.
    fn get_view(self: &Self) -> Rc<dyn IView>;
    // get the view for the view context as a reference.
    fn get_view_as_ref(self: &Self) -> &dyn IView;

    // if the view defines a view start path, this returns the path.
    fn get_view_start_path(self: &Self) -> Option<String>;

    // get the response context for the view context.
    fn get_response_context(self: &Self) -> &dyn IResponseContext;
    // get the request context for the view context.
    fn get_request_context(self: &Self) -> &dyn IRequestContext;

    // get a string from the view data or the controller context.
    fn get_string(self: &Self, key: String) -> String;
    // get a string from the view data or the controller context.
    fn get_str(self: &Self, key: &str) -> String;
    // try to get a string from the view data or the controller context.
    fn try_get_string(self: &Self, key: String) -> Option<String>;
    // try to get a string from the view data or the controller context.
    fn try_get_str(self: &Self, key: &str) -> Option<String>;
    
    // insert a string into the view data.
    fn insert_string(self: &Self, key: String, value: String) -> String;
    // insert a string into the view data.
    fn insert_str(self: &Self, key: &str, value: String) -> String;

    // open a data (project/module) file from the view context.
    fn open_data_file(self: &Self, path: &str) -> Result<std::fs::File, std::io::Error>;
    // open a view file from the view context.
    fn open_view_file(self: &Self, path: &str) -> Result<std::fs::File, std::io::Error>;

    // resolve a views path string from the view context.
    fn resolve_views_path_string(self: &Self, path: &str) -> Option<String>;

    // resolve a data file path string from the view context.
    fn resolve_data_file_path_string(self: &Self, path: &str) -> Option<String>;

    fn get_markdown_file_nocache(self: &Self, path: &str) -> Option<String>;
}

// this struct implements IViewContext.
pub struct ViewContext<'a> {
    // the view to render.
    view: Rc<dyn IView>,
    // the context data for the view context.
    ctxdata: Rc<RefCell<HashMap<String, Box<dyn Any>>>>,
    // the view data for the view context.
    viewdata: Rc<RefCell<HashMap<String, String>>>,
    // the view model for the view context.
    viewmodel: Option<Rc<dyn IViewModel>>,
    // the view renderer for the view context.
    view_renderer: Rc<dyn IViewRenderer>,
    // the response context for the view context.
    // response_context: &'a dyn IResponseContext,
    // the request context for the view context.
    request_context: &'a dyn IRequestContext,
}
unsafe impl <'a> Send for ViewContext<'a> {}
unsafe impl <'a> Sync for ViewContext<'a> {}


impl <'a> ViewContext<'a> {
    // create a new ViewContext struct.
    // view: the view to render.
    // viewmodel: the view model for the view context.
    // view_renderer: the view renderer for the view context.
    // controller_ctx: the controller context for the view context.
    // response_context: the response context for the view context.
    // returns: a new ViewContext struct.
    pub fn new(
                view: Rc<dyn IView>,
                viewmodel: Option<Rc<dyn IViewModel>>,
                view_renderer: Rc<dyn IViewRenderer>,
                // response_context: &'a dyn IResponseContext,
                request_context: &'a dyn IRequestContext,
            ) -> Self {
        Self {
            viewdata: Rc::new(RefCell::new(HashMap::new())),
            ctxdata: Rc::new(RefCell::new(HashMap::new())),
            view: view,
            viewmodel: viewmodel,
            view_renderer: view_renderer,
            // response_context: response_context,
            request_context: request_context,
        }
    }

    // create a new view context for a view that is rendered within the current view context.
    // this is used for rendering layouts and partial views.
    // view: the view to render.
    // returns: a new view context.
    pub fn recurse_into_new_context(parent_context: &'a dyn IViewContext, view: Rc<dyn IView>) -> ViewContext<'a> {
        Self::new(
            view,
            parent_context.get_viewmodel(),
            parent_context.get_view_renderer(),
            // parent_context.get_response_context(),
            parent_context.get_request_context(),
        )
    }

    // clone the view context for a layout view.
    pub fn clone_for_layout(ctx: &'a dyn IViewContext, layout_view: Rc<dyn IView>) -> ViewContext<'a> {
        let copy = Self::new(layout_view.clone(), ctx.get_viewmodel(), ctx.get_view_renderer(), ctx.get_request_context());
        copy.viewdata.as_ref().replace(ctx.get_view_data().as_ref().borrow().clone());
        copy
    }
}

impl <'a> IViewContext for ViewContext<'a> {
    fn get_view_renderer(self: &Self) -> Rc<dyn IViewRenderer> {
        self.view_renderer.clone()
    }

    fn get_view_data(self: &Self) -> Rc<RefCell<HashMap<String, String>>> {
        self.viewdata.clone()
    }

    fn get_ctx_data(self: &Self) -> Rc<RefCell<HashMap<String, Box<dyn Any>>>> {
        self.ctxdata.clone()
    }

    fn get_viewmodel(self: &Self) -> Option<Rc<dyn IViewModel>> {
        match self.viewmodel {
            Some(ref vm) => Some(vm.clone()),
            None => None,
        }
    }

    fn get_view(self: &Self) -> Rc<dyn IView> {
        self.view.clone()
    }

    fn get_view_as_ref(self: &Self) -> &dyn IView {
        self.view.as_ref()
    }

    fn get_response_context(self: &Self) -> &dyn IResponseContext {
        unimplemented!()
        // self.response_context
    }

    fn get_request_context(self: &Self) -> &dyn IRequestContext {
        self.request_context
    }

    fn get_string(self: &Self, key: String) -> String {
        match self.get_view_data().as_ref().borrow().get(&key) {
            Some(s) if s.len() > 0 => s.clone(),
            Some(_) | None => {
                // match self.response_context.get_string(key.clone()) {
                    // Some(s) => s.clone(),
                    // None => {
                        self.request_context.get_string(key)
                    // },
                // }
            },
        }
    }

    fn try_get_string(self: &Self, key: String) -> Option<String> {
        match self.get_view_data().as_ref().borrow().get(&key) {
            Some(s) => Some(s.clone()),
            None => {
                // match self.response_context.get_string(key.clone()) {
                    // Some(s) => Some(s.clone()),
                    // None => {
                        self.request_context.try_get_string(key)
                    // },
                // }
            },
        }
    }

    fn get_str(self: &Self, key: &str) -> String {
        self.get_string(key.to_string())
    }

    fn try_get_str(self: &Self, key: &str) -> Option<String> {
        self.try_get_string(key.to_string())
    }
    
    fn insert_string(self: &Self, key: String, value: String) -> String {
        self.get_view_data().as_ref().borrow_mut().insert(key, value.clone());
        value
    }

    fn insert_str(self: &Self, key: &str, value: String) -> String {
        self.insert_string(key.to_string(), value)
    }

    fn open_data_file(self: &Self, path: &str) -> Result<std::fs::File, std::io::Error> {
        match self.resolve_data_file_path_string(path) {
            Some(path) => {
                std::fs::File::open(path)
            },
            None => {
                Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not resolve data file path"))
            },
        }
    }

    fn open_view_file(self: &Self, path: &str) -> Result<std::fs::File, std::io::Error> {
        match self.resolve_views_path_string(path) {
            Some(path) => {
                std::fs::File::open(path)
            },
            None => {
                Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not resolve view file path"))
            },
        }
    }

    // this needs to be fixed to be more flexible and like .net core using config and options
    fn resolve_views_path_string(self: &Self, path: &str) -> Option<String> {
        self.view_renderer.resolve_views_path_string(path)
    }

    // this needs to be fixed to be more flexible and like .net core using config and options
    fn resolve_data_file_path_string(self: &Self, path: &str) -> Option<String> {
        self.view_renderer.resolve_data_file_path_string(path)
    }

    fn get_view_start_path(self: &Self) -> Option<String> {
        self.try_get_str("viewstart")
    }

    fn get_markdown_file_nocache(self: &Self, path: &str) -> Option<String> {
        match self.open_data_file(path) {
            Ok(mut f) => {
                let mut buffer = String::new();
                match f.read_to_string(&mut buffer) {
                    Ok(_x) => {
                        Some(comrak::markdown_to_html(&buffer, &comrak::ComrakOptions::default()))
                    },
                    Err(e) => {
                        panic!("Could not read data at {}: {:?}", path, e);
                    }
                }
            },
            Err(e) => {
                panic!("cannot read external markdown file nocache '{}', could not open: {:?}", path, e);
            }
        }
    }
}