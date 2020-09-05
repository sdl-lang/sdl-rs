
#[derive(Clone,Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}