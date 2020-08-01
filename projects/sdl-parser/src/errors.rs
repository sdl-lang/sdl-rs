use sdl_pest::Rule;

#[derive(Debug)]
pub enum Error {
    LexerError(String),
    FileNotFound(String),
    IOError(String),
}

pub type ParserResult<T> = Result<T, Error>;

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
