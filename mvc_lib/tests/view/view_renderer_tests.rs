use std::any::Any;
use std::rc::Rc;

use mvc_lib::contexts::iresponse_context::MockIResponseContext;
use mvc_lib::contexts::irequest_context::MockIRequestContext;
use mvc_lib::model_binder::iviewmodel::MockIViewModel;
use mvc_lib::services::service_collection::{ServiceCollection, IServiceCollection};
use mvc_lib::services::service_descriptor::ServiceDescriptor;
use mvc_lib::services::service_scope::ServiceScope;
use mvc_lib::view::iview::IView;
use mvc_lib::view::view_renderer::{ViewRenderer, IViewRenderer};


struct MockView {
    content: String
}

impl MockView {
    fn new() -> Self {
        Self {
            content: "Hello, world!".to_string()
        }
    }

    fn new_service(_services: &dyn IServiceCollection) -> Vec<Box<dyn Any>> {
        vec![Box::new(Rc::new(Self::new()) as Rc<dyn IView>)]
    }
}

impl IView for MockView {
    fn get_path(self: &Self) -> String {
        "tests/view/view_renderer_tests.rs".to_string()
    }

    fn get_raw(self: &Self) -> String {
        self.content.clone()
    }

    fn get_model_type_name(self: &Self) -> Option<String> {
        None
    }

    fn render(self: &Self, _ctx: &dyn mvc_lib::contexts::view_context::IViewContext, _services: &dyn IServiceCollection) -> Result<mvc_lib::view::rusthtml::html_string::HtmlString, mvc_lib::view::rusthtml::rusthtml_error::RustHtmlError> {
        Ok(mvc_lib::view::rusthtml::html_string::HtmlString::new_from_html(self.content.clone()))
    }
}


#[test]
fn view_renderer_render() {
    // need to register view at view_path
    let view_renderer = ViewRenderer::new();

    // todo / fixme: this does not actually exist so we need to mock the view provider to provide a mock view.
    let view_path = "tests/view/view_renderer_tests.rs";
    let mock_vm = MockIViewModel::new();
    let view_model = Rc::new(mock_vm.object());
    let mut response_context = MockIResponseContext::new();
    response_context
        .expect_get_string()
        .returning(|x| {
            match x.as_str() {
                _ => None
            }
        });

    let mut request_context = MockIRequestContext::new();
    request_context
        .expect_get_string()
        .returning(|x| {
            match x.as_str() {
                "Content-Type" => "text/html".to_string(),
                _ => "".to_string()
            }
        });

    let mut services = ServiceCollection::new_root();
    services.add(ServiceDescriptor::new_from::<dyn IView, MockView>(MockView::new_service, ServiceScope::Request));
    ViewRenderer::add_to_services(&mut services);

    let result = view_renderer.render_with_layout_if_specified(&view_path.to_string(), Some(view_model), &request_context, &services).unwrap();
    assert_eq!(result.content, "Hello, world!");
}