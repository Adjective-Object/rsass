//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/01_simple_css.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  color: blue;\
             \n}"),
        "a {\
         \n  color: blue;\
         \n}\n"
    );
}
