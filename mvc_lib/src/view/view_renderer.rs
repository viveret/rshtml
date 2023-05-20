use std::any::Any;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

use crate::core::string_extensions::string_ends_with_any;

use crate::contexts::controller_context::ControllerContext;
use crate::contexts::view_context::IViewContext;
use crate::contexts::view_context::ViewContext;
use crate::contexts::response_context::ResponseContext;

use crate::view::iview::IView;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

// this defines the interface for a class that can render views.
// the view result calls this to render the view from the controller.
pub trait IViewRenderer {
    // render the view with the specified path and view model.
    // view_path: the path to the view to render.
    // view_model: the view model to render the view with.
    // controller_ctx: the controller context for the view.
    // response_ctx: the response context for the view.
    // services: the services available to the view.
    // returns: the rendered view or an error.
    fn render_with_layout_if_specified(
        self: &Self,
        view_path: &String,
        view_model: Rc<Option<Box<dyn Any>>>,
        controller_ctx: Rc<ControllerContext>,
        response_ctx: Rc<ResponseContext>,
        services: &dyn IServiceCollection
    ) -> Result<HtmlString, RustHtmlError>;

    // get the layout view from the view context.
    // view_ctx: the view context to get the layout view from.
    // services: the services available to the view.
    // returns: the layout view or None if not specified.
    fn get_layout_view_from_context(self: &Self, view_ctx: &mut ViewContext, services: &dyn IServiceCollection) -> Option<Rc<dyn IView>>;

    // get all views available to the view renderer.
    // services: the services available to the view renderer.
    // returns: all views available to the view renderer.
    fn get_all_views(self: &Self, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>>;

    // get all views with the specified path.
    // path: the path to the views to get.
    // services: the services available to the view renderer.
    fn get_views(self: &Self, path: &String, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>>;
    
    // get the view with the specified path.
    // path: the path to the view to get.
    // services: the services available to the view renderer.
    // returns: the view with the specified path.
    fn get_view(self: &Self, path: &String, services: &dyn IServiceCollection) -> Rc<dyn IView>;
}

// this is a struct that implements IViewRenderer.
pub struct ViewRenderer {
    // the views available to the view renderer.
    cached_views: RefCell<Option<Vec<Rc<dyn IView>>>>,
}

impl ViewRenderer  {
    pub fn new() -> Self {
        Self {
            cached_views: RefCell::new(None)
        }
    }

    // create a new instance of the view renderer service for a service collection.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(ViewRenderer::new()) as Rc<dyn IViewRenderer>)]
    }
}

impl IViewRenderer for ViewRenderer {
    fn render_with_layout_if_specified(
        self: &Self,
        view_path: &String,
        view_model: Rc<Option<Box<dyn Any>>>,
        controller_ctx: Rc<ControllerContext>,
        response_ctx: Rc<ResponseContext>,
        services: &dyn IServiceCollection
    ) -> Result<HtmlString, RustHtmlError> {
        let view_renderer_service_instance = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let mut body_view_ctx = ViewContext::new(self.get_view(view_path, services), view_model.clone(), view_renderer_service_instance.clone(), controller_ctx.clone(), response_ctx.clone());
        match body_view_ctx.get_view_as_ref().render(&body_view_ctx, services) {
            Ok(body_html) => {
                let mut combined_body_html_str = String::new();
                combined_body_html_str.push_str(&body_view_ctx.collect_html().content);
                combined_body_html_str.push_str(&body_html.content);
                let combined_body_html = HtmlString::new_from_html(combined_body_html_str);

                let layout_view_option = self.get_layout_view_from_context(&mut body_view_ctx, services);
                match layout_view_option {
                    Some(ref layout_view) => {
                        // println!("layout_view_option: found");
                        let layout_view_ctx = body_view_ctx.clone_for_layout(layout_view.clone());
                        layout_view_ctx.insert_str("BodyHtml", combined_body_html.content);

                        match layout_view_ctx.get_view_as_ref().render(layout_view_ctx.deref(), services) {
                            Ok(layout_html) => {
                                let mut combined_layout_html_str = String::new();
                                combined_layout_html_str.push_str(&layout_view_ctx.collect_html().content);
                                combined_layout_html_str.push_str(&layout_html.content);

                                Ok(HtmlString::new_from_html(combined_layout_html_str))
                            },
                            Err(e) => Err(RustHtmlError(Cow::Owned(format!("Could not render layout for view: {}", e)))),
                        }
                    },
                    None => {
                        println!("layout_view_option: NOT found");
                        Ok(combined_body_html)
                    },
                }
            },
            Err(e) => Err(RustHtmlError(Cow::Owned(format!("Could not render view: {}", e)))),
        }
    }

    fn get_layout_view_from_context(self: &Self, view_context: &mut ViewContext, services: &dyn IServiceCollection) -> Option<Rc<dyn IView>> {
        let layout_view_path_option = view_context.get_str("Layout");
        if layout_view_path_option.len() > 0 {
            Some(self.get_view(&layout_view_path_option, services))
        } else {
            None
        }
    }

    fn get_all_views(self: &Self, services: &dyn IServiceCollection) -> Vec<Rc<dyn IView>> {
        self.cached_views
            .borrow_mut()
            .get_or_insert_with(|| 
                ServiceCollectionExtensions::get_required_multiple::<dyn IView>(services)
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
            .filter(|x| string_ends_with_any(x.get_path(), &[path, format!("{}.rs", path).as_str(), format!("{}.rshtml", path).as_str()]))
            .map(|x| x.clone())
            .collect()
    }

    fn get_view(self: &Self, path: &String, services: &dyn IServiceCollection) -> Rc<dyn IView> {
        self.get_views(path, services)
            .first()
            .expect(&format!("No views found at '{}'", path.as_str()).to_string()).clone()
    }
}