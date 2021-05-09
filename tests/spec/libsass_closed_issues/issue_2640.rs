//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue-2640.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".theme1, .theme2 {\
             \n  .something {\
             \n    /* nothing */\
             \n  }\
             \n}\n\
             \n$sel: selector-nest(\'.theme1, .theme2\', \'.something\');\
             \n  \
             \n#{$sel} {\
             \n  /* nothing */\
             \n}\n"),
        ".theme1 .something, .theme2 .something {\
         \n  /* nothing */\
         \n}\
         \n.theme1 .something, .theme2 .something {\
         \n  /* nothing */\
         \n}\n"
    );
}
