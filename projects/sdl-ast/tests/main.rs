use sdl_ast::{ASTKind, AST};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn sized() {
    println!("{}", std::mem::size_of::<AST>());
    println!("{}", std::mem::size_of::<Box<ASTKind>>());
    println!("{}", std::mem::size_of::<Vec<ASTKind>>());
    println!("{}", std::mem::size_of::<String>());
    assert_eq!(std::mem::size_of::<ASTKind>(), 32);
}
