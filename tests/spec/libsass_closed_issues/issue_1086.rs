//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1086.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("$map: (-1px: 12);"), "");
}
