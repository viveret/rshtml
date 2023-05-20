
use std::rc::Rc;

use crate::contexts::view_context::IViewContext;
use crate::services::service_collection::IServiceCollection;
use crate::view::iview::IView;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

// Helper functions for RustHtml views.
pub struct RustHtmlViewMacros {}

impl RustHtmlViewMacros {
    // render a section of the view or return an error if it does not exist.
    // section_name: the name of the section to render.
    // view: the parent view this section is being rendered from.
    // ctx: the context for the parent view.
    // services: the services available from the parent view.
    // returns: the rendered section or an error if it does not exist.
    pub fn render_section<'a, 'b, 'c>(section_name: &'a str, _view: &'b dyn IView, _ctx: &dyn IViewContext, _services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError<'c>> {
        // println!("render_section: {}", section_name);
        Ok(HtmlString::new_from_html("".to_string()))
    }

    // render a section of the view (if it exists).
    // section_name: the name of the section to render.
    // view: the parent view this section is being rendered from.
    // ctx: the context for the parent view.
    // services: the services available from the parent view.
    // returns: the rendered section or an empty string if it does not exist.
    pub fn render_section_optional<'a, 'b, 'c>(section_name: &'a str, _view: &'b dyn IView, _ctx: &dyn IViewContext, _services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError<'c>> {
        // println!("render_section_optional: {}", section_name);
        Ok(HtmlString::new_from_html("".to_string()))
    }

    // render the body of the layout view.
    // view: the layout view being rendered.
    // ctx: the context for the layout view.
    // services: the services available from the layout view.
    // returns: the rendered body of the layout view or an error.
    pub fn render_body<'a>(_view: &dyn IView, ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError<'a>> {
        let ctxdata_rc = ctx.get_ctx_data();
        let ctxdata = ctxdata_rc.as_ref().borrow();
        let body_view_option = ctxdata.get("BodyView");
        match body_view_option {
            Some(body_view_any) => {
                let body_view = body_view_any.downcast_ref::<Rc<dyn IView>>().expect("could not downcast Any to Box<dyn IView>").clone();
                // need new context for child view
                let new_ctx = ctx.recurse_into_new_context(body_view.clone());
                match body_view.render(&*new_ctx, services) {
                    Ok(html) => {
                        ctx.write_html(new_ctx.collect_html());
                        Ok(html)
                    },
                    Err(RustHtmlError(e)) => Err(RustHtmlError::from_string(e.to_string())),
                }
                
            },
            None => {
                let body_html = ctx.get_str("BodyHtml");
                if body_html.len() > 0 {
                    ctx.write_html_str(&body_html);
                    Ok(HtmlString::empty())
                } else {
                    panic!("BodyView and BodyHtml not found for call to render_body()")
                }
            },
        }
    }
}