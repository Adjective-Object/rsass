//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1945.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: #{\"\\\\\"}#{\"baz\"};\
             \n}"),
        "foo {\
         \n  bar: \\baz;\
         \n}\n"
    );
}
