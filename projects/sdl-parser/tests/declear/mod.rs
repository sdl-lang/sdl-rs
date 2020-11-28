use super::*;

const SET: &'static str = r#"
x = 1;
let y = 2;
"#;

#[test]
fn set() {
    println!("{}", render(SET).unwrap());
}
