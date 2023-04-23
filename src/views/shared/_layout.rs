mvc_macro_lib::rusthtml_view_macro! {
    @name "shared__layout"
    @{
        let untitled = "Untitled".to_string();
        let mut page_title = view_context.get_str("Title");
        if page_title.len() == 0 {
            page_title = untitled;
        }

        let page_action = view_context.get_str("ActionName");
        let page_controller = view_context.get_str("ControllerName");
        let page_area = view_context.get_str("AreaName");

    }
    @functions {
        pub fn is_same_action(
            a_action: &'static str, a_controller: &'static str, a_area: &'static str,
            b_action: &String, b_controller: &String, b_area: &String) -> bool {
                (a_action == b_action.as_str() || a_action == "*" || b_action == "*") && 
                (a_controller == b_controller.as_str() || a_controller == "*" || b_controller == "*") && 
                (a_area == b_area.as_str() || a_area == "*" || b_area == "*")
            }
        pub fn is_same_action_is_selected(
            a_action: &'static str, a_controller: &'static str, a_area: &'static str,
            b_action: &String, b_controller: &String, b_area: &String) -> &'static str {
                if is_same_action(a_action, a_controller, a_area, b_action, b_controller, b_area) {
                    "is-selected"
                } else {
                    ""
                }
            }
    }
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>@format!("{} - WebApplication1", page_title)</title>

    <environment include="Development">
        <link rel="stylesheet" href="/stacks.css" />
        <link rel="stylesheet" href="/css/site.css" />
    </environment>
    <environment exclude="Development">
        <link rel="stylesheet" href="/stacks.min.css" />
        <link rel="stylesheet" href="/css/site.min.css" asp-append-version="true" />
    </environment>
</head>
<body class="theme-system">
    <header class="s-topbar stacks-topbar ps-fixed h64 js-stacks-topbar print:d-none">
        <div class="s-topbar--container px8">
            <a href="#" class="s-topbar--menu-btn d-none md:d-flex js-hamburger-btn"><span></span></a>
            <a class="s-topbar--logo" href="/">
                <span class="v-visible-sr">Site home</span>
                @htmlfile "src/views/shared/_icon_svg.html"

                <environment include="Development"><i>Beta</i></environment>
            </a>

            <ul class="s-navigation ml8 fw-nowrap sm:d-none">
                @let home_class = format!("s-navigation--item {}", is_same_action_is_selected("*", "Home", "", &page_action, &page_controller, &page_area));
                @let learn_class = format!("s-navigation--item {}", is_same_action_is_selected("*", "Learn", "", &page_action, &page_controller, &page_area));
                <li><a class=@home_class href="/">Home</a></li>
                <li><a class=@learn_class href="/learn">Learn</a></li>
                <li><a class="s-navigation--item" href="https://github.com/viveret/rshtml">GitHub</a></li>
                <environment include="Development">
                    @let is_dev_controller = is_same_action("*", "Dev", "", &page_action, &page_controller, &page_area) || is_same_action("*", "AuthRoles", "", &page_action, &page_controller, &page_area);
                    @let dev_class = format!("s-navigation--item {}", if is_dev_controller { "is-selected" } else { "" });
                    <li><a class=@dev_class href="/dev">@"Dev Tools"</a></li>
                </environment>
            </ul>

            <ol class="s-topbar--content sm:ml0 overflow-hidden"></ol>

            <div class="s-topbar--searchbar w100 wmx3 sm:wmx-initial js-search">
                <div class="s-topbar--searchbar--input-group">
                    <span class="algolia-autocomplete" style="position: relative; display: inline-block; direction: ltr;"><input id="searchbox" type="text" placeholder="Search…" value="" autocomplete="off" class="s-input s-input__search ds-input" spellcheck="false" role="combobox" aria-autocomplete="list" aria-expanded="false" aria-label="search input" aria-owns="algolia-autocomplete-listbox-0" style="position: relative; vertical-align: top;" dir="auto"><pre aria-hidden="true" style="position: absolute; visibility: hidden; white-space: pre; font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI Adjusted&quot;, &quot;Segoe UI&quot;, &quot;Liberation Sans&quot;, sans-serif; font-size: 13px; font-style: normal; font-variant: normal; font-weight: 400; word-spacing: 0px; letter-spacing: normal; text-indent: 0px; text-rendering: optimizelegibility; text-transform: none;"></pre><span class="ds-dropdown-menu" style="position: absolute; top: 100%; z-index: 100; display: none; left: 0px; right: auto;" role="listbox" id="algolia-autocomplete-listbox-0"><div class="ds-dataset-1"></div></span></span>
                    <svg aria-hidden="true" class="svg-icon iconSearch s-input-icon s-input-icon__search" width="18" height="18" viewBox="0 0 18 18"><path d="m18 16.5-5.14-5.18h-.35a7 7 0 1 0-1.19 1.19v.35L16.5 18l1.5-1.5ZM12 7A5 5 0 1 1 2 7a5 5 0 0 1 10 0Z"></path></svg>
                </div>
            </div>
        </div>
    </header>

    <div>
        <partial name="_CookieConsentPartial" />
    </div>

    <div class="container body-content ps-relative py24 t64 mx-auto w100 wmx12">
        @render_body()
        <footer class="pt32">
            <hr />

            @let current_year = chrono::prelude::Utc::now().format("%Y");
            <p>&copy; @format!("{} - Example Rust Html Web Application", current_year)</p>

            @let compile_timestamp = format!("Page compiled at {}", self.when_compiled.format("%Y-%m-%d   %H:%M:%S"));
            @let view_timestamp = format!("Page viewed at {}", chrono::prelude::Utc::now().format("%Y-%m-%d   %H:%M:%S"));
            <p>@format!("{} — {}", compile_timestamp, view_timestamp)</p>
            <p>@format!("Layout path: {}, action: {}, controller: {}, area: {}", self.ViewPath, page_action, page_controller, page_area)</p>
        </footer>
    </div>

    <environment include="Development">
        <script src="/js/site.js" asp-append-version="true"></script>
    </environment>
    <environment exclude="Development">
        <script src="/js/site.min.js" asp-append-version="true"></script>
    </environment>

    @render_section_optional("Scripts")
</body>
</html>
}