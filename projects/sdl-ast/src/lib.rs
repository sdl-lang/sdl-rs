#![feature(box_syntax)]

mod ast;
mod compile;
mod traits;
mod utils;

pub use ast::{ASTKind, Template, TemplateKind, AST};
pub use utils::TextRange;
