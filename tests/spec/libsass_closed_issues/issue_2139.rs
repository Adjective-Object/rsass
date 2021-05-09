//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2139.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  color: #FFF;\
             \n}\n\
             \n.bar .baz {\
             \n  @extend .foo;\n\
             \n  border: \'1px\';\
             \n}\n\
             \n*:not(.foo) {\
             \n  display: none;\
             \n}\n"),
        ".foo, .bar .baz {\
         \n  color: #FFF;\
         \n}\
         \n.bar .baz {\
         \n  border: \"1px\";\
         \n}\
         \n*:not(.foo) {\
         \n  display: none;\
         \n}\n"
    );
}
