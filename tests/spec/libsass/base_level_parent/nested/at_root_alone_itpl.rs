//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/at-root-alone-itpl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("test {\r\
             \n  @at-root {\r\
             \n    #{&} {\r\
             \n      foo {\r\
             \n        bar: baz;\r\
             \n      }\r\
             \n    }\r\
             \n  }\r\
             \n}\r\n"),
        "test foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
