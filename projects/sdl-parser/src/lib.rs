#![feature(once_cell)]

mod errors;
mod parser;

pub use errors::{Error, ParserResult};
pub use parser::ParserConfig;
