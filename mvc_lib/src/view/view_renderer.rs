use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::result::Result;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::core::type_info::TypeInfo;

use crate::action_results::view_result::ViewResult;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;
use crate::contexts::response_context::ResponseContext;
use crate::contexts::request_context::RequestContext;
use crate::contexts::view_context::IViewContext;
use crate::contexts::view_context::ViewContext;

use crate::view::rusthtml::html_string::HtmlString;
use crate::view::iview::IView;

use crate::services::service_collection::IServiceCollection;

pub trait IViewRenderer {
    fn render_view(self: &Self, view_ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error>>;
    // fn render_page(self: &Self, view_ctx: &dyn IViewContext, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<String, Box<dyn Error>>;
    fn render_partial(self: &Self, view_path: String, view_model: Option<Rc<Box<dyn Any>>>, view_renderer: Rc<Box<dyn IViewRenderer>>, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error>>;
    
    fn get_layout_view_from_context(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Option<Rc<Box<dyn IView>>>;

    fn get_all_views(self: &Self, services: Arc<RwLock<dyn IServiceCollection>>) -> Vec<Rc<Box<dyn IView>>>;
    fn get_views(self: &Self, path: &String, services: Arc<RwLock<dyn IServiceCollection>>) -> Vec<Rc<Box<dyn IView>>>;
    fn get_view(self: &Self, path: &String, services: Arc<RwLock<dyn IServiceCollection>>) -> Rc<Box<dyn IView>>;
}

pub struct ViewRenderer {
    cached_views: RefCell<Option<Vec<Rc<Box<dyn IView>>>>>,
}

impl ViewRenderer  {
    pub fn new() -> Self {
        Self {
            cached_views: RefCell::new(None)
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Rc<dyn Any>> {
        vec![Rc::new(Box::new(ViewRenderer::new()) as Box<dyn IViewRenderer>)]
    }
}

impl IViewRenderer for ViewRenderer {
    // fn render_page(self: &Self, view_result: &ViewResult, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<String, Box<dyn Error>> {
    //     let layout_view_option = self.get_layout_view_from_context(controller_ctx.clone(), services);
    //     match layout_view_option {
    //         Some(layout_view) => {
    //             let body_view = self.get_view(&view_result.path, services);
    //             controller_ctx.as_ref().borrow_mut().view_data.insert("Body".to_string(), Rc::new(Box::new(body_view.clone())));

    //             let ctx = ViewContext::new(layout_view.clone(), view_result.model, Rc::new(Box::new(self) as Box<dyn IViewRenderer>), controller_ctx.clone(), response_ctx.clone(), request_ctx.clone());
    //             let render_result = layout_view.render(&ctx, services);
    //             Ok(
    //                 render_result
    //                 .expect(&format!("Could not render layout view {}", layout_view.get_path()))
    //                 .content
    //             )
    //         },
    //         None => {
    //             self.render_partial(view_result, controller_ctx, response_ctx, request_ctx, services)
    //         }
    //     }
    // }

    fn get_layout_view_from_context(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Option<Rc<Box<dyn IView>>> {
        let my_view_data = controller_ctx.as_ref().borrow_mut().get_view_data();
        let my_view_data_value = my_view_data.as_ref().borrow_mut();
        let layout_view_path_option = my_view_data_value.get("Layout");
        match layout_view_path_option {
            Some(layout_view_path_any) => {
                let layout_view_path = layout_view_path_any.downcast_ref::<&str>().expect("could not downcast Any to Box<String>");
                Some(self.get_view(&layout_view_path.to_string(), services))
            },
            None => None
        }
    }

    fn render_partial(self: &Self, view_path: String, view_model: Option<Rc<Box<dyn Any>>>, view_renderer: Rc<Box<dyn IViewRenderer>>, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error>> {
        let view_to_render = self.get_view(&view_path, services.clone());
        let ctx = Arc::new(RwLock::new(ViewContext::new(view_to_render.clone(), view_model, view_renderer, controller_ctx.clone(), response_ctx.clone(), request_ctx.clone())));
        self.render_view(ctx, services)
    }

    fn render_view(self: &Self, view_ctx: Arc<RwLock<dyn IViewContext>>, services: Arc<RwLock<dyn IServiceCollection>>) -> Result<Box<HtmlString>, Box<dyn Error>> {
        let view = view_ctx.clone().read().unwrap().get_view();
        println!("Rendering {}", view.get_path());
        view.render(view_ctx.clone(), services.clone())
    }

    fn get_all_views(self: &Self, services: Arc<RwLock<dyn IServiceCollection>>) -> Vec<Rc<Box<dyn IView>>> {
        self.cached_views
            .borrow_mut()
            .get_or_insert_with(|| 
                services
                    .read()
                    .unwrap()
                    .get_required(TypeInfo::rc_of::<dyn IView>())
                    .iter()
                    .map(|x| x.clone().downcast::<Box<dyn IView>>().expect("could not downcast Any to Box<dyn IView>"))
                    .collect()
            )
            .clone()
            .iter()
            .map(|x| x.clone())
            .collect()
    }

    fn get_views(self: &Self, path: &String, services: Arc<RwLock<dyn IServiceCollection>>) -> Vec<Rc<Box<dyn IView>>> {
        self.cached_views
            .borrow_mut()
            .get_or_insert_with(|| 
                services
                    .read()
                    .unwrap()
                    .get_required(TypeInfo::rc_of::<dyn IView>())
                    .iter()
                    .map(|x| x.clone().downcast::<Box<dyn IView>>().expect("could not downcast Any to Box<dyn IView>"))
                    .collect()
            )
            .clone()
            .iter()
            .filter(|x| x.get_path().ends_with(path) || x.get_path().ends_with(format!("{}.rs", path).as_str()))
            .map(|x| x.clone())
            .collect()
    }

    fn get_view(self: &Self, path: &String, services: Arc<RwLock<dyn IServiceCollection>>) -> Rc<Box<dyn IView>> {
        self.get_views(path, services)
            .first()
            .expect(&format!("No views found at '{}'", path.as_str()).to_string()).clone()
    }
}