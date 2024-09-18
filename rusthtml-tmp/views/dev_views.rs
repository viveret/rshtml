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
RouteValuesBuilder; pub struct view_dev_views
{
    model_type_name : & 'static str, ViewPath : & 'static str, raw : & 'static
    str, when_compiled : DateTime < Utc > ,
} impl view_dev_views
{
    pub fn new() -> Self
    {
        Self
        {
            model_type_name :
            "crate::view_models::dev::views::ViewsViewModel", ViewPath : file!
            (), raw : "", when_compiled : DateTime ::
            parse_from_rfc2822("Mon, 16 Sep 2024 02:52:24 +0000").expect("could not parse when compiled").into(),
        }
    } pub fn new_service() -> Box < dyn Any >
    {
        Box :: new(Rc :: new(Self :: new()) as Rc < dyn IView >) as Box < dyn
        Any >
    }
} impl IView for view_dev_views
{
    fn get_path(self : & Self) -> String { self.ViewPath.to_string() } fn
    get_raw(self : & Self) -> String { self.raw.to_string() } fn
    get_model_type_name(self : & Self) -> Option < String >
    { Some(self.model_type_name.to_string()) } fn
    render(self : & Self, view_context : & dyn IViewContext, services : & dyn
    IServiceCollection) -> Result < HtmlString, RustHtmlError >
    {
        let vm = view_context.get_viewmodel(); let model = match vm
        {
            Some(m) =>
            {
                m.as_ref().as_any().downcast_ref :: < crate :: view_models ::
                dev :: views :: ViewsViewModel >
                ().expect(format!
                ("could not downcast model from Rc<dyn IModel>({:?}) to {:?}",
                m.get_type_info(), TypeInfo :: of :: < crate :: view_models ::
                dev :: views :: ViewsViewModel > ()).as_str()).clone()
            }, None => panic! ("No model set")
        }; let html = HtmlHelpers :: < crate :: view_models :: dev :: views ::
        ViewsViewModel > :: new(view_context, services); let render =
        RenderHelpers :: new(view_context, services); let url = UrlHelpers ::
        new(view_context, services); let html_output = HtmlBuffer :: new();
        match
        view_context.get_view_renderer().render_with_layout_if_specified(&
        "dev/_view_start.rs".to_string(), view_context.get_viewmodel(),
        view_context.get_request_context(), services)
        {
            Ok(html) => { html_output.write_html(html); }, Err(err) =>
            {
                html_output.write_html_str(format!
                ("could not render view_start: {}", err).as_str());
            }
        }
        view_context.insert_str("Title", "Compiled Views - Dev".to_string());
        html_output.write_html(HtmlString ::
        from(html.link(url.url_action(false, Some(false), None, Some("index"),
        Some("Dev"), None, None).as_str(), "< Back to dev routes list",
        None))); html_output.write_html_str("<h1");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(view_context.get_str("Title")));
        html_output.write_html_str("</h1>"); html_output.write_html_str("<p");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(format! ("In total there are {} views:", model.views.len())));
        html_output.write_html_str("</p>"); html_output.write_html_str("<ul");
        html_output.write_html_str(">"); for compiled_view in
        model.views.iter()
        {
            let href =
            url.url_action(false, Some(false), None, Some("view_details"),
            Some("Dev"), None,
            Some(&RouteValuesBuilder::build_area(compiled_view.path.as_str())));
            let model_type_name = match &compiled_view.model_type_name
            {
                Some(s) => format!("Requires model type {}", s), None =>
                "No model type required".to_string(),
            }; <li> <a href=@href>@compiled_view.path.as_str()
            <span>@" "</span> @model_type_name</a> </li>
        } html_output.write_html_str("</ul>"); Ok(html_output.collect_html())
    }
}