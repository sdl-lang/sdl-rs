mod template;

pub use crate::ast::template::{Template, TemplateKind};
use crate::TextRange;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
};

type RangedString = (String, TextRange);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AST {
    Node {
        kind: ASTKind,
        //      children: Box<[AST]>,
        children: Vec<AST>,
        r: Option<Box<TextRange>>,
    },
    Leaf {
        kind: ASTKind,
        r: Option<Box<TextRange>>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    None,
    Program,
    Statement,
    Template(Box<Template>),

    InfixExpression,
    PrefixExpression,
    SuffixExpression,

    Null,
    Boolean(bool),
    String(String),
}

impl Default for AST {
    fn default() -> Self {
        Self::Leaf {
            kind: ASTKind::None,
            r: Default::default(),
        }
    }
}

impl AST {
    pub fn kind(&self) -> ASTKind {
        match self {
            Self::Node { kind, .. } | Self::Leaf { kind, .. } => kind.to_owned(),
        }
    }

    pub fn children(&self) -> Vec<AST> {
        match self {
            Self::Node { children, .. } => children.to_vec(),
            Self::Leaf { .. } => vec![],
        }
    }
    pub fn range(&self) -> TextRange {
        match self {
            Self::Node { r, .. } | Self::Leaf { r, .. } => r.clone().unwrap_or_default().as_ref().clone(),
        }
    }
}

impl AST {
    pub fn program(children: Vec<AST>) -> Self {
        Self::Node {
            kind: ASTKind::Program,
            children,
            r: Default::default(),
        }
    }
    pub fn statement(children: Vec<AST>, r: TextRange) -> Self {
        Self::Node {
            kind: ASTKind::Statement,
            children,
            r: box_range(r),
        }
    }
    pub fn expression(children: Vec<AST>, r: TextRange) -> Self {
        Self::Node {
            kind: ASTKind::Statement,
            children,
            r: box_range(r),
        }
    }

    pub fn infix(op: &str, lhs: AST, rhs: AST, r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::InfixExpression,
            r: box_range(r),
        }
    }

    pub fn prefix(op: &str, rhs: AST, r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::PrefixExpression,
            r: box_range(r),
        }
    }

    pub fn suffix(op: &str, lhs: AST, r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::SuffixExpression,
            r: box_range(r),
        }
    }

    pub fn template(value: Template, r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::Template(Box::new(value)),
            r: box_range(r),
        }
    }

    pub fn null(r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::Null,
            r: box_range(r),
        }
    }
    pub fn boolean(value: bool, r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::Boolean(value),
            r: box_range(r),
        }
    }
    pub fn string(value: String, r: TextRange) -> Self {
        Self::Leaf {
            kind: ASTKind::String(value),
            r: box_range(r),
        }
    }
}

fn box_range(r: TextRange) -> Option<Box<TextRange>> {
    match r.sum() {
        0 => None,
        _ => Some(Box::new(r)),
    }
}
