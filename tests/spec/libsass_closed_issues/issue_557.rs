//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_557.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("\
             \na {\
             \n  foo: map-get((foo: 1, bar: 2), \"bar\");\
             \n}\n"),
        "a {\
         \n  foo: 2;\
         \n}\n"
    );
}
