//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1271.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$character-code: f102;\n\
             \ntest {\n\
             \n  /* Expected: \"\\f102\" */\n\
             \n  /* Sass 3.4 */\
             \n  content: unquote(\"\\\"\\\\#{$character-code}\\\"\");\n\
             \n  /* Sass 3.3 */\
             \n  content: str-slice(\"\\x\", 1, 1) + $character-code;\n\
             \n}"),
        "test {\
         \n  /* Expected: \"\\f102\" */\
         \n  /* Sass 3.4 */\
         \n  content: \"\\f102\";\
         \n  /* Sass 3.3 */\
         \n  content: \"xf102\";\
         \n}\n"
    );
}
