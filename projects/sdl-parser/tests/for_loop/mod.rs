use super::*;

const FOR_LIST: &'static str = r#"
for i in [1, 2, 3] {
    i + 1
}
"#;

const FOR_STRING: &'static str = r#"
for i in "abc" {
    i + 1
}
"#;

#[test]
fn new() {
    let out = parse(FOR_LIST);
    // println!("{:#?}", out);
    let mut ctx = SDLContext::default();
    let out = ctx.evaluate(&out).unwrap();
    // println!("{:?}", out);
    let out = ctx.render(&out).unwrap();
    println!("{}", out);
}

#[test]
fn new2() {
    let out = parse(FOR_STRING);
    // println!("{:#?}", out);
    let mut ctx = SDLContext::default();
    let out = ctx.evaluate(&out).unwrap();
    // println!("{:?}", out);
    let out = ctx.render(&out).unwrap();
    println!("{}", out);
}
