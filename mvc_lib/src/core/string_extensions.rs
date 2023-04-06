// https://codereview.stackexchange.com/a/229808
pub fn string_ends_with_any(s: String, suffixes: &[&str]) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}