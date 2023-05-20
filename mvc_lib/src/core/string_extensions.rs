// https://codereview.stackexchange.com/a/229808
// returns if the string ends with any of the given suffixes.
// s: the string to check.
// suffixes: the suffixes to check.
// returns: true if the string ends with any of the given suffixes, false otherwise.
pub fn string_ends_with_any(s: String, suffixes: &[&str]) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}