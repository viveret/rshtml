use as_any :: Downcast; use std :: any :: Any; use std :: borrow :: Cow; use
std :: cell :: RefCell; use std :: collections :: HashMap; use std :: error ::
Error; use std :: rc :: Rc; use std :: io :: Read; use std :: ops :: Deref;
use std :: sync :: { Arc, RwLock }; use chrono :: { DateTime, TimeZone, Utc };
use proc_macro2 :: TokenStream; use core_macro_lib :: { * }; use mvc_lib ::
core :: type_info :: TypeInfo; use mvc_lib :: core :: html_buffer ::
IHtmlBuffer; use mvc_lib :: core :: html_buffer :: HtmlBuffer; use mvc_lib ::
contexts :: controller_context :: IControllerContext; use mvc_lib :: contexts
:: view_context :: IViewContext; use mvc_lib :: model_binder :: imodel ::
IModel; use mvc_lib :: model_binder :: imodel :: AnyIModel; use mvc_lib ::
services :: service_collection :: IServiceCollection; use mvc_lib :: view ::
rusthtml :: helpers :: ihtml_helpers :: IHtmlHelpers; use mvc_lib :: view ::
rusthtml :: helpers :: html_helpers :: HtmlHelpers; use mvc_lib :: view ::
rusthtml :: helpers :: irender_helpers :: IRenderHelpers; use mvc_lib :: view
:: rusthtml :: helpers :: render_helpers :: RenderHelpers; use mvc_lib :: view
:: rusthtml :: html_string :: HtmlString; use mvc_lib :: view :: rusthtml ::
rusthtml_error :: RustHtmlError; use mvc_lib :: view :: iview :: IView; use
mvc_lib :: routing :: iurl_helpers :: IUrlHelpers; use mvc_lib :: routing ::
url_helpers :: UrlHelpers; use mvc_lib :: routing :: route_values_builder ::
RouteValuesBuilder; pub struct view_home_index
{
    model_type_name : & 'static str, ViewPath : & 'static str, raw : & 'static
    str, when_compiled : DateTime < Utc > ,
} impl view_home_index
{
    pub fn new() -> Self
    {
        Self
        {
            model_type_name : "", ViewPath : file! (), raw : "", when_compiled
            : DateTime ::
            parse_from_rfc2822("Mon, 16 Sep 2024 02:52:49 +0000").expect("could not parse when compiled").into(),
        }
    } pub fn new_service() -> Box < dyn Any >
    {
        Box :: new(Rc :: new(Self :: new()) as Rc < dyn IView >) as Box < dyn
        Any >
    }
} impl IView for view_home_index
{
    fn get_path(self : & Self) -> String { self.ViewPath.to_string() } fn
    get_raw(self : & Self) -> String { self.raw.to_string() } fn
    get_model_type_name(self : & Self) -> Option < String >
    { Some(self.model_type_name.to_string()) } fn
    render(self : & Self, view_context : & dyn IViewContext, services : & dyn
    IServiceCollection) -> Result < HtmlString, RustHtmlError >
    {
        let html = HtmlHelpers :: < AnyIModel > ::
        new(view_context, services); let render = RenderHelpers ::
        new(view_context, services); let url = UrlHelpers ::
        new(view_context, services); let html_output = HtmlBuffer :: new();
        match
        view_context.get_view_renderer().render_with_layout_if_specified(&
        "home/view_start.rs".to_string(), view_context.get_viewmodel(),
        view_context.get_request_context(), services)
        {
            Ok(html) => { html_output.write_html(html); }, Err(err) =>
            {
                html_output.write_html_str(format!
                ("could not render view_start: {}", err).as_str());
            }
        }
        view_context.insert_str("Title",
        "Rust HTML (rshtml) Home".to_string());
        html_output.write_html_str("<h1"); html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(view_context.get_str("Title")));
        html_output.write_html_str("</h1>"); Ok(html_output.collect_html())
    }
}