//! Tests auto-converted from "sass-spec/spec/css/function_name_identifiers.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  b: url;\
             \n  c: calc;\
             \n  d: element;\
             \n  e: expression;\
             \n  f: progid;\
             \n}\n"),
        "a {\
         \n  b: url;\
         \n  c: calc;\
         \n  d: element;\
         \n  e: expression;\
         \n  f: progid;\
         \n}\n"
    );
}
