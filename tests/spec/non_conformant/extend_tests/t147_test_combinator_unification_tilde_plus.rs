//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/147_test_combinator_unification_tilde_plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("a.a + x {a: b}\
             \nb.b ~ y {@extend x}\n"),
        "a.a + x, b.b ~ a.a + y {\
         \n  a: b;\
         \n}\n"
    );
}
