use sdl_ast::AST;
use sdl_parser::ParserConfig;

#[test]
fn ready() {
    println!("it works!")
}

pub fn parse(input: &str) -> AST {
    let mut parser = ParserConfig::default();
    parser.parse(input).unwrap_or_default()
}

#[test]
fn new() {
    println!(
        "{:#?}",
        parse(
            r#"
for a in [1, 2, 3] {
    <p>{a}</p>
}
    "#
        )
    )
}
