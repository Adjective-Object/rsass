//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/062_test_element_unification_with_namespaced_element_target.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%-a ns|a.foo {a: b}\
             \na {@extend .foo} -a {@extend %-a}\n"),
        "-a ns|a.foo {\
         \n  a: b;\
         \n}\n"
    );
}
