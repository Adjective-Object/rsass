//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/basic-prefix.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "test {\r\
             \n  pre& {\r\
             \n    foo {\r\
             \n      bar: baz;\r\
             \n    }\r\
             \n  }\r\
             \n}"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |   pre&{\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
