mod expression;
mod loops;
mod operations;
mod symbol;
mod template;

pub use crate::ast::{
    expression::{CallChain, InfixExpression, StringExpression, UnaryExpression},
    loops::{ForInLoop, IfElseChain},
    operations::Operator,
    symbol::Symbol,
    template::{Template, TemplateKind},
};
use crate::TextRange;
use bigdecimal::BigDecimal;
use num::BigInt;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub struct ASTNode {
    pub kind: ASTKind,
    pub range: Option<Box<TextRange>>,
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
                match range {
                    None => (),
                    Some(s) => {
                        builder.field("range", s.as_ref());
                    }
                }
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
    pub fn block(children: Vec<ASTNode>, r: TextRange) -> Self {
        Self { kind: ASTKind::Block(children), range: box_range(r) }
    }
    pub fn statement(children: Vec<ASTNode>, r: TextRange) -> Self {
        Self { kind: ASTKind::Statement(children), range: box_range(r) }
    }

    pub fn if_else_chain(cds: Vec<ASTNode>, acts: Vec<ASTNode>, r: TextRange) -> Self {
        let kind = ASTKind::IfElseChain(Box::new(IfElseChain::build(cds, acts)));
        Self { kind, range: box_range(r) }
    }

    pub fn for_in_loop(pattern: ASTNode, terms: ASTNode, block: ASTNode, guard: Option<ASTNode>, for_else: Option<ASTNode>, r: TextRange) -> Self {
        let kind = ASTKind::ForInLoop(Box::new(ForInLoop { pattern, terms, guard, block, for_else }));
        Self { kind, range: box_range(r) }
    }

    pub fn expression(children: ASTNode, eos: bool, r: TextRange) -> Self {
        let kind = ASTKind::Expression(Box::new(children), eos);
        Self { kind, range: box_range(r) }
    }

    pub fn operation(op: &str, kind: &str, r: TextRange) -> Self {
        let o = match kind {
            "<" => Operator::prefix(op),
            ">" => Operator::suffix(op),
            _ => Operator::infix(op),
        };
        let kind = ASTKind::Operator(Box::new(o));
        Self { kind, range: box_range(r) }
    }

    pub fn string_expression(value: Vec<ASTNode>, handler: Option<ASTNode>, r: TextRange) -> Self {
        let kind = ASTKind::StringExpression(Box::new(StringExpression { handler, inner: value }));
        Self { kind, range: box_range(r) }
    }

    pub fn infix_expression(op: ASTNode, lhs: ASTNode, rhs: ASTNode, r: TextRange) -> Self {
        let kind = ASTKind::InfixExpression(Box::new(InfixExpression { op, lhs, rhs }));
        Self { kind, range: box_range(r) }
    }

    pub fn prefix_expression(op: ASTNode, rhs: ASTNode, r: TextRange) -> Self {
        let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: rhs }));
        Self { kind, range: box_range(r) }
    }

    pub fn suffix_expression(op: ASTNode, lhs: ASTNode, r: TextRange) -> Self {
        let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: lhs }));
        Self { kind, range: box_range(r) }
    }

    pub fn call_chain(chain: CallChain, r: TextRange) -> Self {
        Self { kind: ASTKind::CallChain(Box::new(chain)), range: box_range(r) }
    }

    pub fn call_index(index: &str, r: TextRange) -> Self {
        let n = BigInt::parse_bytes(index.as_bytes(), 10).unwrap_or_default();
        Self { kind: ASTKind::CallIndex(Box::new(n)), range: box_range(r) }
    }

    pub fn template(value: Template, r: TextRange) -> Self {
        Self { kind: ASTKind::Template(Box::new(value)), range: box_range(r) }
    }

    pub fn list(value: Vec<ASTNode>, r: TextRange) -> Self {
        Self { kind: ASTKind::List(value), range: box_range(r) }
    }

    pub fn null(r: TextRange) -> Self {
        Self { kind: ASTKind::Null, range: box_range(r) }
    }

    pub fn boolean(value: bool, r: TextRange) -> Self {
        Self { kind: ASTKind::Boolean(value), range: box_range(r) }
    }
    pub fn string(value: String, r: TextRange) -> Self {
        Self { kind: ASTKind::UnescapedText(value), range: box_range(r) }
    }
    pub fn string_escaped(value: String, r: TextRange) -> Self {
        Self { kind: ASTKind::EscapedText(value), range: box_range(r) }
    }
    pub fn integer(value: &str, base: u32, r: TextRange) -> Self {
        let n = BigInt::parse_bytes(value.as_bytes(), base).unwrap_or_default();
        Self { kind: ASTKind::Integer(Box::new(n)), range: box_range(r) }
    }
    pub fn decimal(value: &str, base: u32, r: TextRange) -> Self {
        let n = BigDecimal::parse_bytes(value.as_bytes(), base).unwrap_or_default();
        Self { kind: ASTKind::Decimal(Box::new(n)), range: box_range(r) }
    }
    pub fn symbol(value: Vec<ASTNode>, r: TextRange) -> Self {
        Self { kind: ASTKind::Symbol(Box::new(Symbol::from(value))), range: box_range(r) }
    }
}

fn box_range(r: TextRange) -> Option<Box<TextRange>> {
    match r.sum() {
        0 => None,
        _ => Some(Box::new(r)),
    }
}
