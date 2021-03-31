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
        item_type: String,
        position: Range,
    },
    InvalidIterator {
        item_type: String,
        position: Range,
    },
    IfLost {
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
            SDLErrorKind::InvalidIndex { index, item_type, position } => {
                writeln!(f, "IndexError: Unable to get index {} on type `{}`", index, item_type)?;
                write!(f, "--> {}:{}",position.start.line+ 1,position.start.character + 1)
            }
            SDLErrorKind::InvalidIterator { item_type, position } => {
                writeln!(f, "IteratorError: Type `{}` is not an iterable element", item_type)?;
                write!(f, "--> {}:{}",position.start.line+ 1,position.start.character + 1)
            }
            SDLErrorKind::IfLost { position } => {
                writeln!(f, "IfLostError: If statements are not exhaustive")?;
                write!(f, "--> {}:{}",position.start.line+ 1,position.start.character + 1)
            }
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

    pub fn invalid_operation(msg: impl Into<String>, p: Range) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::InvalidOperation {
                info: msg.into(),
                position: p
            })
        }
    }

    pub fn invalid_iterator(item_type: impl Into<String>, p: Range) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::InvalidIterator {
                item_type: item_type.into(),
                position: p
            })
        }
    }

    pub fn if_lost(p: Range) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::IfLost {
                position: p
            })
        }

    }

    pub fn invalid_index(index: impl Into<String>,
                         item_type: impl Into<String>,
                         position: Range,) -> SDLError {
        Self {
            kind: Box::new(SDLErrorKind::InvalidIndex {
                index: index.into(),
                item_type: item_type.into(),
                position
            })
        }

    }
}