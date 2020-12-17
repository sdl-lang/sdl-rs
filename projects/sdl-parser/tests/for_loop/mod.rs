use super::*;

const FOR_LIST: &'static str = r#"
for i in [1, 2, 3] {
    i + 1
}
"#;

#[test]
fn for_i_in_list() {
    println!("{}", render(FOR_LIST).unwrap());
}

const FOR_STRING: &'static str = r#"
for i in "abc" {
    i + "x"
}
"#;

#[test]
fn for_i_in_string() {
    println!("{}", render(FOR_STRING).unwrap());
}

const FOR_IF_GUARD: &'static str = r#"
for i in "abc" if x {
    i + 1
}
"#;

#[test]
fn for_if_guard() {
    println!("{}", render(FOR_IF_GUARD).unwrap());
}

const FOR_ELSE_GUARD: &'static str = r#"
for i in [ ] {
    false
}
else {
    true
}
"#;

#[test]
fn for_else_guard() {
    assert_eq!(render(FOR_ELSE_GUARD).unwrap(),"true")
}

const FOR_IF_ELSE_GUARD: &'static str = r#"
for i in [1, 2, 3] if x > 5 {
    false
}
else {
    true
}
"#;

#[test]
fn for_if_else_guard() {
    assert_eq!(render(FOR_IF_ELSE_GUARD).unwrap(),"true")
}
