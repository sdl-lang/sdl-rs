use lsp_types::Range;
use std::fmt::{Display, self, Formatter};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct SDLError {
    kind: Box<SDLErrorKind>
}

#[derive(Debug, Clone)]
pub enum SDLErrorKind {
    FileNotFound(String),
    InvalidOperation {
        info: String,
        position: Range,
    },
    InvalidIndex {
        index: String,
        lhs_type: String,
        position: Range,
    },
    IfLost {
        info: String,
        position: Range,
    },
    LexerError {
      info: String
    },
    FormatError(std::fmt::Error),
}

pub type Result<T> = std::result::Result<T, SDLError>;

impl Error for SDLError {}

impl Display for SDLError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.kind.as_ref() {
            SDLErrorKind::FileNotFound(_) => {write!(f, "FileNotFound")}
            SDLErrorKind::InvalidOperation { .. } => {write!(f, "InvalidOperation")}
            SDLErrorKind::InvalidIndex { index, lhs_type, position } => {
                writeln!(f, "IndexError: Unable to get index {} on type `{}`", index, lhs_type)?;
                write!(f, "--> {}:{}",position.start.character,position.start.line)
            }
            SDLErrorKind::IfLost { .. } => {write!(f, "IfLostError: The if statement does not cover all cases")}
            SDLErrorKind::FormatError(_) => {write!(f, "FormatError")}
            SDLErrorKind::LexerError { .. } => {write!(f, "LexerError")}
        }
    }
}


impl From<std::fmt::Error> for SDLError {
    fn from(e: std::fmt::Error) -> Self {
        Self {
            kind: Box::new(SDLErrorKind::FormatError(e))
        }
    }
}

impl SDLError {
    pub fn lexer_error(msg: impl Into<String>) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::LexerError {
                info: msg.into(),
            })
        }
    }

    pub fn invalid_operation(msg: &str, p: Range) -> SDLError {

        Self {
            kind: Box::new(SDLErrorKind::InvalidOperation {
                info: String::from(msg),
                position: p
            })
        }
    }
    pub fn if_lost(msg: &str, p: Range) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::IfLost {
                info: String::from(msg),
                position: p
            })
        }

    }
    pub fn invalid_index(index: impl Into<String>,
                         lhs_type: impl Into<String>,
                         position: Range,) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::InvalidIndex {
                index: index.into(),
                lhs_type: lhs_type.into(),
                position
            })
        }

    }
}