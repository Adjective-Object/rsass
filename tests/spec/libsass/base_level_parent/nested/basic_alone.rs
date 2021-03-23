//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/nested/basic-alone.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  & {\r\
            \n    foo {\r\
            \n      bar: baz;\r\
            \n    }\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "test foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}