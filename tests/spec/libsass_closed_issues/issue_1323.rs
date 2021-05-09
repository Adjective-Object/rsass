//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1323.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@import url(foo.css) only screen;\
             \n@import url(foo.css) (min-width:400px);\
             \n@import url(foo.css) (min-width:400px) and (max-width:599px);\n"
        ),
        "@import url(foo.css) only screen;\
         \n@import url(foo.css) (min-width: 400px);\
         \n@import url(foo.css) (min-width: 400px) and (max-width: 599px);\n"
    );
}
