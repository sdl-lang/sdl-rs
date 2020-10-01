use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Symbol {
   pub path: Vec<AST>,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut path = self.path.iter();
        if let Some(head) = path.next() {
            match &head.kind {
                ASTKind::String(s) => write!(f, "{}", s)?,
                _ => unreachable!(),
            }
        }
        for i in path {
            match &i.kind {
                ASTKind::String(s) => write!(f, "::{}", s)?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl From<Vec<AST>> for Symbol {
    fn from(path: Vec<AST>) -> Self {
       Self {
           path,
       }
    }
}

impl Symbol {
    pub fn namespace(&self) {

    }
    pub fn name(&self) -> String {
        match &self.path.iter().last().unwrap().kind {
            ASTKind::String(s) => s.to_owned(),
            _ => unreachable!(),
        }
    }
}