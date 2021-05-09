//! Tests auto-converted from "sass-spec/spec/values/maps/map-values.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  foo: map-values((foo: 1, bar: 2));\
             \n  foo: map-values((foo: 1, bar: 2, baz: 2));\
             \n}\n"),
        "div {\
         \n  foo: 1, 2;\
         \n  foo: 1, 2, 2;\
         \n}\n"
    );
}
