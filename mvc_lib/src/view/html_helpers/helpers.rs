use super::form_helpers::FormHelpers;

pub struct HtmlHelpers {
    pub form: FormHelpers,
}

impl HtmlHelpers {
    pub fn new() -> Self {
        Self {
            form: FormHelpers::new(),
        }
    }
}