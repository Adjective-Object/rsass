//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1434.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n    a: selector-nest(\'.foo\', \'.bar > .baz\');\
             \n    b: selector-nest(\'.foo\', \'.bar ~ .baz\');\
             \n    c: selector-nest(\'.foo\', \'.bar + .baz\');\
             \n    d: selector-nest(\'.foo > .bar\', \'.baz\');\
             \n    e: selector-nest(\'.foo ~ .bar\', \'.baz\');\
             \n    f: selector-nest(\'.foo + .bar\', \'.baz\');\
             \n}\n"),
        ".foo {\
         \n  a: .foo .bar > .baz;\
         \n  b: .foo .bar ~ .baz;\
         \n  c: .foo .bar + .baz;\
         \n  d: .foo > .bar .baz;\
         \n  e: .foo ~ .bar .baz;\
         \n  f: .foo + .bar .baz;\
         \n}\n"
    );
}
