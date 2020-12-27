pub struct SDLParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    emptyStatement,
    eos,
    comma_or_semi,
    importStatement,
    use_alias,
    use_module_select,
    module_block,
    module_tuple,
    ModuleSplit,
    controlFlow,
    condition,
    block,
    if_statement,
    if_single,
    if_nested,
    if_single_else,
    if_nested_else,
    else_if_block,
    if_else_block,
    for_statement,
    for_if,
    for_else,
    pattern,
    pattern_bare,
    re_control,
    Control,
    classStatement,
    extendStatement,
    assign_statement,
    assign_word,
    define_statement,
    define_terms,
    define_pair,
    define_word,
    annotation,
    annotation_call,
    expression,
    expr,
    term,
    chain_call,
    dot_call,
    data,
    tuple,
    list,
    dict,
    dict_pair,
    dict_key,
    apply,
    apply_kv,
    slice,
    index,
    index_range,
    index_step,
    template,
    EmptyTemplate,
    Fragment,
    SDLFragment,
    OpenClose,
    SDLOpenClose,
    SelfClose,
    HTMLBad,
    HTMLBadTag,
    HTMLComment,
    HtmlDTD,
    html_term,
    html_pair,
    text_mode,
    text_char,
    HTMLEscape,
    SpecialValue,
    Number,
    Byte,
    Decimal,
    DecimalBad,
    Integer,
    String,
    StringEmpty,
    StringNormal,
    StringApostrophe,
    StringQuotation,
    StringAcute,
    StringQuote,
    Quote,
    Acute,
    Apostrophe,
    Quotation,
    WHITESPACE,
    COMMENT,
    MultiLineComment,
    SYMBOL,
    BadSymbol,
    Symbol,
    namespace,
    Prefix,
    Suffix,
    Infix,
    Logical,
    Compare,
    Additive,
    Multiplied,
    Assign,
    Set,
    Or,
    LazyOr,
    Star,
    Escape,
    Proportion,
    Dot,
    Comma,
    Semicolon,
    Colon,
    Question,
    Underline,
    Load,
    Save,
    LeftShift,
    RightShift,
    LessEqual,
    GraterEqual,
    Plus,
    Minus,
    Power,
    Surd,
    Increase,
    Decrease,
    To,
    Elvis,
    LogicOr,
    LogicAnd,
    LogicNot,
    Ellipsis,
    Concat,
    Destruct,
    Bang,
    Sharp,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for SDLParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::statement, |state| self::emptyStatement(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::importStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::classStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::extendStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::controlFlow(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::assign_statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::define_statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::annotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| self::expression(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn emptyStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::emptyStatement, |state| self::eos(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma_or_semi(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Comma(state).or_else(|state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn importStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::importStatement, |state| state.sequence(|state| state.match_string("import").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Dot(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Dot(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::use_alias(state)).or_else(|state| state.restore_on_err(|state| self::use_module_select(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_alias(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::use_alias, |state| state.restore_on_err(|state| state.sequence(|state| self::String(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("as")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))).or_else(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("as")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_module_select(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::use_module_select, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::module_block(state)).or_else(|state| self::Star(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::comma_or_semi(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::comma_or_semi(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::comma_or_semi(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_tuple(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_tuple, |state| state.restore_on_err(|state| self::use_alias(state)).or_else(|state| state.restore_on_err(|state| self::use_module_select(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleSplit(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Dot(state).or_else(|state| self::Proportion(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn controlFlow(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::re_control(state)).or_else(|state| state.restore_on_err(|state| self::if_statement(state))).or_else(|state| state.restore_on_err(|state| self::for_statement(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn condition(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.restore_on_err(|state| self::expr(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::if_statement, |state| state.restore_on_err(|state| self::if_nested_else(state)).or_else(|state| state.restore_on_err(|state| self::if_single_else(state))).or_else(|state| state.restore_on_err(|state| self::if_nested(state))).or_else(|state| state.restore_on_err(|state| self::if_single(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_single(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_nested(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::else_if_block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::else_if_block(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::else_if_block(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_single_else(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::if_else_block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_nested_else(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::else_if_block(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::else_if_block(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::else_if_block(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::if_else_block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn else_if_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.match_string("else").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("if"))).or_else(|state| state.match_string("ef")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_else_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("else").or_else(|state| state.match_string("es")).or_else(|state| state.match_string("el")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_statement, |state| state.sequence(|state| state.match_string("for").and_then(|state| super::hidden::skip(state)).and_then(|state| self::pattern(state).or_else(|state| self::pattern_bare(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("in")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::for_if(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::for_else(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_if(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_if, |state| state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_else(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_else, |state| state.sequence(|state| state.match_string("else").and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn pattern(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::pattern, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn pattern_bare(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::pattern_bare, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn re_control(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::re_control, |state| state.sequence(|state| self::Control(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Control(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Control, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("return").or_else(|state| state.match_string("yield")).or_else(|state| state.match_string("break")).or_else(|state| state.match_string("pass"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn classStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::classStatement, |state| state.sequence(|state| state.match_string("class").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn extendStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::extendStatement, |state| state.sequence(|state| state.match_string("extend").and_then(|state| super::hidden::skip(state)).and_then(|state| self::Symbol(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.match_string("with").or_else(|state| self::Colon(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_statement, |state| state.restore_on_err(|state| state.sequence(|state| self::assign_word(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::pattern(state).or_else(|state| self::pattern_bare(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::pattern(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_word(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_word, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("let").or_else(|state| state.match_string("const"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_statement, |state| state.restore_on_err(|state| state.sequence(|state| self::define_word(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_terms(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_terms(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::block(state)).or_else(|state| state.restore_on_err(|state| self::statement(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_terms(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_terms, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")").or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::define_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_pair(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_pair, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Set(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_word(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_word, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("def")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn annotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::annotation, |state| state.sequence(|state| state.sequence(|state| self::annotation_call(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::annotation_call(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::annotation_call(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn annotation_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::annotation_call, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("@").and_then(|state| state.restore_on_err(|state| self::list(state)).or_else(|state| state.restore_on_err(|state| self::apply(state))).or_else(|state| self::Symbol(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::expr, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::term, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::Prefix(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Prefix(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.restore_on_err(|state| self::chain_call(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn chain_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::chain_call, |state| state.sequence(|state| self::data(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::slice(state)).or_else(|state| state.restore_on_err(|state| self::apply(state))).or_else(|state| self::Suffix(state)).or_else(|state| state.restore_on_err(|state| self::dot_call(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::slice(state)).or_else(|state| state.restore_on_err(|state| self::apply(state))).or_else(|state| self::Suffix(state)).or_else(|state| state.restore_on_err(|state| self::dot_call(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dot_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::dot_call, |state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Integer(state).or_else(|state| state.sequence(|state| self::Symbol(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::apply(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::data, |state| state.restore_on_err(|state| self::template(state)).or_else(|state| state.restore_on_err(|state| self::dict(state))).or_else(|state| state.restore_on_err(|state| self::list(state))).or_else(|state| self::SpecialValue(state)).or_else(|state| self::Number(state)).or_else(|state| state.restore_on_err(|state| self::String(state))).or_else(|state| self::Symbol(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tuple(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::tuple, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.sequence(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]").or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}").or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::dict_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict_pair(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::dict_pair, |state| state.sequence(|state| self::dict_key(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict_key(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Integer(state).or_else(|state| self::SYMBOL(state)).or_else(|state| state.restore_on_err(|state| self::String(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::apply, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")").or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::apply_kv(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_kv(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::apply_kv, |state| state.restore_on_err(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).or_else(|state| state.restore_on_err(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn slice(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::slice, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::index, |state| state.restore_on_err(|state| self::index_step(state)).or_else(|state| state.restore_on_err(|state| self::index_range(state))).or_else(|state| state.restore_on_err(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index_range(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::index_range, |state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index_step(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::index_step, |state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn template(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::template, |state| state.restore_on_err(|state| self::SDLFragment(state)).or_else(|state| state.restore_on_err(|state| self::Fragment(state))).or_else(|state| self::EmptyTemplate(state)).or_else(|state| state.restore_on_err(|state| self::SDLOpenClose(state))).or_else(|state| state.restore_on_err(|state| self::OpenClose(state))).or_else(|state| state.restore_on_err(|state| self::HTMLBad(state))).or_else(|state| state.restore_on_err(|state| self::SelfClose(state))).or_else(|state| self::HtmlDTD(state)).or_else(|state| self::HTMLComment(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyTemplate(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EmptyTemplate, |state| state.match_string("<|>"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Fragment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Fragment, |state| state.sequence(|state| state.match_string("<>").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::text_mode(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::text_mode(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("</>"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SDLFragment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SDLFragment, |state| state.sequence(|state| state.match_string("<\\>").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("</>"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn OpenClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::OpenClose, |state| state.sequence(|state| state.match_string("<").and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| self::Symbol(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::html_term(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::html_term(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::text_mode(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::text_mode(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("</")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SDLOpenClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SDLOpenClose, |state| state.sequence(|state| state.match_string("<\\").and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| self::Symbol(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::html_term(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::html_term(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("</")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SelfClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SelfClose, |state| state.sequence(|state| state.match_string("<").and_then(|state| super::hidden::skip(state)).and_then(|state| self::Symbol(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::html_term(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::html_term(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("/>"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLBad, |state| state.sequence(|state| state.match_string("<").and_then(|state| super::hidden::skip(state)).and_then(|state| self::HTMLBadTag(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::html_term(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::html_term(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLBadTag(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLBadTag, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("img").or_else(|state| state.match_string("hr")).or_else(|state| state.match_string("br")).or_else(|state| state.match_string("input")).or_else(|state| state.match_string("link")).or_else(|state| state.match_string("meta")).or_else(|state| state.match_string("area")).or_else(|state| state.match_string("base")).or_else(|state| state.match_string("col")).or_else(|state| state.match_string("wbr")).or_else(|state| state.match_string("command")).or_else(|state| state.match_string("embed")).or_else(|state| state.match_string("keygen")).or_else(|state| state.match_string("param")).or_else(|state| state.match_string("source")).or_else(|state| state.match_string("track"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLComment, |state| state.sequence(|state| state.match_string("<!--").and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.match_string("-->"))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::ANY(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::ANY(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("-->"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HtmlDTD(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HtmlDTD, |state| state.sequence(|state| state.match_string("<!").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("DOCTYPE")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.lookahead(false, |state| state.match_string(">"))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::ANY(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::ANY(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn html_term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::html_pair(state)).or_else(|state| self::BadSymbol(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn html_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::html_pair, |state| state.sequence(|state| self::BadSymbol(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("=")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn text_mode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::text_mode, |state| state.sequence(|state| state.match_string("{").and_then(|state| state.repeat(|state| state.restore_on_err(|state| self::statement(state)))).and_then(|state| state.match_string("}"))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::template(state).and_then(|state| state.repeat(|state| state.restore_on_err(|state| self::template(state))))))).or_else(|state| state.sequence(|state| self::HTMLEscape(state).or_else(|state| self::text_char(state)).and_then(|state| state.repeat(|state| self::HTMLEscape(state).or_else(|state| self::text_char(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn text_char(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::text_char, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.lookahead(false, |state| state.match_string("{").or_else(|state| state.match_string("}")).or_else(|state| state.match_string("<")).or_else(|state| state.match_string(">"))).and_then(|state| self::ANY(state))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLEscape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLEscape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("&").and_then(|state| self::ASCII_ALPHA_LOWER(state)).and_then(|state| state.match_string(";"))).or_else(|state| state.sequence(|state| state.match_string("&#").and_then(|state| self::Integer(state)).and_then(|state| state.match_string(";")))).or_else(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecialValue(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SpecialValue, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| self::Byte(state).or_else(|state| self::Decimal(state)).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Byte, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("0").and_then(|state| state.match_string("b").or_else(|state| state.match_string("B"))).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state)))))).or_else(|state| state.sequence(|state| state.match_string("0").and_then(|state| state.match_string("o").or_else(|state| state.match_string("O"))).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state))))))).or_else(|state| state.sequence(|state| state.match_string("0").and_then(|state| state.sequence(|state| state.match_string("x").or_else(|state| state.match_string("X")).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))))).or_else(|state| state.sequence(|state| state.match_string("f").or_else(|state| state.match_string("F")).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Decimal, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::DecimalBad, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))).or_else(|state| state.sequence(|state| self::Dot(state).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0").or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.optional(|state| self::SYMBOL(state)).and_then(|state| state.restore_on_err(|state| self::StringNormal(state)).or_else(|state| self::StringEmpty(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEmpty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEmpty, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))).or_else(|state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Apostrophe(state)))).or_else(|state| state.sequence(|state| self::Quote(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quote(state)))).or_else(|state| state.sequence(|state| self::Acute(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Acute(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringNormal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Apostrophe(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Apostrophe(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringApostrophe(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Apostrophe(state)))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Quotation(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Quotation(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringQuotation(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::Acute(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Acute(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Acute(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringAcute(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Acute(state))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::Quote(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| state.sequence(|state| state.optional(|state| self::Quote(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Quote(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringQuote(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quote(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringApostrophe(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringApostrophe, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Apostrophe(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Apostrophe(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringQuotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringQuotation, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Quotation(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Quotation(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringAcute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringAcute, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Acute(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Acute(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Acute(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Acute(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringQuote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringQuote, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Quote(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Quote(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Escape(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Escape(state).or_else(|state| self::Quote(state)))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Quote(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quote, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("`")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Acute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Acute, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("´")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Apostrophe(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Apostrophe, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("'")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotation, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\"")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::WHITE_SPACE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("/*").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*/")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("*/")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))).or_else(|state| state.sequence(|state| self::Underline(state).and_then(|state| self::XID_CONTINUE(state)).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BadSymbol(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::BadSymbol, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).or_else(|state| state.match_string(":")).or_else(|state| state.match_string("_")).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state).or_else(|state| state.match_string(":")).or_else(|state| state.match_string("-")).or_else(|state| state.match_string(".")))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Symbol(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Symbol, |state| self::namespace(state).or_else(|state| self::SYMBOL(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn namespace(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::namespace, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| state.sequence(|state| self::Proportion(state).and_then(|state| self::SYMBOL(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| self::Proportion(state).and_then(|state| self::SYMBOL(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Prefix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Prefix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Bang(state).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Star(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Suffix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Suffix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Bang(state).or_else(|state| self::Question(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Infix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Compare(state).or_else(|state| self::Additive(state)).or_else(|state| self::Multiplied(state)).or_else(|state| self::Power(state)).or_else(|state| self::Set(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Logical(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Logical, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("a")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Compare(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::Compare, |state| state.sequence(|state| state.match_string("is").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("not"))).or_else(|state| state.match_string("!=")).or_else(|state| state.match_string("is")).or_else(|state| state.match_string("==")).or_else(|state| state.sequence(|state| state.match_string("not").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("in")))).or_else(|state| state.match_string("in")).or_else(|state| state.match_string(">")).or_else(|state| state.match_string("<"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Additive(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Additive, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Plus(state).or_else(|state| self::Minus(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Multiplied(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Multiplied, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Star(state).or_else(|state| state.match_string("/"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Assign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Assign, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+=").or_else(|state| state.match_string("-="))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Or(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Or, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LazyOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LazyOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Star(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Star, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Proportion(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Proportion, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("::").or_else(|state| state.match_string("∷"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Semicolon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Semicolon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Colon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Colon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":").or_else(|state| state.match_string("："))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Question(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Question, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Load(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Load, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<<").or_else(|state| state.match_string("⋘"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Save(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Save, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>>").or_else(|state| state.match_string("⋙"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LeftShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LeftShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<").or_else(|state| state.match_string("≪"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RightShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RightShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>").or_else(|state| state.match_string("≫"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LessEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LessEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GraterEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::GraterEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Plus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Plus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Minus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Minus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("-")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Power(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Power, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("^")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Surd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Surd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("√")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Increase(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Increase, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("++")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decrease(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Decrease, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("--")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn To(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::To, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("->")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Elvis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Elvis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||").or_else(|state| state.match_string("∧"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicAnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicAnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("&&").or_else(|state| state.match_string("∨"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicNot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicNot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("¬")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Ellipsis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Ellipsis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("...").or_else(|state| state.match_string("…"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Concat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Concat, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~~")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Destruct(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Destruct, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Bang(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Bang, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#")))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_BIN_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'1')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_OCT_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'7')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA_LOWER(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::WHITE_SPACE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::emptyStatement => rules::emptyStatement(state),
            Rule::eos => rules::eos(state),
            Rule::comma_or_semi => rules::comma_or_semi(state),
            Rule::importStatement => rules::importStatement(state),
            Rule::use_alias => rules::use_alias(state),
            Rule::use_module_select => rules::use_module_select(state),
            Rule::module_block => rules::module_block(state),
            Rule::module_tuple => rules::module_tuple(state),
            Rule::ModuleSplit => rules::ModuleSplit(state),
            Rule::controlFlow => rules::controlFlow(state),
            Rule::condition => rules::condition(state),
            Rule::block => rules::block(state),
            Rule::if_statement => rules::if_statement(state),
            Rule::if_single => rules::if_single(state),
            Rule::if_nested => rules::if_nested(state),
            Rule::if_single_else => rules::if_single_else(state),
            Rule::if_nested_else => rules::if_nested_else(state),
            Rule::else_if_block => rules::else_if_block(state),
            Rule::if_else_block => rules::if_else_block(state),
            Rule::for_statement => rules::for_statement(state),
            Rule::for_if => rules::for_if(state),
            Rule::for_else => rules::for_else(state),
            Rule::pattern => rules::pattern(state),
            Rule::pattern_bare => rules::pattern_bare(state),
            Rule::re_control => rules::re_control(state),
            Rule::Control => rules::Control(state),
            Rule::classStatement => rules::classStatement(state),
            Rule::extendStatement => rules::extendStatement(state),
            Rule::assign_statement => rules::assign_statement(state),
            Rule::assign_word => rules::assign_word(state),
            Rule::define_statement => rules::define_statement(state),
            Rule::define_terms => rules::define_terms(state),
            Rule::define_pair => rules::define_pair(state),
            Rule::define_word => rules::define_word(state),
            Rule::annotation => rules::annotation(state),
            Rule::annotation_call => rules::annotation_call(state),
            Rule::expression => rules::expression(state),
            Rule::expr => rules::expr(state),
            Rule::term => rules::term(state),
            Rule::chain_call => rules::chain_call(state),
            Rule::dot_call => rules::dot_call(state),
            Rule::data => rules::data(state),
            Rule::tuple => rules::tuple(state),
            Rule::list => rules::list(state),
            Rule::dict => rules::dict(state),
            Rule::dict_pair => rules::dict_pair(state),
            Rule::dict_key => rules::dict_key(state),
            Rule::apply => rules::apply(state),
            Rule::apply_kv => rules::apply_kv(state),
            Rule::slice => rules::slice(state),
            Rule::index => rules::index(state),
            Rule::index_range => rules::index_range(state),
            Rule::index_step => rules::index_step(state),
            Rule::template => rules::template(state),
            Rule::EmptyTemplate => rules::EmptyTemplate(state),
            Rule::Fragment => rules::Fragment(state),
            Rule::SDLFragment => rules::SDLFragment(state),
            Rule::OpenClose => rules::OpenClose(state),
            Rule::SDLOpenClose => rules::SDLOpenClose(state),
            Rule::SelfClose => rules::SelfClose(state),
            Rule::HTMLBad => rules::HTMLBad(state),
            Rule::HTMLBadTag => rules::HTMLBadTag(state),
            Rule::HTMLComment => rules::HTMLComment(state),
            Rule::HtmlDTD => rules::HtmlDTD(state),
            Rule::html_term => rules::html_term(state),
            Rule::html_pair => rules::html_pair(state),
            Rule::text_mode => rules::text_mode(state),
            Rule::text_char => rules::text_char(state),
            Rule::HTMLEscape => rules::HTMLEscape(state),
            Rule::SpecialValue => rules::SpecialValue(state),
            Rule::Number => rules::Number(state),
            Rule::Byte => rules::Byte(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::String => rules::String(state),
            Rule::StringEmpty => rules::StringEmpty(state),
            Rule::StringNormal => rules::StringNormal(state),
            Rule::StringApostrophe => rules::StringApostrophe(state),
            Rule::StringQuotation => rules::StringQuotation(state),
            Rule::StringAcute => rules::StringAcute(state),
            Rule::StringQuote => rules::StringQuote(state),
            Rule::Quote => rules::Quote(state),
            Rule::Acute => rules::Acute(state),
            Rule::Apostrophe => rules::Apostrophe(state),
            Rule::Quotation => rules::Quotation(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::BadSymbol => rules::BadSymbol(state),
            Rule::Symbol => rules::Symbol(state),
            Rule::namespace => rules::namespace(state),
            Rule::Prefix => rules::Prefix(state),
            Rule::Suffix => rules::Suffix(state),
            Rule::Infix => rules::Infix(state),
            Rule::Logical => rules::Logical(state),
            Rule::Compare => rules::Compare(state),
            Rule::Additive => rules::Additive(state),
            Rule::Multiplied => rules::Multiplied(state),
            Rule::Assign => rules::Assign(state),
            Rule::Set => rules::Set(state),
            Rule::Or => rules::Or(state),
            Rule::LazyOr => rules::LazyOr(state),
            Rule::Star => rules::Star(state),
            Rule::Escape => rules::Escape(state),
            Rule::Proportion => rules::Proportion(state),
            Rule::Dot => rules::Dot(state),
            Rule::Comma => rules::Comma(state),
            Rule::Semicolon => rules::Semicolon(state),
            Rule::Colon => rules::Colon(state),
            Rule::Question => rules::Question(state),
            Rule::Underline => rules::Underline(state),
            Rule::Load => rules::Load(state),
            Rule::Save => rules::Save(state),
            Rule::LeftShift => rules::LeftShift(state),
            Rule::RightShift => rules::RightShift(state),
            Rule::LessEqual => rules::LessEqual(state),
            Rule::GraterEqual => rules::GraterEqual(state),
            Rule::Plus => rules::Plus(state),
            Rule::Minus => rules::Minus(state),
            Rule::Power => rules::Power(state),
            Rule::Surd => rules::Surd(state),
            Rule::Increase => rules::Increase(state),
            Rule::Decrease => rules::Decrease(state),
            Rule::To => rules::To(state),
            Rule::Elvis => rules::Elvis(state),
            Rule::LogicOr => rules::LogicOr(state),
            Rule::LogicAnd => rules::LogicAnd(state),
            Rule::LogicNot => rules::LogicNot(state),
            Rule::Ellipsis => rules::Ellipsis(state),
            Rule::Concat => rules::Concat(state),
            Rule::Destruct => rules::Destruct(state),
            Rule::Bang => rules::Bang(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
