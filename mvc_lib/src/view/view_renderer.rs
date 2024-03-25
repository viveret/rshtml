use std::any::Any;
use std::borrow::Cow;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::contexts::irequest_context::IRequestContext;
use crate::core::string_extensions::string_ends_with_any;

use crate::contexts::view_context::IViewContext;
use crate::contexts::view_context::ViewContext;

use crate::core::type_info::TypeInfo;
use crate::model_binder::iviewmodel::IViewModel;
use crate::services::service_collection::ServiceCollection;
use crate::services::service_descriptor::ServiceDescriptor;
use crate::services::service_scope::ServiceScope;
use crate::view::iview::IView;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use crate::services::service_collection::IServiceCollection;
use crate::services::service_collection::ServiceCollectionExtensions;

use super::rusthtml::iviews_path_resolver::IViewsPathResolver;
use super::rusthtml::views_path_resolver::RegularViewsPathResolver;

// this defines the interface for a class that can render views.
// the view result calls this to render the view from the controller.
pub trait IViewRenderer {
    // render the view with the specified path and view model.
    // view_path: the path to the view to render.
    // view_model: the view model to render the view with.
    // response_context: the response context for the view.
    // services: the services available to the view.
    // returns: the rendered view or an error.
    fn render_with_layout_if_specified(
        self: &Self,
        view_path: &String,
        view_model: Option<Rc<dyn IViewModel>>,
        // response_context: &dyn IResponseContext,
        request_context: &dyn IRequestContext,
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



    // resolve the views path string.
    fn resolve_views_path_string(self: &Self, path: &str) -> Option<String>;
    // resolve the data file path string.
    fn resolve_data_file_path_string(self: &Self, path: &str) -> Option<String>;
}

// this is a struct that implements IViewRenderer.
pub struct ViewRenderer {
    // the views available to the view renderer.
    cached_views: RefCell<Option<Vec<Rc<dyn IView>>>>,
    views_path_resolvers: Vec<Rc<dyn IViewsPathResolver>>,
}

impl ViewRenderer  {
    pub fn new() -> Self {
        let project_path = std::env::current_dir().expect("std::env::current_dir()").to_str().expect("to_str").to_string() + "/example_web_app";
        Self {
            cached_views: RefCell::new(None),
            views_path_resolvers: vec![
                Rc::new(RegularViewsPathResolver::new(
                    project_path.clone(),
                )),
            ],
        }
    }

    // create a new instance of the view renderer service for a service collection.
    pub fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(ViewRenderer::new()) as Rc<dyn IViewRenderer>)]
    }

    // add the view renderer service to the service collection.
    pub fn add_to_services(services: &mut ServiceCollection) {
        services.add(ServiceDescriptor::new(TypeInfo::rc_of::<dyn IViewRenderer>(), ViewRenderer::new_service, ServiceScope::Singleton));
    }
}

impl IViewRenderer for ViewRenderer {
    fn render_with_layout_if_specified(
        self: &Self,
        view_path: &String,
        view_model: Option<Rc<dyn IViewModel>>,
        request_context: &dyn IRequestContext,
        services: &dyn IServiceCollection
    ) -> Result<HtmlString, RustHtmlError> {
        let view_renderer_service_instance = ServiceCollectionExtensions::get_required_single::<dyn IViewRenderer>(services);
        let mut body_view_ctx = ViewContext::new(self.get_view(view_path, services), view_model, view_renderer_service_instance.clone(), request_context);
        match body_view_ctx.get_view_as_ref().render(&body_view_ctx, services) {
            Ok(body_html) => {
                // print viewdata keys
                for (key, value) in body_view_ctx.get_view_data().borrow().iter() {
                    println!("{}: {:?}", key, value);
                }
                let layout_view_option = self.get_layout_view_from_context(&mut body_view_ctx, services);
                match layout_view_option {
                    Some(ref layout_view) => {
                        let layout_view_ctx = ViewContext::clone_for_layout(&body_view_ctx, layout_view.clone());
                        layout_view_ctx.insert_str("BodyHtml", body_html.content);

                        match layout_view_ctx.get_view_as_ref().render(&layout_view_ctx, services) {
                            Ok(layout_html) => Ok(layout_html),
                            Err(e) => Err(RustHtmlError::from_string(format!("Could not render layout for view: {}", e))),
                        }
                    },
                    None => {
                        println!("layout_view_option: NOT found");
                        Ok(body_html)
                    },
                }
            },
            Err(e) => Err(RustHtmlError::from_string(format!("Could not render view: {}", e))),
        }
    }

    fn get_layout_view_from_context(self: &Self, view_context: &mut ViewContext, services: &dyn IServiceCollection) -> Option<Rc<dyn IView>> {
        let layout_view_path_option = view_context.get_str("Layout");
        println!("layout_view_path_option: {:?}", layout_view_path_option);
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
        match self.get_views(path, services).first() {
            Some(x) => {
                x.clone()
            },
            None => {
                let available_view_paths = self.get_all_views(services).iter().map(|x| x.get_path()).collect::<Vec<String>>();
                panic!("No views found at '{}' in {:?}. Available views: {:?}", path.as_str(), std::env::current_dir().unwrap(), available_view_paths)
            },
        }
    }

    // this needs to be fixed to be more flexible and like .net core using config and options
    fn resolve_views_path_string(self: &Self, path: &str) -> Option<String> {
        let mut cwd = std::env::current_dir().unwrap();
        let mut path = path.to_string();
        // handle '../' and './' in path
        if path.starts_with("../") {
            loop {
                if path.starts_with("../") && path.len() > 3 {
                    cwd.pop();
                    path = path[3..].to_string();
                } else {
                    break;
                }
            }
        } else if path.starts_with("./") {
            path = path[2..].to_string();
        }

        let path_dir = cwd.to_str().unwrap();
        let x = self.views_path_resolvers
            .iter()
            .flat_map(|x| x.get_view_paths(&path))
            .map(|f| {
                let mut f_absolute = PathBuf::new();
                f_absolute.push(path_dir);
                f_absolute.push(&f);
                f_absolute
            })
            .filter(|x| x.exists() && x.is_file())
            .take(1)
            .next();

        match x {
            Some(x) => {
                return Some(x.to_str().unwrap().to_string());
            },
            None => {
                return None;
            }
        }
    }

    fn resolve_data_file_path_string(self: &Self, path: &str) -> Option<String> {
        match std::fs::File::open(path) {
            Ok(_) => {
                Some(path.to_string())
            },
            Err(_) => {
                None
            },
        }
    }
}