use rusthtml;
use rusthtml_macro;


#[test]
fn it_works() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/should_fail/*.rs");
}