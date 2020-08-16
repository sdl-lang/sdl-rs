mod can_parse;
mod config;
mod regroup;

pub use crate::parser::can_parse::CanParse;
pub use crate::parser::config::ParserConfig;
use crate::ParserResult;
use sdl_ast::{ASTKind, AST};
use sdl_pest::{Pair, Pairs, Parser, Rule, SDLParser, PrecClimber, Assoc, Operator};
use url::Url;
use std::lazy::SyncLazy;




macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

#[rustfmt::skip]
pub static PREC_CLIMBER: SyncLazy<PrecClimber<Rule>> = SyncLazy::new(|| {
    use Rule::*;
    use Assoc::*;
    //TODO: use macro
    PrecClimber::new(vec![
        Operator::new(Set, Left),
        Operator::new(Plus, Left) | Operator::new(Minus, Left),
        Operator::new(Power, Right),
        Operator::new(Dot, Left)
    ])
});


impl ParserConfig {
    pub fn parse(&mut self, input: impl CanParse) -> ParserResult<AST> {
        if let Some(s) = input.as_url() {
            self.file_url = Some(s)
        }
        let input = input
            .as_text()?
            .replace("\r\n", "\n")
            .replace("\\\n", "")
            .replace("\t", &" ".repeat(self.tab_size));
        let pairs = SDLParser::parse(Rule::program, &input)?;
        Ok(self.parse_program(pairs))
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> AST {
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITESPACE => continue,
                Rule::statement=> self.parse_statement(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        AST::program(codes)
    }
    fn parse_statement(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::expression=>self.parse_expression(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        AST::statement(codes,r)
    }
    fn parse_expression(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        // let mut eos = false;
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITESPACE => continue,
                Rule::expr=>self.parse_expr(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        AST::expression(codes,r)
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        PREC_CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                Rule::bracket_call => debug_cases!(pair),
                _ => debug_cases!(pair),
            },
            |left: AST, op: Pair<Rule>, right: AST| match op.as_rule() {
                _ => AST::infix(
                    op.as_str(),
                    left,
                    right,
                    r
                ),
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> AST {
        // let pos = get_position(pairs.as_span());
        let mut base = AST::default();
        let mut prefix = vec![];
        let mut suffix = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::node => base = self.parse_node(pair),
                Rule::Prefix => prefix.push(pair.as_str().to_string()),
                Rule::Suffix => suffix.push(pair.as_str().to_string()),
                _ => unreachable!(),
            };
        }
        AST::default()
    }

    fn parse_node(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            return match pair.as_rule() {
                Rule::expr => self.parse_expr(pair),
                Rule::data => self.parse_data(pair),
                _ => debug_cases!(pair),
            };
        }
        return AST::default();
    }
    /*
    pub fn parse_command_block(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut cmd = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Escape => continue,
                Rule::SYMBOL => cmd = pair.as_str().to_string(),
                _ => debug_cases!(pair),
            };
        }
        let cmd = Command { cmd, kind: CommandKind::Normal, args: vec![], kvs: Default::default() };
        AST::command(cmd, r)
    }
    pub fn parse_paragraph(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let codes = self.parse_span(pairs);
        match codes.len() {
            0 => {
                return AST::default();
            }
            1 => {
                if let ASTKind::MathDisplay(math) = &codes[0].kind() {
                    return AST::math(math.as_ref().to_owned(), "block", r);
                }
            }
            _ => (),
        }
        AST::paragraph(codes, r)
    }
    fn parse_span(&self, pairs: Pair<Rule>) -> Vec<AST> {
        pairs.into_inner().map(|pair| self.parse_span_term(pair)).collect()
    }
    fn parse_span_term(&self, pair: Pair<Rule>) -> AST {
        match pair.as_rule() {
            Rule::EOI => AST::default(),
            Rule::Style => self.parse_styled_text(pair),
            Rule::TextRest => self.parse_normal_text(pair),
            Rule::TildeLine => self.parse_tilde_text(pair),
            Rule::Raw => self.parse_raw_text(pair),
            Rule::Math => self.parse_math_text(pair),
            Rule::RawRest | Rule::StyleRest | Rule::TildeRest | Rule::MathRest => self.parse_normal_text(pair),
            Rule::WHITE_SPACE | Rule::LINE_SEPARATOR => self.parse_normal_text(pair),
            Rule::Escaped => self.parse_escaped(pair),

            _ => debug_cases!(pair),
        }
    }

    fn parse_normal_text(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        AST::text(pairs.as_str().to_string(), "normal", r)
    }
    fn parse_styled_text(&self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str().to_string();
        let r = self.get_position(pairs.as_span());
        let mut level = 0;
        let mut text = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Asterisk => continue,
                Rule::StyleLevel => level += pair.as_str().len(),
                Rule::StyleText => text.extend(self.parse_span(pair)),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => AST::style(text, "*", r),
            2 => AST::style(text, "**", r),
            3 => AST::style(text, "***", r),
            _ => AST::text(s, "normal", r),
        }
    }
    fn parse_tilde_text(&self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str().to_string();
        let r = self.get_position(pairs.as_span());
        let mut level = 0;
        let mut text = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Tilde => continue,
                Rule::TildeLevel => level += pair.as_str().len(),
                Rule::TildeText => text = self.parse_span(pair),
                _ => debug_cases!(pair),
            };
        }
        match level {
            1 => AST::style(text, "~", r),
            2 => AST::style(text, "~~", r),
            3 => AST::style(text, "~~~", r),
            _ => AST::text(s, "normal", r),
        }
    }
    fn parse_raw_text(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        for pair in pairs.into_inner() {
            if let Rule::RawText = pair.as_rule() {
                return AST::text(pair.as_str().to_string(), "raw", r);
            };
        }
        return AST::default();
    }
    fn parse_math_text(&self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str().to_string();
        let r = self.get_position(pairs.as_span());
        let mut inner = pairs.into_inner();
        let level = inner.next().unwrap().as_str().len();
        let text = inner.next().unwrap().as_str().to_string();
        match level {
            1 => AST::math(text, "inline", r),
            2 => AST::math(text, "display", r),
            _ => AST::text(s, "normal", r),
        }
    }
    fn parse_escaped(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let c = match pairs.as_str().chars().next() {
            None => '\\',
            Some(s) => s,
        };
        AST::escaped(c, r)
    }
    */
}

impl ParserConfig {
    fn parse_data(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            return match pair.as_rule() {
                Rule::template=>self.parse_template(pair),
                _ => debug_cases!(pair),
            };
        }
        return AST::default();
    }
    fn parse_template(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            return match pair.as_rule() {
                _ => debug_cases!(pair),
            };
        }
        return AST::default();
    }
}

/*
fn parse_table_align(input: &str) -> Vec<u8> {
    let pairs = SDLParser::parse(Rule::TableMode, input).unwrap_or_else(|e| panic!("{}", e));
    let mut codes = vec![];
    let mut text = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => continue,
            Rule::WHITE_SPACE => text.push(' '),
            Rule::TableRest => text.push_str(pair.as_str()),
            Rule::TableMark => {
                let mut code = 0;
                if text.contains(":-") {
                    code += 1 << 0
                }
                if text.contains("-:") {
                    code += 1 << 1
                }
                codes.push(code);
                text = String::new();
            }
            _ => debug_cases!(pair),
        };
    }
    return codes;
}

#[derive(Debug)]
pub enum List {
    Quote,
    Ordered,
    Orderless,
}

impl List {
    pub fn get_type(input: &str) -> (usize, List) {
        let pairs = List::parse_pairs(input);
        let mut i = 0;
        let mut m = List::Quote;
        for pair in pairs {
            match pair.as_rule() {
                Rule::WHITE_SPACE => i += 1,
                Rule::ListMark => match pair.as_str() {
                    ">" => m = List::Quote,
                    "-" => m = List::Orderless,
                    _ => m = List::Ordered,
                },
                _ => return (i, m),
            };
        }
        return (i, m);
    }
    pub fn trim_indent(line: &str, _indent: usize, ty: &List) -> (bool, String) {
        let mut new = false;
        let mut vec: VecDeque<_> = List::parse_pairs(line).into_iter().collect();
        match ty {
            List::Quote => match vec[0].as_rule() {
                Rule::ListMark => match vec[0].as_str() {
                    ">" => {
                        vec.pop_front();
                    }
                    _ => (),
                },
                _ => (),
            },
            List::Ordered => match vec[0].as_rule() {
                Rule::ListMark => match vec[0].as_str() {
                    "-" | ">" => (),
                    _ => {
                        vec.pop_front();
                        new = true
                    }
                },
                _ => (),
            },
            List::Orderless => match vec[0].as_rule() {
                Rule::ListMark => match vec[0].as_str() {
                    "-" => {
                        vec.pop_front();
                        new = true
                    }
                    _ => (),
                },
                _ => (),
            },
        }
        let v: Vec<&str> = vec.iter().map(|x| x.as_str()).collect();
        return (new, v.join(""));
    }
    fn parse_pairs(input: &str) -> Pairs<Rule> {
        let p = SDLParser::parse(Rule::ListMode, input).unwrap_or_else(|e| panic!("{}", e));
        p.into_iter().next().unwrap().into_inner()
    }
}
*/
