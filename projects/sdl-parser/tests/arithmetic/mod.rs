use super::*;

const INTEGER_BASIC: &'static str = r#"
[
    2 + 2,
    2 - 2,
    2 * 2,
    2 / 2,
]
"#;

#[test]
fn integer_basic() {
    assert_eq!(render(INTEGER_BASIC).unwrap(), "[4, 0, 2, 2]")
}

const DECIMAL_CAST: &'static str = r#"
[
    .0 + 1,
    .0 - 1,
    .0 * 1,
    .0 / 1,
]
"#;

#[test]
fn decimal_cast() {
    assert_eq!(render(DECIMAL_CAST).unwrap(), "[1.0, -1.0, 0.0, 0.0]")
}

const STRING_JOIN: &'static str = r#"
[
    "a" + "b",
]
"#;

#[test]
fn string_join() {
    assert_eq!(render(STRING_JOIN).unwrap(), r#"["ab"]"#)
}