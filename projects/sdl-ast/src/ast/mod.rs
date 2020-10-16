mod expression;
mod loops;
mod operations;
mod symbol;
mod template;

pub use crate::ast::{
    expression::{InfixExpression, UnaryExpression},
    loops::ForInLoop,
    operations::Operation,
    symbol::Symbol,
    template::{Template, TemplateKind},
};
use crate::TextRange;
use std::fmt::{self, Debug, Display, Formatter};
pub use crate::ast::loops::IfElseChain;

#[derive(Clone, Eq, PartialEq)]
pub struct AST {
    pub kind: ASTKind,
    pub range: Option<Box<TextRange>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    None,
    Program(Vec<AST>),
    Block(Vec<AST>),
    Statement(Vec<AST>),

    IfElseChain(Box<IfElseChain>),
    ForInLoop(Box<ForInLoop>),

    Expression(Box<AST>, bool),
    InfixExpression(Box<InfixExpression>),
    PrefixExpression(Box<UnaryExpression>),
    SuffixExpression(Box<UnaryExpression>),

    Template(Box<Template>),
    Text,
    List(Vec<AST>),
    Dict,
    Pair(Box<AST>, Box<AST>),

    Null,
    Boolean(bool),
    String(String),
    Operation(Box<Operation>),
    Symbol(Box<Symbol>),
}

impl Default for AST {
    fn default() -> Self {
        Self { kind: ASTKind::None, range: Default::default() }
    }
}

impl Debug for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ASTKind::Program(v) | ASTKind::Statement(v) => {
                for e in v {
                    Debug::fmt(e, f)?;
                    writeln!(f)?;
                }
                Ok(())
            }
            ASTKind::Null => write!(f, "null"),
            ASTKind::Boolean(v) => write!(f, "{}", v),
            ASTKind::String(v) => write!(f, "{}", v),
            _ => {
                let mut out = f.debug_struct("AST");
                out.field("kind", &self.kind);
                out.finish()
            }
        }
    }
}

impl AST {
    pub fn as_vec(&self) -> Vec<AST> {
        match &self.kind {
            ASTKind::Program(v) | ASTKind::Block(v) => v.to_owned(),
            _ => vec![],
        }
    }
}

impl AST {
    pub fn program(children: Vec<AST>) -> Self {
        Self { kind: ASTKind::Program(children), range: Default::default() }
    }
    pub fn block(children: Vec<AST>, r: TextRange) -> Self {
        Self { kind: ASTKind::Block(children), range: box_range(r) }
    }
    pub fn statement(children: Vec<AST>, r: TextRange) -> Self {
        Self { kind: ASTKind::Statement(children), range: box_range(r) }
    }

    pub fn if_else_chain(cds: Vec<AST>, acts: Vec<AST>, r: TextRange) -> Self {
        let kind = ASTKind::IfElseChain(Box::new(IfElseChain::build(cds,acts)));
        Self { kind, range: box_range(r) }
    }

    pub fn for_in_loop(pattern: AST, terms: AST, block: AST, r: TextRange) -> Self {
        let kind = ASTKind::ForInLoop(Box::new(ForInLoop { pattern, terms, block }));
        Self { kind, range: box_range(r) }
    }

    pub fn expression(children: AST, eos: bool, r: TextRange) -> Self {
        let kind = ASTKind::Expression(Box::new(children), eos);
        Self { kind, range: box_range(r) }
    }

    pub fn operation(op: &str, kind: &str, r: TextRange) -> Self {
        let o = match kind {
            "<" => Operation::prefix(op),
            ">" => Operation::suffix(op),
            _ => Operation::infix(op),
        };
        let kind = ASTKind::Operation(Box::new(o));
        Self { kind, range: box_range(r) }
    }

    pub fn infix_expression(op: AST, lhs: AST, rhs: AST, r: TextRange) -> Self {
        let kind = ASTKind::InfixExpression(Box::new(InfixExpression { op, lhs, rhs }));
        Self { kind, range: box_range(r) }
    }

    pub fn prefix_expression(op: AST, rhs: AST, r: TextRange) -> Self {
        let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: rhs }));
        Self { kind, range: box_range(r) }
    }

    pub fn suffix_expression(op: AST, lhs: AST, r: TextRange) -> Self {
        let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: lhs }));
        Self { kind, range: box_range(r) }
    }

    pub fn template(value: Template, r: TextRange) -> Self {
        Self { kind: ASTKind::Template(Box::new(value)), range: box_range(r) }
    }

    pub fn list(value: Vec<AST>, r: TextRange) -> Self {
        Self { kind: ASTKind::List(value), range: box_range(r) }
    }

    pub fn null(r: TextRange) -> Self {
        Self { kind: ASTKind::Null, range: box_range(r) }
    }

    pub fn integer(value: String, r: TextRange) -> Self {
        Self { kind: ASTKind::String(value), range: box_range(r) }
    }
    pub fn decimal(value: String, r: TextRange) -> Self {
        Self { kind: ASTKind::String(value), range: box_range(r) }
    }
    pub fn boolean(value: bool, r: TextRange) -> Self {
        Self { kind: ASTKind::Boolean(value), range: box_range(r) }
    }
    pub fn string(value: String, r: TextRange) -> Self {
        Self { kind: ASTKind::String(value), range: box_range(r) }
    }
    pub fn symbol(value: Vec<AST>, r: TextRange) -> Self {
        Self { kind: ASTKind::Symbol(Box::new(Symbol::from(value))), range: box_range(r) }
    }
}

fn box_range(r: TextRange) -> Option<Box<TextRange>> {
    match r.sum() {
        0 => None,
        _ => Some(Box::new(r)),
    }
}
