use std::rc::Rc;

use http::HeaderMap;
use mvc_lib::contexts::ihttpconnection_context::MockIHttpConnectionContext;
use mvc_lib::contexts::fromstring_connection_context::FromStringConnectionContext;
use mvc_lib::contexts::httpconnection_context::HttpConnectionContext;
use mvc_lib::contexts::irequest_context::IRequestContext;
use mvc_lib::contexts::request_context::RequestContext;



#[test]
fn request_context_default_works() {
    let connection_context = MockIHttpConnectionContext::new();
    RequestContext::default(&connection_context);
}


#[test]
fn request_context_new_works_for_any_http_version() {
    let create_for_http_version = |v| {
        let connection_context = MockIHttpConnectionContext::new();
        RequestContext::new(
            &connection_context,
            v,
            None,
            None,
            Some(http::Method::GET),
            Box::new(String::new()),
            0,
            Box::new(String::new()),
            Box::new(String::new()),
            HeaderMap::new(),
        );
    };

    create_for_http_version(http::version::Version::HTTP_09);
    create_for_http_version(http::version::Version::HTTP_10);
    create_for_http_version(http::version::Version::HTTP_11);
    create_for_http_version(http::version::Version::HTTP_2);
    create_for_http_version(http::version::Version::HTTP_3);
}


#[test]
fn request_context_new_works_for_valid_http_methods() {
    let create_for_http_method = |v| {
        let connection_context = MockIHttpConnectionContext::new();
        RequestContext::new(
            &connection_context,
            http::version::Version::HTTP_11,
            None,
            None,
            Some(v),
            Box::new(String::new()),
            0,
            Box::new(String::new()),
            Box::new(String::new()),
            HeaderMap::new(),
        );
    };

    create_for_http_method(http::Method::GET);
    create_for_http_method(http::Method::HEAD);
    create_for_http_method(http::Method::POST);
    create_for_http_method(http::Method::PUT);
    create_for_http_method(http::Method::PATCH);
}


#[test]
fn request_context_new_works_for_valid_http_methods_str() {
    let create_for_http_method = |v: &str| {
        let connection_context = MockIHttpConnectionContext::new();
        RequestContext::new(
            &connection_context,
            http::version::Version::HTTP_11,
            None,
            Some(Box::new(v.to_string())),
            None,
            Box::new(String::new()),
            0,
            Box::new(String::new()),
            Box::new(String::new()),
            HeaderMap::new(),
        );
    };

    let methods = vec![
        "GET",
        "HEAD",
        "POST",
        "PUT",
        "PATCH",
    ];
    for method in methods {
        create_for_http_method(method);
        create_for_http_method(method.to_lowercase().as_str());
    }
}


#[test]
fn request_context_new_works_for_http_and_https() {
    let create_for_scheme = |v: &str| {
        let connection_context = MockIHttpConnectionContext::new();
        RequestContext::new(
            &connection_context,
            http::version::Version::HTTP_11,
            Some(Box::new(v.to_string())),
            None,
            Some(http::Method::GET),
            Box::new(String::new()),
            0,
            Box::new(String::new()),
            Box::new(String::new()),
            HeaderMap::new(),
        );
    };

    create_for_scheme("http");
    create_for_scheme("https");
}


#[test]
#[should_panic(expected = "invalid HTTP method")]
fn request_context_new_panic_for_invalid_http_methods_str() {
    let create_for_http_method = |v: &str| {
        let connection_context = MockIHttpConnectionContext::new();
        RequestContext::new(
            &connection_context,
            http::version::Version::HTTP_11,
            None,
            Some(Box::new(v.to_string())),
            None,
            Box::new(String::new()),
            0,
            Box::new(String::new()),
            Box::new(String::new()),
            HeaderMap::new(),
        );
    };

    let methods = vec![
        "GET1",
        "HEAD2",
        "POST ",
        "PUT!",
        "PATCH-",
    ];
    for method in methods {
        create_for_http_method(method);
        create_for_http_method(method.to_lowercase().as_str());
    }
}


#[test]
#[should_panic]
fn request_context_new_panic_for_invalid_scheme() {
    let create_for_scheme = |v: &str| {
        let connection_context = MockIHttpConnectionContext::new();
        RequestContext::new(
            &connection_context,
            http::version::Version::HTTP_11,
            Some(Box::new(v.to_string())),
            None,
            Some(http::Method::GET),
            Box::new(String::new()),
            0,
            Box::new(String::new()),
            Box::new(String::new()),
            HeaderMap::new(),
        );
    };

    create_for_scheme("http1");
    create_for_scheme("-https");
}

#[test]
fn request_context_parse_works() {
    let data = "GET / HTTP/1.1\r\nHost: localhost\r\n";
    let connection_context = Rc::new(FromStringConnectionContext::new(data.to_string(), 1));
    let http_context = HttpConnectionContext::new(connection_context);
    let request = RequestContext::parse(&http_context).unwrap();

    assert_eq!(request.get_http_version(), http::version::Version::HTTP_11);
    assert_eq!(request.get_method(), http::Method::GET);
    assert_eq!(request.get_path(), "/");
}