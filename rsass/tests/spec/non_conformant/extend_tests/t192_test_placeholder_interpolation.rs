//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/192_test_placeholder_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("192_test_placeholder_interpolation")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$foo: foo;\n\
             \n%#{$foo} {color: blue}\
             \n.bar {@extend %foo}\n"),
        ".bar {\
         \n  color: blue;\
         \n}\n"
    );
}
