use mvc_lib::{view::view_renderer::{ViewRenderer, IViewRenderer}, services::service_collection::ServiceCollection};

#[test]
fn view_renderer_render() {
    let mut view_renderer = ViewRenderer::new();
    let view_path = "tests/view/view_renderer_tests.rs";
    let view_model = MockIViewModel::new();
    let response_context = MockIResponseContext::new();
    let request_context = MockIRequestContext::new();
    let services = ServiceCollection::new_root();

    let result = view_renderer.render_with_layout_if_specified(view_path, view_model, response_context, request_context, services).unwrap();
    assert_eq!(result.content, "Hello, world!");
}