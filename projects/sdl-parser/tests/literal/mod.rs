use super::*;

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let out = render(include_str!(concat!($file_name, ".sdl"))).unwrap();
        assert_eq!(include_str!(concat!($file_name, ".out.sdl")), out)
    }
    };
}

run_test![
    number,
    string,string_escape,
];


const COMMENT: &'static str = r#"
/* 1 /* 2 */ 3 */
/* 1 */ 2 /* 3 */
"#;

#[test]
fn comment() {
    assert_eq!(render(COMMENT).unwrap(), "2")
}

const TEMPLATE_ESCAPED: &'static str = r#"[
    <i>{{ 1 ++ "{{2}}" }}</i>
]"#;

#[test]
fn template_escaped() {
    assert_eq!(render(TEMPLATE_ESCAPED).unwrap(), r#"["\"", "\'", "\\", "\n"]"#)
}
