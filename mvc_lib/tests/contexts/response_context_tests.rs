use std::cell::RefCell;
use std::rc::Rc;

use mockall::predicate;
use mvc_lib::action_results::http_result::{OkResult, HttpRedirectResult};
use mvc_lib::contexts::httpconnection_context::HttpConnectionContext;
use mvc_lib::contexts::ihttpconnection_context::MockIHttpConnectionContext;
use mvc_lib::contexts::iresponse_context::IResponseContext;
use mvc_lib::contexts::request_context::RequestContext;
use mvc_lib::contexts::response_context::ResponseContext;
use mvc_lib::contexts::tcp_connection_context::TcpConnectionContext;
use mvc_lib::core::itcp_stream_wrapper::ITcpStreamWrapper;
use mvc_lib::services::service_collection::ServiceCollection;

// mod resp_tests {
#[test]
fn response_context_new_works() {
    let connection_context = MockIHttpConnectionContext::new();
    let request_context = RequestContext::default(&connection_context);
    ResponseContext::new(&request_context);
}

#[test]
fn response_context_add_header_str_works() {
    let mut connection_context = MockIHttpConnectionContext::new();
    connection_context
        .expect_add_header_str()
        .with(predicate::eq("test-key"), predicate::eq("test-value"))
        .return_const(());

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);

    response.add_header_str("test-key", "test-value");
}

#[test]
fn response_context_add_header_string_works() {
    let mut connection_context = MockIHttpConnectionContext::new();
    connection_context
        .expect_add_header_string()
        .with(
            predicate::eq("test-key".to_string()),
            predicate::eq("test-value".to_string()),
        )
        .return_const(());

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);

    response.add_header_string("test-key".to_string(), "test-value".to_string());
}

#[test]
fn response_context_get_has_started_writing_false_works() {
    let mut connection_context = MockIHttpConnectionContext::new();
    connection_context
        .expect_get_has_started_writing()
        .return_const(false);

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);

    assert_eq!(false, response.get_has_started_writing());
}

#[test]
fn response_context_get_has_started_writing_true_works() {
    let mut connection_context = MockIHttpConnectionContext::new();
    connection_context
        .expect_get_has_started_writing()
        .return_const(true);

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);

    assert_eq!(true, response.get_has_started_writing());
}

pub struct TestBufferedTcpStream {
    pub input_buffer: RefCell<Vec<u8>>,
    pub output_buffer: RefCell<Vec<u8>>,

    read_position: RefCell<usize>,
}

impl TestBufferedTcpStream {
    pub fn new() -> Self {
        Self {
            input_buffer: RefCell::new(Vec::new()),
            output_buffer: RefCell::new(Vec::new()),
            read_position: RefCell::new(0),
        }
    }
}

impl ITcpStreamWrapper for TestBufferedTcpStream {
    fn shutdown(&self, _: std::net::Shutdown) -> std::io::Result<()> {
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn read(&self, b: &mut [u8]) -> std::io::Result<usize> {
        let mut read_position = self.read_position.borrow_mut();
        let input_buffer = self.input_buffer.borrow();

        let mut bytes_read = 0;
        for i in 0..b.len() {
            if *read_position >= input_buffer.len() {
                break;
            }

            b[i] = input_buffer[*read_position];
            *read_position += 1;
            bytes_read += 1;
        }

        Ok(bytes_read)
    }

    fn read_line(&self) -> std::io::Result<String> {
        let mut str = String::new();
        let mut read_position = self.read_position.borrow_mut();
        let input_buffer = self.input_buffer.borrow();

        loop {
            if *read_position >= input_buffer.len() {
                break;
            }

            let c = input_buffer[*read_position] as char;
            *read_position += 1;

            if c == '\n' {
                break;
            }

            str.push(c);
        }

        Ok(str)
    }

    fn write(&self, b: &[u8]) -> std::io::Result<usize> {
        let mut output_buffer = self.output_buffer.borrow_mut();
        output_buffer.extend_from_slice(b);
        Ok(b.len())
    }

    fn write_line(&self, b: &String) -> std::io::Result<usize> {
        self.write(b.as_bytes())
    }

    fn remote_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(([127, 0, 0, 1], 80))
    }
}

#[test]
fn response_context_set_action_result_sets_status_code() {
    let s = TestBufferedTcpStream::new();
    let tcp_connection = TcpConnectionContext::new(Rc::new(RefCell::new(s)), 1);
    let connection_context = HttpConnectionContext::new(Rc::new(tcp_connection));

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);

    response.set_action_result(Some(Rc::new(OkResult::default())));

    assert_eq!(false, response.get_has_started_writing());
    assert_eq!(http::StatusCode::OK, response.get_status_code());
}

#[test]
fn response_context_invoke_action_result_ok_works() {
    let s = TestBufferedTcpStream::new();
    let tcp_connection = TcpConnectionContext::new(Rc::new(RefCell::new(s)), 1);
    let connection_context = HttpConnectionContext::new(Rc::new(tcp_connection));

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);
    let services = ServiceCollection::new_root();

    assert_eq!(false, response.get_has_started_writing());
    response.set_action_result(Some(Rc::new(OkResult::default())));
    response
        .invoke_action_result(&request_context, &services)
        .unwrap();

    assert_eq!(true, response.get_has_started_writing());
    assert_eq!(http::StatusCode::OK, response.get_status_code());
}

#[test]
fn response_context_invoke_action_result_redirect_works() {
    let s = TestBufferedTcpStream::new();
    let tcp_connection = TcpConnectionContext::new(Rc::new(RefCell::new(s)), 1);
    let connection_context = HttpConnectionContext::new(Rc::new(tcp_connection));

    let request_context = RequestContext::default(&connection_context);
    let response = ResponseContext::new(&request_context);
    let services = ServiceCollection::new_root();

    let default_redirect = HttpRedirectResult::default();
    let default_redirect_location = default_redirect.redirect_target.clone();
    response.set_action_result(Some(Rc::new(default_redirect)));
    response
        .invoke_action_result(&request_context, &services)
        .unwrap();

    assert_eq!(false, response.get_has_started_writing());
    assert_eq!(http::StatusCode::TEMPORARY_REDIRECT, response.get_status_code());
    assert_eq!(default_redirect_location, response.get_header("Location").unwrap());
}
// }
