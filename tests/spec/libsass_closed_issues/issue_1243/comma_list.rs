//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/comma-list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(runner().ok("$a: 1, 2\n"), "");
}
