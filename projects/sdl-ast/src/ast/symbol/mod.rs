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
                ASTKind::EscapedText(s) => write!(f, "{}", s)?,
                _ => unreachable!(),
            }
        }
        for i in path {
            match &i.kind {
                ASTKind::EscapedText(s) => write!(f, "::{}", s)?,
                _ => unreachable!(),
            }
        }
        Ok(())
    }
}

impl From<Vec<AST>> for Symbol {
    fn from(path: Vec<AST>) -> Self {
        Self { path }
    }
}

impl Symbol {
    pub fn namespace(&self) -> Vec<String> {
        self.path
            .iter()
            .take(self.path.len() - 1)
            .map(|e| match &e.kind {
                ASTKind::EscapedText(s) => s.to_owned(),
                _ => unreachable!(),
            })
            .collect()
    }
    pub fn name(&self) -> String {
        match &self.path.last().unwrap().kind {
            ASTKind::EscapedText(s) => s.to_owned(),
            _ => unreachable!(),
        }
    }
}
