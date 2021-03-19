mod expression;
mod loops;
mod operations;
mod symbol;
mod template;
mod check;

pub use crate::ast::{
    expression::{CallChain, InfixExpression, StringExpression, UnaryExpression},
    loops::{ForInLoop, IfElseChain},
    operations::Operator,
    symbol::Symbol,
    template::{Template, TemplateKind},
};
use bigdecimal::BigDecimal;
use num::BigInt;
use std::fmt::{self, Debug, Display, Formatter};
pub use lsp_types::Range;
pub use lsp_types::Position;

#[derive(Clone, Eq, PartialEq)]
pub struct ASTNode {
    pub kind: ASTKind,
    pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    None,
    Program(Vec<ASTNode>),
    Block(Vec<ASTNode>),
    Statement(Vec<ASTNode>),

    IfElseChain(Box<IfElseChain>),
    ForInLoop(Box<ForInLoop>),

    Expression(Box<ASTNode>, bool),
    CallChain(Box<CallChain>),
    CallIndex(Box<BigInt>),
    InfixExpression(Box<InfixExpression>),
    PrefixExpression(Box<UnaryExpression>),
    SuffixExpression(Box<UnaryExpression>),
    StringExpression(Box<StringExpression>),

    Template(Box<Template>),

    Text,
    List(Vec<ASTNode>),
    Dict,
    Pair(Box<ASTNode>, Box<ASTNode>),

    Null,
    Boolean(bool),
    EscapedText(String),
    UnescapedText(String),
    Integer(Box<BigInt>),
    Decimal(Box<BigDecimal>),
    Operator(Box<Operator>),
    Symbol(Box<Symbol>),
}

impl Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ASTNode { kind, range } => {
                let mut builder = f.debug_struct("AST");
                builder.field("kind", kind);
                builder.field("range", range);
                builder.finish()
            }
        }
    }
}

impl Default for ASTNode {
    fn default() -> Self {
        Self { kind: ASTKind::None, range: Default::default() }
    }
}

impl ASTNode {
    pub fn as_vec(&self) -> Vec<ASTNode> {
        match &self.kind {
            ASTKind::Program(v) | ASTKind::Block(v) => v.to_owned(),
            _ => vec![],
        }
    }
}

impl ASTNode {
    pub fn program(children: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::Program(children), range: Default::default() }
    }
    pub fn block(children: Vec<ASTNode>, range: Range) -> Self {
        Self { kind: ASTKind::Block(children), range, }
    }
    pub fn statement(children: Vec<ASTNode>, range: Range) -> Self {
        Self { kind: ASTKind::Statement(children), range, }
    }

    pub fn if_else_chain(cds: Vec<ASTNode>, acts: Vec<ASTNode>, range: Range) -> Self {
        let kind = ASTKind::IfElseChain(Box::new(IfElseChain::build(cds, acts)));
        Self { kind, range, }
    }

    pub fn for_in_loop(pattern: ASTNode, terms: ASTNode, block: ASTNode, guard: Option<ASTNode>, for_else: Option<ASTNode>, range: Range) -> Self {
        let kind = ASTKind::ForInLoop(Box::new(ForInLoop { pattern, terms, guard, block, for_else }));
        Self { kind, range, }
    }

    pub fn expression(children: ASTNode, eos: bool, range: Range) -> Self {
        let kind = ASTKind::Expression(Box::new(children), eos);
        Self { kind, range, }
    }

    pub fn operation(op: &str, kind: &str, range: Range) -> Self {
        let o = match kind {
            "<" => Operator::prefix(op),
            ">" => Operator::suffix(op),
            _ => Operator::infix(op),
        };
        let kind = ASTKind::Operator(Box::new(o));
        Self { kind, range, }
    }

    pub fn string_expression(value: Vec<ASTNode>, handler: Option<ASTNode>, range: Range) -> Self {
        let kind = ASTKind::StringExpression(Box::new(StringExpression { handler, inner: value }));
        Self { kind, range, }
    }

    pub fn infix_expression(op: ASTNode, lhs: ASTNode, rhs: ASTNode, range: Range) -> Self {
        let kind = ASTKind::InfixExpression(Box::new(InfixExpression { op, lhs, rhs }));
        Self { kind, range, }
    }

    pub fn prefix_expression(op: ASTNode, rhs: ASTNode, range: Range) -> Self {
        let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: rhs }));
        Self { kind, range, }
    }

    pub fn suffix_expression(op: ASTNode, lhs: ASTNode, range: Range) -> Self {
        let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: lhs }));
        Self { kind, range, }
    }

    pub fn call_chain(chain: CallChain, range: Range) -> Self {
        Self { kind: ASTKind::CallChain(Box::new(chain)), range, }
    }

    pub fn call_index(index: &str, is_positive: bool, range: Range) -> Self {
        let n = BigInt::parse_bytes(index.as_bytes(), 10).unwrap_or_default();
        let kind = match is_positive {
            true => {ASTKind::CallIndex(Box::new(n))}
            false => {ASTKind::CallIndex(Box::new(-n))}
        };
        Self { kind, range, }
    }

    pub fn template(value: Template, range: Range) -> Self {
        Self { kind: ASTKind::Template(Box::new(value)), range, }
    }

    pub fn list(value: Vec<ASTNode>, range: Range) -> Self {
        Self { kind: ASTKind::List(value), range, }
    }

    pub fn null(range: Range) -> Self {
        Self { kind: ASTKind::Null, range, }
    }

    pub fn boolean(value: bool, range: Range) -> Self {
        Self { kind: ASTKind::Boolean(value), range, }
    }
    pub fn string(value: String, range: Range) -> Self {
        Self { kind: ASTKind::UnescapedText(value), range, }
    }
    pub fn string_escaped(value: String, range: Range) -> Self {
        Self { kind: ASTKind::EscapedText(value), range, }
    }
    pub fn integer(value: &str, base: u32, range: Range) -> Self {
        let n = BigInt::parse_bytes(value.as_bytes(), base).unwrap_or_default();
        Self { kind: ASTKind::Integer(Box::new(n)), range, }
    }
    pub fn decimal(value: &str, base: u32, range: Range) -> Self {
        let n = BigDecimal::parse_bytes(value.as_bytes(), base).unwrap_or_default();
        Self { kind: ASTKind::Decimal(Box::new(n)), range, }
    }
    pub fn symbol(value: Vec<ASTNode>, range: Range) -> Self {
        Self { kind: ASTKind::Symbol(Box::new(Symbol::from(value))), range, }
    }
}
