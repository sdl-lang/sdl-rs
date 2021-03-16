mod config;
mod regroup;

pub use crate::parser::config::ParserConfig;
use crate::{parser::regroup::PREC_CLIMBER, Result};
use sdl_ast::{ast::CallChain, Template, ASTNode};
use sdl_pest::{Pair, Pairs, Parser, Rule, SDLParser};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParserConfig {
    pub fn parse(&mut self, input: impl AsRef<str>) -> Result<ASTNode> {
        let input = input.as_ref().replace("\r\n", "\n").replace("\\\n", "").replace("\t", &" ".repeat(self.tab_size));
        Ok(self.parse_program(SDLParser::parse(Rule::program, &input)?))
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> ASTNode {
        let mut codes = vec![];
        for pair in pairs {
            if let Rule::statement = pair.as_rule() {
                codes.push(self.parse_statement(pair));
            };
        }
        ASTNode::program(codes)
    }
    fn parse_statement(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::expression => self.parse_expression(pair),
                Rule::if_statement => self.parse_if_else(pair),
                Rule::for_statement => self.parse_for_in(pair),
                Rule::assign_statement => self.parse_assign(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        ASTNode::statement(codes, r)
    }
    fn parse_block(&self, pairs: Pair<Rule>) -> ASTNode {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::statement => self.parse_statement(pair),
            _ => unreachable!(),
        }
    }
}

impl ParserConfig {
    fn parse_if_else(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut conditions = vec![];
        let mut actions = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::expr => conditions.push(self.parse_expr(pair)),
                Rule::block => actions.push(self.parse_block(pair)),
                _ => debug_cases!(pair),
            };
        }
        ASTNode::if_else_chain(conditions, actions, r)
    }

    fn parse_for_in(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut guard = None;
        let mut for_else = None;
        let (mut pattern, mut terms, mut block) = Default::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::pattern | Rule::pattern_bare => pattern = self.parse_pattern(pair),
                Rule::expr => terms = self.parse_expr(pair),
                Rule::block => block = self.parse_block(pair),
                Rule::for_if => guard = Some(self.parse_expr(pair.into_inner().nth(0).unwrap())),
                Rule::for_else => for_else = Some(self.parse_block(pair.into_inner().nth(0).unwrap())),
                // _ => debug_cases!(pair),
                _ => unreachable!(),
            };
        }

        ASTNode::for_in_loop(pattern, terms, block, guard, for_else, r)
    }
}

impl ParserConfig {
    fn parse_expression(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut terms = pairs.into_inner();
        let expr = self.parse_expr(terms.next().unwrap());
        let eos = terms.next().is_some();
        ASTNode::expression(expr, eos, r)
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        PREC_CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                //Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                _ => debug_cases!(pair),
            },
            |left: ASTNode, op: Pair<Rule>, right: ASTNode| match op.as_rule() {
                _ => ASTNode::infix_expression(self.parse_operation(op, "="), left, right, r),
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut base = CallChain::default();
        // let mut prefix = vec![];
        // let mut suffix = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE|Rule::COMMENT => continue,
                Rule::data => base = CallChain::new(self.parse_data(pair)) ,
                Rule::dot_call=> base += self.parse_dot_call(pair),
                //Rule::term => base = self.parse_node(pair),
                //Rule::Prefix => prefix.push(pair.as_str().to_string()),
                //Rule::Suffix => suffix.push(pair.as_str().to_string()),
                _ => debug_cases!(pair),
                //_ => unreachable!(),
            };
        }
        match base.chain.is_empty() {
            true => base.base,
            false => ASTNode::call_chain(base, r),
        }
    }

    fn parse_pattern(&self, pairs: Pair<Rule>) -> ASTNode {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::SYMBOL => self.parse_symbol(pair),
            _ => debug_cases!(pair),
        }
    }

    fn parse_operation(&self, pairs: Pair<Rule>, kind: &str) -> ASTNode {
        let r = self.get_position(&pairs);
        let op = pairs.as_str();
        ASTNode::operation(op, kind, r)
    }

    fn parse_assign(&self, pairs: Pair<Rule>) -> ASTNode {
        let mut terms = pairs.into_inner();
        let pattern = self.parse_pattern(terms.next().unwrap());
        let expr = self.parse_expr(terms.next().unwrap());
        unreachable!("{:?}\n{:?}", pattern, expr)
    }

    fn parse_dot_call(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut positive = true;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Dot => continue,
                Rule::Symbol => continue,
                Rule::apply => continue,
                Rule::Minus => positive = false,
                Rule::Plus => positive = true,
                Rule::Integer => return ASTNode::call_index(pair.as_str(), positive, r),
                _ => debug_cases!(pair),
            };
        }
        unreachable!()
    }
}

impl ParserConfig {
    fn parse_data(&self, pairs: Pair<Rule>) -> ASTNode {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::template => self.parse_template(pair),
            Rule::list => self.parse_list(pair),
            Rule::String => self.parse_string(pair),
            Rule::Number => self.parse_number(pair),
            Rule::Symbol => self.parse_namespace(pair),
            Rule::SpecialValue => self.parse_special(pair),

            _ => debug_cases!(pair),
        }
    }
    fn parse_template(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut tag = ASTNode::default();
        let mut attributes = vec![];
        let mut arguments = vec![];
        let mut children = vec![];
        let pair = pairs.into_inner().nth(0).unwrap();
        let mut template = match pair.as_rule() {
            Rule::SelfClose => Template::self_close(),
            Rule::HTMLBad => Template::html_bad(),
            Rule::OpenClose => Template::open_close(),
            _ => debug_cases!(pair),
        };
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::Symbol => tag = self.parse_namespace(inner),
                Rule::HTMLBadTag => tag = self.parse_symbol(inner),
                Rule::text_mode => children.push(self.parse_text_mode(inner)),

                Rule::BadSymbol => attributes.push(self.parse_string(inner)),
                Rule::html_pair => arguments.push(self.parse_pair(inner)),
                _ => debug_cases!(inner),
            };
        }
        template.set_tag(tag);
        template.set_attributes(attributes);
        template.set_arguments(arguments);
        return ASTNode::template(template, r);
    }
    fn parse_text_mode(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut terms = vec![];
        let mut text = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::statement => terms.push(self.parse_statement(pair)),
                Rule::HTMLText => text.push(pair.as_str()),
                _ => debug_cases!(pair),
            };
        }
        ASTNode::block(terms, r)
    }

    fn parse_list(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut terms = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::Comma => continue,
                Rule::expr => terms.push(self.parse_expr(pair)),
                _ => debug_cases!(pair),
            };
        }
        ASTNode::list(terms, r)
    }
    fn parse_pair(&self, pairs: Pair<Rule>) -> (ASTNode, ASTNode) {
        let (mut key, mut value) = Default::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Set => continue,
                Rule::BadSymbol => key = self.parse_string(pair),
                Rule::term => value = self.parse_term(pair),
                _ => debug_cases!(pair),
            };
        }
        (key, value)
    }
    fn parse_namespace(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut value = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => value.push(self.parse_string(pair)),
                _ => debug_cases!(pair),
            };
        }
        ASTNode::symbol(value, r)
    }
    fn parse_symbol(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let value = vec![self.parse_string(pairs)];
        ASTNode::symbol(value, r)
    }

    fn parse_string(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let mut is_pure_string = true;
        let mut block = vec![];
        let mut _marks = 0;
        let mut buffer = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::StringEmpty => return ASTNode::string(String::new(), r),
                Rule::S1 | Rule::S2 | Rule::S3 | Rule::S4 => _marks += 1,
                Rule::NS1 | Rule::NS2 | Rule::NS3 | Rule::NS4 => {
                    let text = pair.as_str();
                    match text {
                        "{{" => buffer.push('{'),
                        "}}" => buffer.push('}'),
                        "\\n" => buffer.push('\n'),
                        _ => match text.starts_with('\\') {
                            true => buffer.push_str(&text[1..text.len()]),
                            false => buffer.push_str(text),
                        },
                    }
                }
                Rule::expr => {
                    is_pure_string = false;
                    if !buffer.is_empty() {
                        block.push(ASTNode::string(buffer, Default::default()));
                        buffer = String::new()
                    }
                    block.push(self.parse_expr(pair))
                }
                _ => debug_cases!(pair),
            };
        }
        match is_pure_string {
            true => ASTNode::string(buffer, r),
            false => {
                if !buffer.is_empty() {
                    block.push(ASTNode::string(buffer, Default::default()));
                }
                ASTNode::string_expression(block, Default::default(), r)
            }
        }
    }

    fn parse_number(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::Integer => ASTNode::integer(pair.as_str(), 10, r),
            Rule::Decimal => ASTNode::decimal(pair.as_str(), 10, r),
            Rule::DecimalBad => {
                let s = pair.as_str().to_string();
                match s.starts_with(".") {
                    true => ASTNode::decimal(&format!("0{}", s), 10, r),
                    false => ASTNode::decimal(&format!("{}0", s), 10, r),
                }
            }
            Rule::Byte => {
                let s = pair.as_str().to_string();
                match &s[0..1] {
                    "0b" => ASTNode::integer(pair.as_str(), 2, r),
                    "0o" => ASTNode::integer(pair.as_str(), 8, r),
                    "0x" => ASTNode::integer(pair.as_str(), 16, r),
                    _ => ASTNode::decimal(pair.as_str(), 16, r),
                }
            }
            _ => unreachable!(),
        }
    }
    fn parse_special(&self, pairs: Pair<Rule>) -> ASTNode {
        let r = self.get_position(&pairs);
        match pairs.as_str() {
            "true" => ASTNode::boolean(true, r),
            "false" => ASTNode::boolean(false, r),
            _ => ASTNode::null(r),
        }
    }
}
