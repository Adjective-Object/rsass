//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/three_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod blackness {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, 100%, \"foo\")}\n"
            ),
            "Error: $blackness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, 100%, \"foo\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod hue {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(\"foo\", 100%, 50%)}\n"
            ),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(\"foo\", 100%, 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod whiteness {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.hwb(0, \"foo\", 50%)}\n"
            ),
            "Error: $whiteness: \"foo\" is not a number.\
         \n  ,\
         \n2 | a {b: color.hwb(0, \"foo\", 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
