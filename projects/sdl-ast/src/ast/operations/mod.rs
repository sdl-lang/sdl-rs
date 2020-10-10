use super::*;


#[derive(Clone, Eq, PartialEq)]
pub enum Operation {
    Prefix(String),
    Infix(String),
    Suffix(String),
}

impl Operation {
    pub fn prefix(o: &str) -> Self {
        Self::Prefix(o.to_owned())
    }
    pub fn infix(o: &str) -> Self {
        Self::Infix(o.to_owned())
    }
    pub fn suffix(o: &str) -> Self {
        Self::Suffix(o.to_owned())
    }

}



impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operation::Infix(s)|Operation::Prefix(s) | Operation::Suffix(s)=> write!(f,"{}",s),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operation::Prefix(s) | Operation::Suffix(s)=> write!(f,"{}",s),
            Operation::Infix(s) => {
                match s.as_str() {
                    "^" => write!(f,"{}",s),
                    _ => write!(f," {} ",s),
                }
            }
        }

    }
}