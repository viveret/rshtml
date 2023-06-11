use std::borrow::Cow;

// https://codereview.stackexchange.com/a/229808
// returns if the string ends with any of the given suffixes.
// s: the string to check.
// suffixes: the suffixes to check.
// returns: true if the string ends with any of the given suffixes, false otherwise.
pub fn string_ends_with_any(s: String, suffixes: &[&str]) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}





pub fn action_name_to_path(
    controller_name: &str,
    action_name: &str
) -> Cow<'static, str> {
    if action_name == "" || action_name == "index" {
        format!("/{}", controller_name.replace("_", "-").to_lowercase()).into()
    } else {
        format!("/{}/{}", controller_name.replace("_", "-").to_lowercase(), action_name.replace("_", "-").to_lowercase()).into()
    }
}