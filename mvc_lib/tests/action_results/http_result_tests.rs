use mvc_lib::contexts::ihttpconnection_context::MockIHttpConnectionContext;
use mvc_lib::contexts::request_context::RequestContext;
use mvc_lib::contexts::response_context::ResponseContext;
use mvc_lib::action_results::iaction_result::IActionResult;
use mvc_lib::action_results::http_result::HttpRedirectResult;
use mvc_lib::services::service_collection::ServiceCollection;



#[test]
fn redirect_result_sets_location_header() {
    let result = HttpRedirectResult::new("http://www.google.com".to_string());
    assert_eq!(result.get_statuscode(), http::StatusCode::TEMPORARY_REDIRECT);

    // need to mock the response context and request context
    let mut connection_context = MockIHttpConnectionContext::new();
    connection_context.expect_add_header_string().withf(|name, value| {
        name == "Location" && value == "http://www.google.com"
    }).times(1).return_const(());

    let request_context = RequestContext::default(&connection_context);
    let response_context = ResponseContext::new(&request_context);
    let services = ServiceCollection::new_root();
    result.configure_response(&response_context, &request_context, &services).unwrap();
}