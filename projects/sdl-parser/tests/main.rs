use sdl_ast::{Context, AST};
use sdl_parser::ParserConfig;

#[test]
fn ready() {
    println!("it works!")
}

pub fn parse(input: &str) -> AST {
    let mut parser = ParserConfig::default();
    parser.parse(input).unwrap_or_default()
}

const CODE: &'static str = r#"
if 1 + 1 is 2 {
    <p>eq</p>
}
"#;

const CODE2: &'static str = r#"
for a in [1, 2, 3] {
    <p>{a}</p>
}
"#;

#[test]
fn new() {
    let ast = parse(CODE);
    // println!("{:#?}", ast);
    let mut ctx = Context::default();
    let out = ctx.evaluate(&ast).unwrap();
    println!("{:#?}", out);
}
