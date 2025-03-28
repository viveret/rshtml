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
services :: service_scope :: ServiceScope; use mvc_lib :: services ::
service_descriptor :: ServiceDescriptor; use mvc_lib :: services ::
service_collection :: IServiceCollection; use mvc_lib :: services ::
service_collection :: ServiceCollection; use mvc_lib :: view :: rusthtml ::
helpers :: ihtml_helpers :: IHtmlHelpers; use mvc_lib :: view :: rusthtml ::
helpers :: html_helpers :: HtmlHelpers; use mvc_lib :: view :: rusthtml ::
helpers :: irender_helpers :: IRenderHelpers; use mvc_lib :: view :: rusthtml
:: helpers :: render_helpers :: RenderHelpers; use mvc_lib :: view :: rusthtml
:: html_string :: HtmlString; use mvc_lib :: view :: rusthtml ::
rusthtml_error :: RustHtmlError; use mvc_lib :: view :: iview :: IView; use
mvc_lib :: routing :: iurl_helpers :: IUrlHelpers; use mvc_lib :: routing ::
url_helpers :: UrlHelpers; use mvc_lib :: routing :: route_values_builder ::
RouteValuesBuilder; use mvc_lib :: services :: service_collection ::
ServiceCollectionExtensions; pub struct view_home_view_start
{
    model_type_name : & 'static str, ViewPath : & 'static str, raw : & 'static
    str, when_compiled : DateTime < Utc > ,
} impl view_home_view_start
{
    pub fn new() -> Self
    {
        Self
        {
            model_type_name : "", ViewPath : file! (), raw : "", when_compiled
            : DateTime ::
            parse_from_rfc2822("Thu, 27 Mar 2025 02:27:58 +0000").expect("could not parse when compiled").into(),
        }
    } pub fn new_service(_services : & dyn IServiceCollection) -> Vec < Box <
    dyn Any >>
    {
        vec!
        [Box :: new(Rc :: new(Self :: new()) as Rc < dyn IView >) as Box < dyn
        Any >]
    } pub fn add_to_services(services : & mut ServiceCollection)
    {
        services.add(ServiceDescriptor :: new_from :: < dyn IView, Self >
        (Self :: new_service, ServiceScope :: Singleton));
    }
} impl IView for view_home_view_start
{
    fn get_path(& self) -> String { self.ViewPath.to_string() } fn
    get_raw(& self) -> String { self.raw.to_string() } fn
    get_model_type_name(& self) -> Option < String >
    { Some(self.model_type_name.to_string()) } fn
    render(& self, view_context : & dyn IViewContext, services : & dyn
    IServiceCollection) -> Result < HtmlString, RustHtmlError >
    {
        let html = HtmlHelpers :: < AnyIModel > ::
        new(view_context, services); let render = RenderHelpers ::
        new(view_context, services); let url = UrlHelpers ::
        new(view_context, services); let html_output = HtmlBuffer :: new();
        view_context.insert_str("Layout", "shared/_layout.rs".to_string());
        Ok(html_output.collect_html())
    }
}