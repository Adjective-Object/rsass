//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_641.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".#{\"foo\"}--1 { width:100%; }"),
        ".foo--1 {\
         \n  width: 100%;\
         \n}\n"
    );
}
