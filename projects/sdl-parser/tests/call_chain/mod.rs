use super::*;

const SET: &'static str = r#"
a.1
// [].is_empty()
"#;

#[test]
fn set() {
    println!("{}", render(SET).unwrap());
}
