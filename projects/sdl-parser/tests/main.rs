use awsl_parser::{ParserConfig, ParserResult};
use sdl_ast::AST;

#[test]
fn ready() {
    println!("it, works!")
}

pub fn parse(input: &str) -> String {
    let mut parser = ParserConfig::default();
    let ast = parser.parse(input).unwrap_or_default();
    format!("{:?}", ast)
}


#[test]
fn new() {
    println!("{}",parse(r#"
    <img/>
    "#))

}