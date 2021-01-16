#[derive(Debug, Clone)]
pub struct RuntimeError {
    kind: Box<ErrorKind>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    FileNotFound(String),
    InvalidOperation(String),
    IfLost(String),
    FormatError(),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl From<std::fmt::Error> for RuntimeError {
    fn from(e: std::fmt::Error) -> Self {
        Self::invalid_operation(&format!("{}", e))
    }
}

impl RuntimeError {
    pub fn invalid_operation(msg: &str) -> RuntimeError {
        Self { kind: Box::new(ErrorKind::InvalidOperation(msg.to_string())) }
    }
    pub fn if_lost(msg: &str) -> RuntimeError {
        Self { kind: Box::new(ErrorKind::IfLost(msg.to_string())) }
    }
}
