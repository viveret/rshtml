use std::rc::Rc;

use crate::contexts::view_context::{IViewContext, ViewContext};
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
    fn section<'b, 'c, 'd>(self: &Self, _: &'b str) -> Result<HtmlString, RustHtmlError> {
        Ok(HtmlString::new_from_html("".to_string()))
    }

    fn section_optional<'b, 'c, 'd>(self: &Self, _: &'b str) -> Result<HtmlString, RustHtmlError> {
        Ok(HtmlString::new_from_html("".to_string()))
    }

    fn body<'b>(self: &Self) -> Result<HtmlString, RustHtmlError> {
        let ctxdata_rc = self.view_context.get_ctx_data();
        let ctxdata = ctxdata_rc.as_ref().borrow();
        let viewdata_rc = self.view_context.get_view_data();
        let viewdata = viewdata_rc.as_ref().borrow();

        // print the keys
        // println!("ctxdata keys ({:?}):", ctxdata.len());
        // for (key, value) in ctxdata.iter() {
        //     println!("{}: {:?}", key, value);
        // }
        // println!("viewdata keys ({:?}):", viewdata.len());
        // for (key, value) in viewdata.iter() {
        //     println!("{}: {:?}", key, value);
        // }

        // function to get from either ctxdata or viewdata by key and return as string
        let get_str = |key: &str| -> Option<String> {
            match ctxdata.get(key) {
                Some(val) => {
                    // try getting as iview
                    match val.downcast_ref::<Rc<dyn IView>>() {
                        Some(body_view) => {
                            // need new context for child view
                            let new_ctx = ViewContext::recurse_into_new_context(self.view_context, body_view.clone());
                            match body_view.render(&new_ctx, self.services) {
                                Ok(html) => {
                                    Some(html.content)
                                },
                                Err(RustHtmlError(e)) => Some(e.to_string()),
                            }
                        },
                        None => {
                            // try getting as string
                            match val.downcast_ref::<String>() {
                                Some(val_any) => {
                                    Some(val_any.to_string())
                                },
                                None => {
                                    None
                                }
                            }
                        }
                    }
                },
                None => {
                    match viewdata.get(key) {
                        Some(val) => {
                            Some(val.to_string())
                        },
                        None => {
                            None
                        }
                    }
                }
            }
        };

        let keys_to_try = vec![
            "BodyView",
            "BodyHtml",
            "body_view",
            "body_html",
            "Body",
            "body",
        ];
        for key in keys_to_try {
            match get_str(key) {
                Some(val) => {
                    return Ok(HtmlString::new_from_html(val));
                },
                None => {
                    continue;
                }
            }
        }
        Ok(HtmlString::new_from_html("BodyView and BodyHtml not found for call to render_body()".to_string()))
    }
}