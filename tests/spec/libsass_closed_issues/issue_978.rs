//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_978.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  [baz=\"#{&}\"] {\
             \n    foo: bar;\
             \n  }\
             \n}"),
        ".foo [baz=\".foo\"] {\
         \n  foo: bar;\
         \n}\n"
    );
}
