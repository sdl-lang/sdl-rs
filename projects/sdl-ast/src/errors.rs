use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RuntimeError {
    #[error("FileNotFound")]
    FileNotFound(String),
    #[error("InvalidOperation")]
    InvalidOperation {
        info: String,
        position: String,
    },
    #[error("{position}\nIfLostError: The if statement does not cover all cases\n{info}")]
    IfLost {
        info: String,
        position: String,
    },
    #[error("FormatError: {0}")]
    FormatError(#[from] std::fmt::Error),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl RuntimeError {
    pub fn invalid_operation(msg: &str, p: String) -> RuntimeError {
        Self::InvalidOperation {
            info: String::from(msg),
            position: p
        }
    }
    pub fn if_lost(msg: &str, p: String) -> RuntimeError {
        Self::IfLost {
            info: String::from(msg),
            position: p
        }
    }
}