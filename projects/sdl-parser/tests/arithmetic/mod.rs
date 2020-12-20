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