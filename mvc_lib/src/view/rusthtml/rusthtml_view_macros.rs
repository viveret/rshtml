
use std::rc::Rc;

use crate::contexts::view_context::IViewContext;
use crate::contexts::controller_context::IControllerContext;
use crate::services::service_collection::IServiceCollection;
use crate::view::iview::IView;
use crate::view::rusthtml::html_string::HtmlString;
use crate::view::rusthtml::rusthtml_error::RustHtmlError;

pub struct RustHtmlViewMacros {
}

impl RustHtmlViewMacros {
    pub fn render_section<'a, 'b, 'c>(section_name: &'a str, view: &'b dyn IView, ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError<'c>> {
        println!("render_section: {}", section_name);
        Ok(HtmlString::new_from_html("".to_string()))
    }

    pub fn render_section_optional<'a, 'b, 'c>(section_name: &'a str, view: &'b dyn IView, ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError<'c>> {
        println!("render_section_optional: {}", section_name);
        Ok(HtmlString::new_from_html("".to_string()))
    }

    pub fn render_body<'a>(view: &dyn IView, ctx: &dyn IViewContext, services: &dyn IServiceCollection) -> Result<HtmlString, RustHtmlError<'a>> {
        let body_view_option = ctx.get_controller_ctx().borrow().get_view_data_value("Body");
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
            None => panic!("Body view not found")
        }
    }
}