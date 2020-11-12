use super::*;

const FOR_LIST: &'static str = r#"
for i in [1, 2, 3] {
    i + 1
}
"#;

const FOR_STRING: &'static str = r#"
for i in "abc" if x {
    i + 1
}
else {
    y
}
"#;

#[test]
fn for_i_in_list() {
    println!("{}", render(FOR_LIST).unwrap());
}

#[test]
fn for_i_in_string() {
    println!("{}", render(FOR_STRING).unwrap());
}
