use mvc_lib::view::rusthtml::html_string::HtmlString;


#[test]
fn html_string_from_string() {
    let s = String::from("Hello, world!");
    let hs = HtmlString::from(s);
    assert_eq!(hs.content, "Hello, world!");
}

#[test]
fn html_string_from_str() {
    let s = "Hello, world!";
    let hs = HtmlString::from(s);
    assert_eq!(hs.content, "Hello, world!");
}

#[test]
fn html_string_is_empty_true() {
    let hs = HtmlString::empty();
    assert_eq!(hs.is_empty(), true);
}

#[test]
fn html_string_is_empty_false() {
    let hs = HtmlString::from("Hello, world!");
    assert_eq!(hs.is_empty(), false);
}

#[test]
fn html_string_len() {
    let hs = HtmlString::from("Hello, world!");
    assert_eq!(hs.len(), "Hello, world!".len());
    let hs = HtmlString::empty();
    assert_eq!(hs.len(), 0);
}

#[test]
fn html_string_escapes_special_characters() {
    for html in vec![
        "<div></div>",
        "<a href=\"www.google.com\"></a>",
        "&lt;a&gt;",
    ] {
        let hs = HtmlString::from(html);
        assert_eq!(html_escape::encode_text(html), hs.content);
    }
}