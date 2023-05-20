use super::form_helpers::FormHelpers;

// helpers for HTML views
pub struct HtmlHelpers {
    // form helpers
    pub form: FormHelpers,
}

impl HtmlHelpers {
    pub fn new() -> Self {
        Self {
            form: FormHelpers::new(),
        }
    }
}