use super::*;

const INDEX_OF_LIST: &'static str = r#"
[
    [1, 2, 3].0,
    [1, 2, 3].1,
    [1, 2, 3].2,
    [1, 2, 3].3,
]
// [].is_empty()
"#;

#[test]
fn index_of_list() {
    assert_eq!(render(INDEX_OF_LIST).unwrap(), "[1, 2, 3, null]")
}

const INDEX_OF_STRING: &'static str = r#"
[
    "abc".0,
    "abc".1,
    "abc".2,
    "abc".3,
]
// [].is_empty()
"#;

#[test]
fn index_of_string() {
    assert_eq!(render(INDEX_OF_STRING).unwrap(), r#"["a", "b", "c", null]"#)
}
