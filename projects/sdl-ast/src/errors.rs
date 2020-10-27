#[derive(Debug)]
pub struct RuntimeError {
    kind: Box<ErrorKind>,
}

#[derive(Debug)]
pub enum ErrorKind {
    FileNotFound(String),
    InvalidOperation(String),
    IfLost(String)
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl RuntimeError {
    pub fn invalid_operation(msg: &str) -> RuntimeError {
        Self { kind: Box::new(ErrorKind::InvalidOperation(msg.to_string())) }
    }
    pub fn if_lost(msg: &str) -> RuntimeError {
        Self { kind: Box::new(ErrorKind::IfLost(msg.to_string())) }
    }
}
