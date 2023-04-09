Rust HTML (rshtml)
------------------

### Project Summary
This project is focused on imitating Razor / Blazor and the MVC system for C # / .NET. There are 
a lot of similarly or equally named classes, enums, and methods, but 99% of the code was written
without referencing the dotnet core source code.

### Supported Features
- API-only and full web applications
- Dependency injection / type constructor dependency resolution
- Service collections and scoping
- HTTP request / response middleware
- Controllers, actions, action results
- HTML view templating system


### To Do / In Progress Features
- Constraints
- Authorization
- Authentication
- HTTPS
- Automatic route mapping
- Areas
- Logging / error handling


### Main Differences from C# / dot net
- Razor templating text nodes cannot capture spacing at the moment. Non-captured spaces are automatically stripped. To fix this, use <code>@""</code> or <code>@format!()</code>.
- Rust HTML (.rshtml) files are not known by the cargo build system. Changes won't automatically trigger rebuilds.


### Rust HTML Templates
The HTML rendering / templating system is similar to Razor / Blazor in C#. There are 
enough similarities and differences that they should be mentioned here.


#### Similarities
- Use the '@' symbol to escape HTML and use a directive or rust code.
- keywords / directives like '@model', '@functions', '@use' are used to extend the templating language.
- Extendable through dependency injection at many different layers.
- Can define a layout view for views that require an outer template that calls render_body().
- Can render partial views in a template or on own from action in controller, or by using view renderer.
- View paths are search by closest first, with the Shared folder being last.
- Default list of imports required to support the view template.
- Automatically prepend _view_start.rshtml to views in folder unless "" is specified


#### Differences
- The entry point for Rust HTML is in a macro within a rust file, so the rust parser has precedent in certain cases for tokenization and validation.
- Currently no support for hot reloading or runtime compilation of views.
- Some variable names and method names are lower case (instead of LikeThis, it is like_this).
- Can directly import HTML or Rust HTML files with @html and @rshtml, and import markdown / convert to HTML with @mdfile_const or @mdfile_nocache.


#### Not Yet Implemented
- @attributes
- @inject
- @implements
- @section
- @typeparam
- @addTagHelper
- @removeTagHelper
- @tagHelperPrefix
- Tag helpers in general
- Display and editor templates
- Sessions or state management
- Explicit HTML mode by using @:
- Any kind of reflection
- Automatic controller / action route mapping (might be able to use https://stackoverflow.com/a/74573771/11765486)
- Model binding / validation
- General configuration
- Localization
- Background tasks / hosted services
- Entity framework?