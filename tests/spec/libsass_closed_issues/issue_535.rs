//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_535.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$width: 10;\n\
             \n.test {\
             \n  margin-left: - 54 * $width - 1;\
             \n}\n"),
        ".test {\
         \n  margin-left: -541;\
         \n}\n"
    );
}
