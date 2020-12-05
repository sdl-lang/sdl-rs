use super::*;

const COMMENT: &'static str = r#"
/* 1 /* 2 */ 3 */
/* 1 */ 2 /* 3 */
"#;


#[test]
fn comment() {
   assert_eq!(render(COMMENT).unwrap(), "2")
}

const NUMBER: &'static str =r#"[0, .1, 2., 3.0, '4', "5.0"]"#;


#[test]
fn number() {
   assert_eq!(render(NUMBER).unwrap(), "[0, .1, 2., 3.0, '4', \"5.0\"]")
}