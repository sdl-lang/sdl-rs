use super::*;

#[derive(Clone, Eq, PartialEq)]
pub enum Operator {
    Prefix(String),
    Infix(String),
    Suffix(String),
}

impl Operator {
    pub fn prefix(o: &str) -> Self {
        Self::Prefix(o.to_owned())
    }
    pub fn infix(o: &str) -> Self {
        Self::Infix(o.replace(" ", ""))
    }
    pub fn suffix(o: &str) -> Self {
        Self::Suffix(o.to_owned())
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operator::Infix(s) | Operator::Prefix(s) | Operator::Suffix(s) => write!(f, "{}", s),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operator::Prefix(s) | Operator::Suffix(s) => write!(f, "{}", s),
            Operator::Infix(s) => match s.as_str() {
                "^" => write!(f, "{}", s),
                _ => write!(f, " {} ", s),
            },
        }
    }
}
