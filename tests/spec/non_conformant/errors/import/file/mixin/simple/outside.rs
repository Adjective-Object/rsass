//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/mixin/simple/outside.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("_include.scss", "")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin import-google-fonts() {\r\
             \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\r\
             \n}\r\
             \n$family: unquote(\"Droid+Sans\");\r\
             \n@include import-google-fonts();"
        ),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
}
