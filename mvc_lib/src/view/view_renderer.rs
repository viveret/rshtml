use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::contexts::controller_context::IControllerContext;
use crate::contexts::controller_context::ControllerContext;

use crate::view::iview::IView;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

pub trait IViewRenderer {
    // fn render_view(self: &Self, view_ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError>;
    // fn render_page(self: &Self, view_ctx: &dyn IViewContext, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<String, Box<dyn Error>>;
    //fn render_partial(self: &Self, view_path: String, view_model: Option<Rc<dyn Any>>, view_renderer: Rc<dyn IViewRenderer>, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError>;
    
    fn get_layout_view_from_context(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Option<Rc<dyn IView>>;

    fn get_all_views(self: &Self, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>>;
    fn get_views(self: &Self, path: &String, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>>;
    fn get_view(self: &Self, path: &String, services: &dyn IServiceCollection) -> Rc<dyn IView>;
}

pub struct ViewRenderer {
    cached_views: RefCell<Option<Vec<Rc<dyn IView>>>>,
}

impl ViewRenderer  {
    pub fn new() -> Self {
        Self {
            cached_views: RefCell::new(None)
        }
    }

    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(ViewRenderer::new()) as Rc<dyn IViewRenderer>)]
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

    fn get_layout_view_from_context(self: &Self, controller_ctx: Rc<RefCell<ControllerContext>>, services: &dyn IServiceCollection) -> Option<Rc<dyn IView>> {
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

    // fn render_partial(self: &Self, view_path: String, view_model: Option<Rc<dyn Any>>, view_renderer: Rc<dyn IViewRenderer>, controller_ctx: Rc<RefCell<ControllerContext>>, response_ctx: Rc<RefCell<ResponseContext>>, request_ctx: Rc<RequestContext>, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError> {
    //     let view_to_render = self.get_view(&view_path, services);
    //     let view_ctx = ViewContext::new(view_to_render.clone(), view_model, view_renderer, controller_ctx.clone(), response_ctx.clone(), request_ctx.clone());
    //     // self.render_view(&view_ctx, services)
    //     view_ctx.get_view().render(&view_ctx, services)
    // }

    // fn render_view(self: &Self, view_ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError> {
    // }

    fn get_all_views(self: &Self, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>> {
        self.cached_views
            .borrow_mut()
            .get_or_insert_with(|| 
                ServiceCollectionExtensions::get_required_multiple::<dyn IView>(services)
                // services
                //     .get_required(TypeInfo::rc_of::<dyn IView>())
                //     .iter()
                //     .map(|x| x.downcast::<Rc<dyn IView>>().expect("could not downcast Any to Rc<dyn IView>"))
                //     .map(|x| *x)
                //     .collect()
            )
            .clone()
            .iter()
            .map(|x| x.clone())
            .collect()
    }

    fn get_views(self: &Self, path: &String, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>> {
        self.cached_views
            .borrow_mut()
            .get_or_insert_with(|| 
                ServiceCollectionExtensions::get_required_multiple::<dyn IView>(services)
            )
            .clone()
            .iter()
            .filter(|x| x.get_path().ends_with(path) || x.get_path().ends_with(format!("{}.rs", path).as_str()))
            .map(|x| x.clone())
            .collect()
    }

    fn get_view(self: &Self, path: &String, services: &dyn IServiceCollection) -> Rc<dyn IView> {
        self.get_views(path, services)
            .first()
            .expect(&format!("No views found at '{}'", path.as_str()).to_string()).clone()
    }
}