//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/19_escapes_single_quoted_lowercase/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".result {\
             \n  output: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\
             \n  output: #{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'};\
             \n  output: \"[#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}]\";\
             \n  output: \"#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\";\
             \n  output: \'#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\';\
             \n  output: \"[\'#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\']\";\
             \n}\n"
        ),
        ".result {\
         \n  output: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\
         \n  output: \"[\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz]\";\
         \n  output: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
         \n  output: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
         \n  output: \"[\'\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\']\";\
         \n}\n"
    );
}
