use crate::view::rusthtml::rusthtml_error::RustHtmlError;



pub struct PanicOrReturnError {}

impl PanicOrReturnError {
    // panic or return an error. if should_panic_or_return_error is true, then panic. otherwise, return an error.
    // should_panic_or_return_error: whether or not to panic or return an error.
    // message: the error message.
    // returns: an error with the message.
    pub fn panic_or_return_error<'a, T>(should_panic_or_return_error: bool, message: String) -> Result<T, RustHtmlError<'a>> {
        if should_panic_or_return_error {
            panic!("{}", message);
        } else {
            return Err(RustHtmlError::from_string(message));
        }
    }
}