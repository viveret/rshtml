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
RouteValuesBuilder; use mvc_lib :: services :: service_collection ::
ServiceCollectionExtensions; pub struct view_shared__layout
{
    model_type_name : & 'static str, ViewPath : & 'static str, raw : & 'static
    str, when_compiled : DateTime < Utc > ,
} impl view_shared__layout
{
    pub fn new() -> Self
    {
        Self
        {
            model_type_name : "", ViewPath : file! (), raw : "", when_compiled
            : DateTime ::
            parse_from_rfc2822("Tue, 01 Oct 2024 23:59:48 +0000").expect("could not parse when compiled").into(),
        }
    } pub fn new_service() -> Box < dyn Any >
    {
        Box :: new(Rc :: new(Self :: new()) as Rc < dyn IView >) as Box < dyn
        Any >
    }
} impl IView for view_shared__layout
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
        pub fn
        is_same_action(a_action : & 'static str, a_controller : & 'static str,
        a_area : & 'static str, b_action : & String, b_controller : & String,
        b_area : & String) -> bool
        {
            (a_action == b_action.as_str() || a_action == "*" || b_action ==
            "*") &&
            (a_controller == b_controller.as_str() || a_controller == "*" ||
            b_controller == "*") &&
            (a_area == b_area.as_str() || a_area == "*" || b_area == "*")
        } pub fn
        is_same_action_is_selected(a_action : & 'static str, a_controller : &
        'static str, a_area : & 'static str, b_action : & String, b_controller
        : & String, b_area : & String) -> & 'static str
        {
            if
            is_same_action(a_action, a_controller, a_area, b_action,
            b_controller, b_area) { "is-selected" } else { "" }
        } let untitled = "Untitled".to_string(); let mut page_title =
        view_context.get_str("Title"); if page_title.len() == 0
        { page_title = untitled; } let page_action =
        view_context.get_str("ActionName"); let page_controller =
        view_context.get_str("ControllerName"); let page_area =
        view_context.get_str("AreaName");
        html_output.write_html_str("<!DOCTYPE");
        html_output.write_html_str("/>"); html_output.write_html_str("<html");
        html_output.write_html_str(">"); html_output.write_html_str("<head");
        html_output.write_html_str(">"); html_output.write_html_str("<meta");
        html_output.write_html_str(">"); html_output.write_html_str("<meta");
        html_output.write_html_str(">"); html_output.write_html_str("<title");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(format! ("{} - WebApplication1", page_title)));
        html_output.write_html_str("</title>");
        html_output.write_html_str("<environment");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(">");
        html_output.write_html_str("</environment>");
        html_output.write_html_str("<environment");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(">");
        html_output.write_html_str("</environment>");
        html_output.write_html_str("</head>");
        html_output.write_html_str("<body"); html_output.write_html_str(">");
        html_output.write_html_str("<header");
        html_output.write_html_str(">"); html_output.write_html_str("<div");
        html_output.write_html_str(">"); html_output.write_html_str("<a");
        html_output.write_html_str(">"); html_output.write_html_str("<span");
        html_output.write_html_str(">");
        html_output.write_html_str("</span>");
        html_output.write_html_str("</a>"); html_output.write_html_str("<a");
        html_output.write_html_str(">"); html_output.write_html_str("<span");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString :: from("Site home"));
        html_output.write_html_str("</span>");
        html_output.write_html_str("<environment");
        html_output.write_html_str(">"); html_output.write_html_str("<i");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString :: from("Beta"));
        html_output.write_html_str("</i>");
        html_output.write_html_str("</environment>");
        html_output.write_html_str("</a>"); html_output.write_html_str("<ul");
        html_output.write_html_str(">"); let home_class = format!
        ("s-navigation--item {}",
        is_same_action_is_selected("*", "Home", "", & page_action, &
        page_controller, & page_area)); let learn_class = format!
        ("s-navigation--item {}",
        is_same_action_is_selected("*", "Learn", "", & page_action, &
        page_controller, & page_area)); let learn_href =
        url.url_action(false, Some(false), None, Some("index"), Some("Learn"),
        None, None); html_output.write_html_str("<li");
        html_output.write_html_str(">"); html_output.write_html_str("<a");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString :: from("GitHub"));
        html_output.write_html_str("</a>");
        html_output.write_html_str("</li>");
        html_output.write_html_str("<environment");
        html_output.write_html_str(">"); let is_dev_controller =
        is_same_action("*", "Dev", "", & page_action, & page_controller, &
        page_area) ||
        is_same_action("*", "AuthRoles", "", & page_action, & page_controller,
        & page_area); let dev_class = format!
        ("s-navigation--item {}", if is_dev_controller { "is-selected" } else
        { "" }); let dev_href =
        url.url_action(false, Some(false), None, Some("index"), Some("Dev"),
        None, None); html_output.write_html_str("</environment>");
        html_output.write_html_str("</ul>");
        html_output.write_html_str("<ol"); html_output.write_html_str(">");
        html_output.write_html_str("</ol>");
        html_output.write_html_str("<div"); html_output.write_html_str(">");
        html_output.write_html_str("<div"); html_output.write_html_str(">");
        html_output.write_html_str("<span"); html_output.write_html_str(">");
        html_output.write_html_str("<input");
        html_output.write_html_str("/>"); html_output.write_html_str("<pre");
        html_output.write_html_str(">"); html_output.write_html_str("</pre>");
        html_output.write_html_str("<span"); html_output.write_html_str(">");
        html_output.write_html_str("<div"); html_output.write_html_str(">");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</span>");
        html_output.write_html_str("</span>");
        html_output.write_html_str("<svg"); html_output.write_html_str(">");
        html_output.write_html_str("<path"); html_output.write_html_str(">");
        html_output.write_html_str("</path>");
        html_output.write_html_str("</svg>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</header>");
        html_output.write_html_str("<div"); html_output.write_html_str(">");
        html_output.write_html_str("<partial");
        html_output.write_html_str(">"); html_output.write_html_str("</div>");
        html_output.write_html_str("<div"); html_output.write_html_str(">");
        html_output.write_html(HtmlString :: from(render.body()));
        html_output.write_html_str("<footer");
        html_output.write_html_str(">"); html_output.write_html_str("<hr");
        html_output.write_html_str("/>"); let current_year = chrono :: prelude
        :: Utc :: now().format("%Y"); html_output.write_html_str("<p");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString :: from("&copy; "));
        html_output.write_html(HtmlString ::
        from(format!
        ("{} - Example Rust Html Web Application", current_year)));
        html_output.write_html_str("</p>"); let compile_timestamp = format!
        ("Page compiled at {}",
        self.when_compiled.format("%Y-%m-%d   %H:%M:%S")); let view_timestamp
        = format!
        ("Page viewed at {}", chrono :: prelude :: Utc ::
        now().format("%Y-%m-%d   %H:%M:%S"));
        html_output.write_html_str("<p"); html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(format! ("{} — {}", compile_timestamp, view_timestamp)));
        html_output.write_html_str("</p>"); html_output.write_html_str("<p");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(format!
        ("Layout path: {}, action: {}, controller: {}, area: {}",
        self.ViewPath, page_action, page_controller, page_area)));
        html_output.write_html_str("</p>");
        html_output.write_html_str("</footer>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("<environment");
        html_output.write_html_str(">");
        html_output.write_html_str("<script");
        html_output.write_html_str(">");
        html_output.write_html_str("</script>");
        html_output.write_html_str("</environment>");
        html_output.write_html_str("<environment");
        html_output.write_html_str(">");
        html_output.write_html_str("<script");
        html_output.write_html_str(">");
        html_output.write_html_str("</script>");
        html_output.write_html_str("</environment>");
        html_output.write_html(HtmlString ::
        from(render.section_optional("Scripts")));
        html_output.write_html_str("</body>");
        html_output.write_html_str("</html>"); Ok(html_output.collect_html())
    }
}