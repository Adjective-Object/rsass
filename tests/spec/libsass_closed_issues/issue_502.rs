//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_502.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("$a: 1;;\
             \n;;\n"),
        ""
    );
}
