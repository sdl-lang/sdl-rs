use super::*;

const COMMENT: &'static str = r#"
/* 1 /* 2 */ 3 */
/* 1 */ 2 /* 3 */
"#;

#[test]
fn comment() {
    assert_eq!(render(COMMENT).unwrap(), "2")
}

const NUMBER: &'static str = r#"[0x0, .1, 2., 3.0, '4', "5.0"]"#;

#[test]
fn number() {
    assert_eq!(render(NUMBER).unwrap(), r#"[0, 0.1, 2.0, 3.0, "4", "5.0"]"#)
}

const STRING: &'static str = r#"[
    "",
    '1',
    `2`,
    ´3´,
    "{ 2 + 2 }",
]"#;

#[test]
fn string() {
    assert_eq!(render(STRING).unwrap(), r#"["", "1", "2", "3", "4"]"#)
}

const STRING_ESCAPED: &'static str = r#"[
    "\"",
    '\'',
    `\\`,
    ´\n´,
]"#;

#[test]
fn string_escaped() {
    assert_eq!(render(STRING_ESCAPED).unwrap(), r#"["\"", "\'", "\\", "\n"]"#)
}