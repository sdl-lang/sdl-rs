mod config;
mod regroup;

pub use crate::parser::config::ParserConfig;
use crate::{parser::regroup::PREC_CLIMBER, Result};
use sdl_ast::{ast::CallChain, Template, AST};
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
    pub fn parse(&mut self, input: impl AsRef<str>) -> Result<AST> {
        let input = input.as_ref().replace("\r\n", "\n").replace("\\\n", "").replace("\t", &" ".repeat(self.tab_size));
        Ok(self.parse_program(SDLParser::parse(Rule::program, &input)?))
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> AST {
        let mut codes = vec![];
        for pair in pairs {
            if let Rule::statement = pair.as_rule() {
                codes.push(self.parse_statement(pair));
            };
        }
        AST::program(codes)
    }
    fn parse_statement(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
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
        AST::statement(codes, r)
    }
    fn parse_block(&self, pairs: Pair<Rule>) -> AST {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::statement => self.parse_statement(pair),
            _ => unreachable!(),
        }
    }
}

impl ParserConfig {
    fn parse_if_else(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut conditions = vec![];
        let mut actions = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::expr => conditions.push(self.parse_expr(pair)),
                Rule::block => actions.push(self.parse_block(pair)),
                _ => debug_cases!(pair),
            };
        }
        AST::if_else_chain(conditions, actions, r)
    }

    fn parse_for_in(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
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

        AST::for_in_loop(pattern, terms, block, guard, for_else, r)
    }
}

impl ParserConfig {
    fn parse_expression(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut terms = pairs.into_inner();
        let expr = self.parse_expr(terms.next().unwrap());
        let eos = terms.next().is_some();
        AST::expression(expr, eos, r)
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        PREC_CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                //Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                _ => debug_cases!(pair),
            },
            |left: AST, op: Pair<Rule>, right: AST| match op.as_rule() {
                _ => AST::infix_expression(self.parse_operation(op, "="), left, right, r),
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut base = CallChain::default();
        // let mut prefix = vec![];
        // let mut suffix = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::chain_call => base = self.parse_chain_call(pair),
                //Rule::term => base = self.parse_node(pair),
                //Rule::Prefix => prefix.push(pair.as_str().to_string()),
                //Rule::Suffix => suffix.push(pair.as_str().to_string()),
                _ => debug_cases!(pair),
                //_ => unreachable!(),
            };
        }
        match base.chain.is_empty() {
            true => base.base,
            false => AST::call_chain(base, r)
        }
    }

    fn parse_pattern(&self, pairs: Pair<Rule>) -> AST {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::SYMBOL => self.parse_symbol(pair),
            _ => debug_cases!(pair),
        }
    }

    fn parse_operation(&self, pairs: Pair<Rule>, kind: &str) -> AST {
        let r = self.get_position(pairs.as_span());
        let op = pairs.as_str();
        AST::operation(op, kind, r)
    }

    fn parse_assign(&self, pairs: Pair<Rule>) -> AST {
        let mut terms = pairs.into_inner();
        let pattern = self.parse_pattern(terms.next().unwrap());
        let expr = self.parse_expr(terms.next().unwrap());
        unreachable!("{:?}\n{:?}", pattern, expr)
    }

    fn parse_chain_call(&self, pairs: Pair<Rule>) -> CallChain {
        // let r = self.get_position(pairs.as_span());
        let mut items = pairs.into_inner();
        let mut base = CallChain::new(self.parse_data(items.next().unwrap()));
        for pair in items {
            match pair.as_rule() {
                Rule::dot_call => base += self.parse_dot_call(pair),
                _ => debug_cases!(pair),
            };
        }
        return base
    }

    fn parse_dot_call(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        // let mut terms = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Dot => continue,
                Rule::Symbol => continue,
                Rule::apply => continue,
                Rule::Integer => return AST::call_index(pair.as_str(), r),
                _ => debug_cases!(pair),
            };
        }
        unreachable!()
    }
}

impl ParserConfig {
    fn parse_data(&self, pairs: Pair<Rule>) -> AST {
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
    fn parse_template(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut tag = AST::default();
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
        return AST::template(template, r);
    }
    fn parse_text_mode(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut terms = vec![];
        let mut text = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::statement => terms.push(self.parse_statement(pair)),
                Rule::text_char => text.push(pair.as_str()),
                _ => debug_cases!(pair),
            };
        }
        AST::block(terms, r)
    }

    fn parse_list(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut terms = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::Comma => continue,
                Rule::expr => terms.push(self.parse_expr(pair)),
                _ => debug_cases!(pair),
            };
        }
        AST::list(terms, r)
    }
    fn parse_pair(&self, pairs: Pair<Rule>) -> (AST, AST) {
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
    fn parse_namespace(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut value = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => value.push(self.parse_string(pair)),
                _ => debug_cases!(pair),
            };
        }
        AST::symbol(value, r)
    }
    fn parse_symbol(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let value = vec![self.parse_string(pairs)];
        AST::symbol(value, r)
    }

    fn parse_string(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut block = vec![];
        let mut marks = 0;
        let mut buffer = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Quotation|Rule::Quote|Rule::Apostrophe=>marks+=1,
                Rule::NonQuotation |Rule::NonQuote| Rule::NonApostrophe=> {
                    buffer.push_str(pair.as_str())
                }
                Rule::StringEscaped=> {
                    match pair.as_str() {
                        "\\\\" => buffer.push('\\'),
                        "\\\"" => buffer.push('\"'),
                        _ => debug_cases!(pair)
                    }
                },
                Rule::expr=> {
                    if !buffer.is_empty() {
                        block.push(AST::string( Default::default()))
                        buffer = String::new()
                    }


                },
                _ => debug_cases!(pair),
            };
        }
        AST::string(buffer, r)
    }

    fn parse_number(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::Integer => AST::integer(pair.as_str(), 10, r),
            Rule::Decimal => AST::decimal(pair.as_str(), 10, r),
            Rule::DecimalBad => {
                let s = pair.as_str().to_string();
                match s.starts_with(".") {
                    true => AST::decimal(&format!("0{}", s), 10, r),
                    false => AST::decimal(&format!("{}0", s), 10, r),
                }
            }
            Rule::Byte => {
                let s = pair.as_str().to_string();
                match &s[0..1] {
                    "0b" => AST::integer(pair.as_str(), 2, r),
                    "0o" => AST::integer(pair.as_str(), 8, r),
                    "0x" => AST::integer(pair.as_str(), 16, r),
                    _ => AST::decimal(pair.as_str(), 16, r),
                }
            }
            _ => unreachable!(),
        }
    }
    fn parse_special(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        match pairs.as_str() {
            "true" => AST::boolean(true, r),
            "false" => AST::boolean(false, r),
            _ => AST::null(r),
        }
    }
}
