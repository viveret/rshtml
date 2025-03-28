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
            parse_from_rfc2822("Thu, 27 Mar 2025 02:28:13 +0000").expect("could not parse when compiled").into(),
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
} impl IView for view_shared__layout
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
        html_output.write_html_str(" "); html_output.write_html_str("html");
        html_output.write_html_str(">"); html_output.write_html_str("<html");
        html_output.write_html_str(">"); html_output.write_html_str("<head");
        html_output.write_html_str(">"); html_output.write_html_str("<meta");
        html_output.write_html_str(" ");
        html_output.write_html_str("charset");
        html_output.write_html_str("=");
        html_output.write_html_str("\"utf-8\"");
        html_output.write_html_str(">"); html_output.write_html_str("<meta");
        html_output.write_html_str(" ");
        html_output.write_html_str("content");
        html_output.write_html_str("=");
        html_output.write_html_str("\"width=device-width, initial-scale=1.0\"");
        html_output.write_html_str(" "); html_output.write_html_str("name");
        html_output.write_html_str("=");
        html_output.write_html_str("\"viewport\"");
        html_output.write_html_str(">"); html_output.write_html_str("<title");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString ::
        from(format! ("{} - WebApplication1", page_title)));
        html_output.write_html_str("</title>");
        html_output.write_html_str("<link");
        html_output.write_html_str(" "); html_output.write_html_str("rel");
        html_output.write_html_str("=");
        html_output.write_html_str("\"stylesheet\"");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str("\"/stacks.css\"");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(" "); html_output.write_html_str("rel");
        html_output.write_html_str("=");
        html_output.write_html_str("\"stylesheet\"");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str("\"/css/site.css\"");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(" "); html_output.write_html_str("rel");
        html_output.write_html_str("=");
        html_output.write_html_str("\"stylesheet\"");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str("\"/stacks.min.css\"");
        html_output.write_html_str(">"); html_output.write_html_str("<link");
        html_output.write_html_str(" ");
        html_output.write_html_str("asp-append-version");
        html_output.write_html_str("=");
        html_output.write_html_str("\"true\"");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str("\"/css/site.min.css\"");
        html_output.write_html_str(" "); html_output.write_html_str("rel");
        html_output.write_html_str("=");
        html_output.write_html_str("\"stylesheet\"");
        html_output.write_html_str(">");
        html_output.write_html_str("</head>");
        html_output.write_html_str("<body");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"theme-system\"");
        html_output.write_html_str(">");
        html_output.write_html_str("<header");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar stacks-topbar ps-fixed h64 js-stacks-topbar print:d-none\"");
        html_output.write_html_str(">"); html_output.write_html_str("<div");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar--container px8\"");
        html_output.write_html_str(">"); html_output.write_html_str("<a");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("="); html_output.write_html_str("\"#\"");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar--menu-btn d-none md:d-flex js-hamburger-btn\"");
        html_output.write_html_str(">"); html_output.write_html_str("<span");
        html_output.write_html_str(">");
        html_output.write_html_str("</span>");
        html_output.write_html_str("</a>"); html_output.write_html_str("<a");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("="); html_output.write_html_str("\"/\"");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar--logo\"");
        html_output.write_html_str(">"); html_output.write_html_str("<span");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"v-visible-sr\"");
        html_output.write_html_str(">");
        html_output.write_html_str("Site home");
        html_output.write_html_str("</span>");
        html_output.write_html_str("<i"); html_output.write_html_str(">");
        html_output.write_html_str("Beta");
        html_output.write_html_str("</i>");
        html_output.write_html_str("</a>"); html_output.write_html_str("<ul");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-navigation ml8 fw-nowrap sm:d-none\"");
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
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("="); html_output.write_html_str("\"/\"");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str(& home_class);
        html_output.write_html_str(">"); html_output.write_html_str("Home");
        html_output.write_html_str("</a>");
        html_output.write_html_str("</li>");
        html_output.write_html_str("<li"); html_output.write_html_str(">");
        html_output.write_html_str("<a");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str(& learn_href);
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str(& learn_class);
        html_output.write_html_str(">"); html_output.write_html_str("Learn");
        html_output.write_html_str("</a>");
        html_output.write_html_str("</li>");
        html_output.write_html_str("<li"); html_output.write_html_str(">");
        html_output.write_html_str("<a");
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str("\"https://github.com/viveret/rshtml\"");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-navigation--item\"");
        html_output.write_html_str(">"); html_output.write_html_str("GitHub");
        html_output.write_html_str("</a>");
        html_output.write_html_str("</li>"); let is_dev_controller =
        is_same_action("*", "Dev", "", & page_action, & page_controller, &
        page_area) ||
        is_same_action("*", "AuthRoles", "", & page_action, & page_controller,
        & page_area); let dev_class = format!
        ("s-navigation--item {}", if is_dev_controller { "is-selected" } else
        { "" }); let dev_href =
        url.url_action(false, Some(false), None, Some("index"), Some("Dev"),
        None, None); html_output.write_html_str("<li");
        html_output.write_html_str(">"); html_output.write_html_str("<a");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str(& dev_class);
        html_output.write_html_str(" "); html_output.write_html_str("href");
        html_output.write_html_str("=");
        html_output.write_html_str(& dev_href);
        html_output.write_html_str(">");
        html_output.write_html_str("Dev Tools");
        html_output.write_html_str("</a>");
        html_output.write_html_str("</li>");
        html_output.write_html_str("</ul>");
        html_output.write_html_str("<ol");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar--content sm:ml0 overflow-hidden\"");
        html_output.write_html_str(">"); html_output.write_html_str("</ol>");
        html_output.write_html_str("<div");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar--searchbar w100 wmx3 sm:wmx-initial js-search\"");
        html_output.write_html_str(">"); html_output.write_html_str("<div");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-topbar--searchbar--input-group\"");
        html_output.write_html_str(">"); html_output.write_html_str("<span");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"algolia-autocomplete\"");
        html_output.write_html_str(" "); html_output.write_html_str("style");
        html_output.write_html_str("=");
        html_output.write_html_str("\"position: relative; display: inline-block; direction: ltr;\"");
        html_output.write_html_str(">"); html_output.write_html_str("<input");
        html_output.write_html_str(" "); html_output.write_html_str("type");
        html_output.write_html_str("=");
        html_output.write_html_str("\"text\"");
        html_output.write_html_str(" "); html_output.write_html_str("role");
        html_output.write_html_str("=");
        html_output.write_html_str("\"combobox\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("spellcheck");
        html_output.write_html_str("=");
        html_output.write_html_str("\"false\"");
        html_output.write_html_str(" "); html_output.write_html_str("id");
        html_output.write_html_str("=");
        html_output.write_html_str("\"searchbox\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("placeholder");
        html_output.write_html_str("=");
        html_output.write_html_str("\"Search…\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("autocomplete");
        html_output.write_html_str("=");
        html_output.write_html_str("\"off\"");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"s-input s-input__search ds-input\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("aria-autocomplete");
        html_output.write_html_str("=");
        html_output.write_html_str("\"list\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("aria-owns");
        html_output.write_html_str("=");
        html_output.write_html_str("\"algolia-autocomplete-listbox-0\"");
        html_output.write_html_str(" "); html_output.write_html_str("dir");
        html_output.write_html_str("=");
        html_output.write_html_str("\"auto\"");
        html_output.write_html_str(" "); html_output.write_html_str("value");
        html_output.write_html_str("="); html_output.write_html_str("\"\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("aria-expanded");
        html_output.write_html_str("=");
        html_output.write_html_str("\"false\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("aria-label");
        html_output.write_html_str("=");
        html_output.write_html_str("\"search input\"");
        html_output.write_html_str(" "); html_output.write_html_str("style");
        html_output.write_html_str("=");
        html_output.write_html_str("\"position: relative; vertical-align: top;\"");
        html_output.write_html_str("/>"); html_output.write_html_str("<pre");
        html_output.write_html_str(" ");
        html_output.write_html_str("aria-hidden");
        html_output.write_html_str("=");
        html_output.write_html_str("\"true\"");
        html_output.write_html_str(" "); html_output.write_html_str("style");
        html_output.write_html_str("=");
        html_output.write_html_str("\"position: absolute; visibility: hidden; white-space: pre; font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI Adjusted&quot;, &quot;Segoe UI&quot;, &quot;Liberation Sans&quot;, sans-serif; font-size: 13px; font-style: normal; font-variant: normal; font-weight: 400; word-spacing: 0px; letter-spacing: normal; text-indent: 0px; text-rendering: optimizelegibility; text-transform: none;\"");
        html_output.write_html_str(">"); html_output.write_html_str("</pre>");
        html_output.write_html_str("<span");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"ds-dropdown-menu\"");
        html_output.write_html_str(" "); html_output.write_html_str("id");
        html_output.write_html_str("=");
        html_output.write_html_str("\"algolia-autocomplete-listbox-0\"");
        html_output.write_html_str(" "); html_output.write_html_str("style");
        html_output.write_html_str("=");
        html_output.write_html_str("\"position: absolute; top: 100%; z-index: 100; display: none; left: 0px; right: auto;\"");
        html_output.write_html_str(" "); html_output.write_html_str("role");
        html_output.write_html_str("=");
        html_output.write_html_str("\"listbox\"");
        html_output.write_html_str(">"); html_output.write_html_str("<div");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"ds-dataset-1\"");
        html_output.write_html_str(">"); html_output.write_html_str("</div>");
        html_output.write_html_str("</span>");
        html_output.write_html_str("</span>");
        html_output.write_html_str("<svg");
        html_output.write_html_str(" "); html_output.write_html_str("height");
        html_output.write_html_str("="); html_output.write_html_str("\"18\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("aria-hidden");
        html_output.write_html_str("=");
        html_output.write_html_str("\"true\"");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"svg-icon iconSearch s-input-icon s-input-icon__search\"");
        html_output.write_html_str(" "); html_output.write_html_str("width");
        html_output.write_html_str("="); html_output.write_html_str("\"18\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("viewBox");
        html_output.write_html_str("=");
        html_output.write_html_str("\"0 0 18 18\"");
        html_output.write_html_str(">"); html_output.write_html_str("<path");
        html_output.write_html_str(" "); html_output.write_html_str("d");
        html_output.write_html_str("=");
        html_output.write_html_str("\"m18 16.5-5.14-5.18h-.35a7 7 0 1 0-1.19 1.19v.35L16.5 18l1.5-1.5ZM12 7A5 5 0 1 1 2 7a5 5 0 0 1 10 0Z\"");
        html_output.write_html_str(">");
        html_output.write_html_str("</path>");
        html_output.write_html_str("</svg>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("</header>");
        html_output.write_html_str("<div"); html_output.write_html_str(">");
        html_output.write_html_str("<partial");
        html_output.write_html_str(" "); html_output.write_html_str("name");
        html_output.write_html_str("=");
        html_output.write_html_str("\"_CookieConsentPartial\"");
        html_output.write_html_str("/>");
        html_output.write_html_str("</div>");
        html_output.write_html_str("<div");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"container body-content ps-relative py24 t64 mx-auto w100 wmx12\"");
        html_output.write_html_str(">");
        html_output.write_html(HtmlString :: from(render.body()));
        html_output.write_html_str("<footer");
        html_output.write_html_str(" "); html_output.write_html_str("class");
        html_output.write_html_str("=");
        html_output.write_html_str("\"pt32\"");
        html_output.write_html_str(">"); html_output.write_html_str("<hr");
        html_output.write_html_str("/>"); let current_year = chrono :: prelude
        :: Utc :: now().format("%Y"); html_output.write_html_str("<p");
        html_output.write_html_str(">");
        html_output.write_html_str("&copy; ");
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
        html_output.write_html_str("<script");
        html_output.write_html_str(" "); html_output.write_html_str("src");
        html_output.write_html_str("=");
        html_output.write_html_str("\"/js/site.js\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("asp-append-version");
        html_output.write_html_str("=");
        html_output.write_html_str("\"true\"");
        html_output.write_html_str(">");
        html_output.write_html_str("</script>");
        html_output.write_html_str("<script");
        html_output.write_html_str(" "); html_output.write_html_str("src");
        html_output.write_html_str("=");
        html_output.write_html_str("\"/js/site.min.js\"");
        html_output.write_html_str(" ");
        html_output.write_html_str("asp-append-version");
        html_output.write_html_str("=");
        html_output.write_html_str("\"true\"");
        html_output.write_html_str(">");
        html_output.write_html_str("</script>");
        html_output.write_html(HtmlString ::
        from(render.section_optional("Scripts")));
        html_output.write_html_str("</body>");
        html_output.write_html_str("</html>"); Ok(html_output.collect_html())
    }
}