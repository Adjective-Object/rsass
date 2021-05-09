//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1550/mixin_embedded.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin foo() {\
             \n  @function foo() {\
             \n    @return \'foo\';\
             \n  }\
             \n}\n"
        ),
        "Error: Mixins may not contain function declarations.\
         \n  ,\
         \n2 |   @function foo() {\
         \n  |   ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
