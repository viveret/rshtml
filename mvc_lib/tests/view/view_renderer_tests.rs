use std::rc::Rc;

use mvc_lib::contexts::iresponse_context::MockIResponseContext;
use mvc_lib::contexts::irequest_context::MockIRequestContext;
use mvc_lib::model_binder::iviewmodel::MockIViewModel;
use mvc_lib::services::service_collection::ServiceCollection;
use mvc_lib::view::view_renderer::{ViewRenderer, IViewRenderer};


#[test]
fn view_renderer_render() {
    // need to register view at view_path
    let mut view_renderer = ViewRenderer::new();
    let view_path = "tests/view/view_renderer_tests.rs";
    let mock_vm = MockIViewModel::new();
    let view_model = Rc::new(mock_vm.object());
    let response_context = MockIResponseContext::new();
    let request_context = MockIRequestContext::new();
    let mut services = ServiceCollection::new_root();
    ViewRenderer::add_to_services(&mut services);

    let result = view_renderer.render_with_layout_if_specified(&view_path.to_string(), Some(view_model), &response_context, &request_context, &services).unwrap();
    assert_eq!(result.content, "Hello, world!");
}