#[derive(Debug)]
pub struct RuntimeError {
    kind: Box<ErrorKind>,
}

#[derive(Debug)]
pub enum ErrorKind {
    FileNotFound(String),
    InvalidOperation(String),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl RuntimeError {
    pub fn invalid_operation(msg: &str) -> RuntimeError {
        Self { kind: Box::new(ErrorKind::InvalidOperation(msg.to_string())) }
    }
}
