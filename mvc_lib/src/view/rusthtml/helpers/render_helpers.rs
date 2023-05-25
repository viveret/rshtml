use std::rc::Rc;

use crate::contexts::view_context::IViewContext;
use crate::services::service_collection::IServiceCollection;
use crate::view::iview::IView;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

use super::irender_helpers::IRenderHelpers;

// Helper functions for RustHtml views.
pub struct RenderHelpers<'a> {
    view_context: &'a dyn IViewContext,
    services: &'a dyn IServiceCollection
}

impl <'a> RenderHelpers<'a> {
    pub fn new(view_context: &'a dyn IViewContext, services: &'a dyn IServiceCollection) -> Self {
        Self {
            view_context: view_context,
            services: services
        }
    }
}

impl <'a> IRenderHelpers<'a> for RenderHelpers<'a> {
    fn section<'b, 'c, 'd>(self: &Self, section_name: &'b str) -> Result<HtmlString, RustHtmlError<'d>> {
        Ok(HtmlString::new_from_html("".to_string()))
    }

    fn section_optional<'b, 'c, 'd>(self: &Self, section_name: &'b str) -> Result<HtmlString, RustHtmlError<'d>> {
        Ok(HtmlString::new_from_html("".to_string()))
    }

    fn body<'b>(self: &Self) -> Result<HtmlString, RustHtmlError<'b>> {
        let ctxdata_rc = self.view_context.get_ctx_data();
        let ctxdata = ctxdata_rc.as_ref().borrow();
        let body_view_option = ctxdata.get("BodyView");
        match body_view_option {
            Some(body_view_any) => {
                let body_view = body_view_any.downcast_ref::<Rc<dyn IView>>().expect("could not downcast Any to Box<dyn IView>").clone();
                // need new context for child view
                let new_ctx = self.view_context.recurse_into_new_context(body_view.clone());
                match body_view.render(&*new_ctx, self.services) {
                    Ok(html) => {
                        Ok(html)
                    },
                    Err(RustHtmlError(e)) => Err(RustHtmlError::from_string(e.to_string())),
                }
                
            },
            None => {
                let body_html = self.view_context.get_str("BodyHtml");
                if body_html.len() > 0 {
                    Ok(HtmlString::new_from_html(body_html))
                } else {
                    panic!("BodyView and BodyHtml not found for call to render_body()")
                }
            },
        }
    }
}