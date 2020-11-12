#![feature(once_cell)]

mod errors;
mod parser;

pub use errors::{Error, Result};
pub use parser::ParserConfig;
