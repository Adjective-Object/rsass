//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/05_variable_quoted_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
             \n.result {\
             \n  dquoted: \"#{#{$input}}\";\
             \n  dquoted: \"#{\"[#{$input}]\"}\";\
             \n  dquoted: \"#{\"#{$input}\"}\";\
             \n  dquoted: \"#{\'#{$input}\'}\";\
             \n  dquoted: \"#{\"[\'#{$input}\']\"}\";\
             \n  squoted: \'#{#{$input}}\';\
             \n  squoted: \'#{\"[#{$input}]\"}\';\
             \n  squoted: \'#{\"#{$input}\"}\';\
             \n  squoted: \'#{\'#{$input}\'}\';\
             \n  squoted: \'#{\"[\'#{$input}\']\"}\';\
             \n}\n"
        ),
        ".result {\
         \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  dquoted: \"[\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz]\";\
         \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  dquoted: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\']\";\
         \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  squoted: \"[\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz]\";\
         \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  squoted: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\']\";\
         \n}\n"
    );
}
