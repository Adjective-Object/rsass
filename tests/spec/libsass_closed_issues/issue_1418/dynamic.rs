//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1418/dynamic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "foo {\
             \n    color: call(missing, $a: b);\
             \n}\n"
        ),
        "DEPRECATION WARNING: Passing a string to call() is deprecated and will be illegal\
         \nin Dart Sass 2.0.0. Use call(get-function(missing)) instead.\n\
         \n  ,\
         \n2 |     color: call(missing, $a: b);\
         \n  |            ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 2:12  root stylesheet\n\
         \nError: Plain CSS functions don\'t support keyword arguments.\
         \n  ,\
         \n2 |     color: call(missing, $a: b);\
         \n  |            ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:12  root stylesheet",
    );
}
