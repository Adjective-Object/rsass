//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2123/test-02.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@font-face { font-family:\'foo\';src:url(//foo.bar) format(\'woff2\'); }"
        ),
        "@font-face {\
         \n  font-family: \"foo\";\
         \n  src: url(//foo.bar) format(\"woff2\");\
         \n}\n"
    );
}
