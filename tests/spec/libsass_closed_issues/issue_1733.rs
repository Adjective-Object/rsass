//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1733.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: #ff6600;\
             \n  b: #ff6600\
             \n}\n"),
        "foo {\
         \n  a: #ff6600;\
         \n  b: #ff6600;\
         \n}\n"
    );
}
