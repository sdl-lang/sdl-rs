use sdl_ast::RuntimeError;
use sdl_pest::Rule;

#[derive(Debug)]
pub enum Error {
    LexerError(String),
    FileNotFound(String),
    IOError(String),
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

type LexerError = sdl_pest::Error<Rule>;
type IOError = std::io::Error;

impl From<LexerError> for Error {
    fn from(e: LexerError) -> Self {
        Error::LexerError(format!("{}", e))
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::IOError(format!("{:?}", e))
    }
}

impl From<RuntimeError> for Error {
    fn from(e: RuntimeError) -> Self {
        Error::RuntimeError(format!("{:?}", e))
    }
}
