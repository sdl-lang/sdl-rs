use sdl_ast::{ASTKind, ASTNode};
use std::collections::{BTreeMap, HashMap};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn sized() {
    println!("{}", std::mem::size_of::<ASTNode>());
    println!("{}", std::mem::size_of::<Box<ASTKind>>());
    println!("{}", std::mem::size_of::<Vec<ASTKind>>());
    println!("{}", std::mem::size_of::<String>());
    println!("{}", std::mem::size_of::<HashMap<String, ASTKind>>());
    println!("{}", std::mem::size_of::<BTreeMap<String, ASTKind>>());
    assert_eq!(std::mem::size_of::<ASTKind>(), 32);
}
