//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_615.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$foo: \"bar\";\
             \n%#{\"foo--#{$foo}\"} {\
             \n  foo: bar;\
             \n}\n\
             \na {\
             \n  @extend %foo--bar;\
             \n}\n"),
        "a {\
         \n  foo: bar;\
         \n}\n"
    );
}
