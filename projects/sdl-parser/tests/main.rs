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
<img rel src="https://avatars.githubusercontent.com/u/17541209?s=60&amp;v=4" alt="@GalAster" size="20" height="20" width="20" class="avatar-user avatar avatar--small ">
"#;

#[test]
fn new() {
    let out = parse(CODE);
    // println!("{:#?}", ast);
    let mut ctx = Context::default();
    let out = ctx.evaluate(&out).unwrap();
    let out = ctx.render(&out).unwrap();
    println!("{:#?}", out);
}
