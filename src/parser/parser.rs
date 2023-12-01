use std::fmt::Display;

use crate::{parser::{AnyOf6, ParseContext, OneOf, Not, AnyV, Funnel5, Funnel4, AnyMemTable, Funnel6, Join, Mem, Funnel2, AnyOf10, OneOf9, AnyOf9, OneOf8, MapPValue, Funnel9, LRJoin, Trace, Funnel10, Never}, srule};

use super::{ParseResult, Span, ZeroOrMore, ParseNode, SpanOf, Map, OneOf3, Spanned, OneOrMore, AnyOf3, Maybe, AnyOf2, MapV, OneOf6, SRule, Leader, Surround, End, Req, OneOf5, AnyOf5, AnyOf4, OneOf4, OneOf2, ParsePos, ParseStore};

use ParseResult::*;
use unicode_xid::UnicodeXID;

/// 
/// A parse position the parser.
/// 
#[derive(Clone, Copy)]
pub struct PPos {
    /// The byte index in the `str`
    pub index: usize,
    /// The character column (starts at 1).
    pub column: usize,
    /// The character line (starts at 1).
    pub line: usize,
}

impl PPos {
    /// Creates and returns a new `PPos`.
    pub fn new() -> Self {
        Self {
            index: 0,
            column: 1,
            line: 1,
        }
    }

    /// Creates a new `PPos` with the given index.
    pub fn with_index(index: usize) -> Self {
        Self { index, column: 1, line: 1 }
    }
}

impl PartialEq for PPos {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for PPos { }

impl std::fmt::Debug for PPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for PPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.line, f)?;
        f.write_str(":")?;
        Display::fmt(&self.column, f)?;
        f.write_str(":")?;
        Display::fmt(&self.index, f)
    }
}

impl ParsePos for PPos {
    type Key = usize;
    fn key(&self) -> usize {
        self.index
    }
}

impl ParseStore<PPos, char> for str {
    fn value_at(&self, pos: &mut PPos) -> Option<char> {
        if let Some(ch) = self.value_at(&mut pos.index) {
            if ch == '\n' {
                pos.line += 1;
                pos.column = 1;
            } else {
                pos.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }
}

impl ParseStore<PPos, char> for &str {
    fn value_at(&self, pos: &mut PPos) -> Option<char> {
        (**self).value_at(pos)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RArg {
    pub id: Span<PPos>,
    pub ty: RType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RBlock {
    pub statements: Vec<RStatement>
}

/// 
/// An if statement or chain of if statments.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RIf {
    If {
        /// The previous if statement (if there is one) in the chain of if
        /// statements. Only if its expression returns `false` can this if
        /// statement check its own expression.
        prev: Option<Box<RIf>>,
        /// The expression that must resolve to a boolean
        expr: RExpr,
        /// The block of code to run if the expression evaluates to `true`.
        block: RBlock
    },
    Else {
        prev: Box<RIf>,
        block: RBlock,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RType {
    Array {
        /// The type of every item in the array (since arrays can only hold 1
        /// type of item, this is that type).
        item_type: Box<Self>,
        /// The number of items in the array.
        item_number: PPos,
    },
    Tuple {
        /// The types in the tuple, stored in the same order as given.
        types: Vec<Self>,
    },
    Template {
        /// The name of the type.
        name: Span<PPos>,
        /// The arguments for the type.
        args: Vec<Self>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RExpr {
    Lit(RLit),
    Var(Span<PPos>),
    Path(Vec<Span<PPos>>),
    Block(RBlock),
    Add(Box<RExpr>, Span<PPos>, Box<RExpr>),
    Sub(Box<RExpr>, Span<PPos>, Box<RExpr>),
    Div(Box<RExpr>, Span<PPos>, Box<RExpr>),
    Mul(Box<RExpr>, Span<PPos>, Box<RExpr>),
    Pow(Box<RExpr>, Span<PPos>, Box<RExpr>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RStatement {
    /// An expression with an optional semicolon after it (semicolon can only be omitted if it is at the end of a group).
    Expr {
        expr: RExpr,
        semi: Option<Span<PPos>>
    },
    /// A return statement.
    Return {
        return_span: Span<PPos>,
        expr: RExpr,
        semi: Option<Span<PPos>>
    },
    /// A semicolon
    SColon {
        semi: Span<PPos>
    },
    /// An if statement or chain of if statements.
    If {
        stmt: RIf,
    },
    /// Assign a variable a value
    Assign {
        /// The `let` keyword span.
        let_: Span<PPos>,
        /// The variable name.
        ident: Span<PPos>,
        /// The optionally-specified type.
        ty: Option<RType>,
        /// The `=` sign.
        equal_sign: Span<PPos>,
        /// The expression after the `=`.
        equal_value: RExpr,
        /// The optional semicolon after the assignment statement (can only be
        /// omitted if on last line of group).
        semicolon: Option<Span<PPos>>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RReturnType {
    /// Returns the given type.
    Type(Span<PPos>),
    /// Function never returns.
    Never,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RFn {
    pub span: Span<PPos>,
    pub fn_span: Span<PPos>,
    pub id: Span<PPos>,
    pub args: Vec<RArg>,
    pub ret_type: RReturnType,
    pub body: RBlock,
}

// -- Literals --

///
/// A literal that cannot have a sign.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RLit {
    Char(RCharLit),
    String(RStrLit),
    RawString(RRawStrLit),
    Byte(RByteLit),
    ByteString(RByteStrLit),
    RawByteString(RRawByteStrLit),
    Integer(RIntLit),
    Float(RFloatLit),
    Bool(RBoolLit),
}

// --- Crate ---

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RCrate {
    pub utf8bom: Option<Span<PPos>>,
    pub shebang: Option<Span<PPos>>,
    // pub attrs: RInnerAttributes,
    pub items: Vec<RItem>,
}

/// 
/// An item in a crate.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RItem {
//    Mod {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RMod,
//    },
//    ExternCrate {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RExternCrate,
//    },
//    UseDecl {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RUseDecl,
//    },
    Fn {
        span: Span<PPos>,
        vis: Option<RVis>,
        val: RFn,
    },
//    TypeAlias {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RTypeAlias,
//    },
//    Struct {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RStruct,
//    },
//    Enum {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: REnum,
//    },
//    Union {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RUnion,
//    },
//    Const {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RConstItem,
//    },
//    Static {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RStaticItem,
//    },
//    Trait {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RTrait,
//    },
//    Impl {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RImpl,
//    },
//    ExternBlock {
//        span: Span<PPos>,
//        vis: Option<RVis>,
//        val: RExternBlock,
//    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// The visibility of an item.
pub enum RVis {
    VisPub   { span: Span<PPos>, },
    VisCrate { span: Span<PPos>, },
    VisSelf  { span: Span<PPos>, },
    VisSuper { span: Span<PPos>, },
//    VisPath  {
//        span: Span<PPos>,
//        path: RSimplePath,
//    }
}

// --- Literals ---

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RBoolLit {
    True  { span: Span<PPos> },
    False { span: Span<PPos> }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RCharLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

/// 
/// A string where escape sequences are allowed.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RByteStrLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RRawByteStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RByteLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub suffix: Option<Span<PPos>>
}

/// 
/// A string where no escape sequences are allowed i.e. the string is
/// exactly as it looks.
/// 
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RRawStrLit {
    pub span: Span<PPos>,
    pub text: Span<PPos>,
    pub suffix: Option<Span<PPos>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RFloatLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
    pub exp: Option<Span<PPos>>,
    pub value_exp_span: Span<PPos>,
    pub suffix: Option<Span<PPos>>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RSFloatLit {
    pub span: Span<PPos>,
    pub neg: bool,
    pub lit: RFloatLit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RDecLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RBinLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ROctLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RHexLit {
    pub span: Span<PPos>,
    pub value: Span<PPos>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RIntLit {
    DecLit {
        span: Span<PPos>,
        lit: RDecLit,
        suffix: Option<Span<PPos>>,
    },
    BinLit {
        span: Span<PPos>,
        lit: RBinLit,
        suffix: Option<Span<PPos>>,
    },
    OctLit {
        span: Span<PPos>,
        lit: ROctLit,
        suffix: Option<Span<PPos>>,
    },
    HexLit {
        span: Span<PPos>,
        lit: RHexLit,
        suffix: Option<Span<PPos>>,
    },
}

use ParseResult::*;
/// 
/// Parses a file and returns the result.
/// 
pub fn parse_file(file_text: &str) -> ParseResult<RCrate, String, PPos> {
    println!("Parsing: \"{}\"", file_text);

    // create `expr` (it requires a number of recursive child nodes)
    srule!(w, w_rule);
    srule!(alpha, alpha_rule);
    srule!(numeric, numeric_rule);
    srule!(ident, ident_rule);
    srule!(param, param_rule);
    srule!(params, params_rule);
    srule!(return_type, return_type_rule);
    srule!(expr, expr_rule);
    srule!(mul_or_div, mul_or_div_rule);
    srule!(power, power_rule);
    srule!(value, value_rule);
    srule!(block, block_rule);
    srule!(type_tuple, type_tuple_rule);
    srule!(ty, ty_rule);
    srule!(statement, statement_rule);
    srule!(func, func_rule);
    srule!(file, file_rule);
    srule!(item, item_rule);
    srule!(vis, vis_rule);

    srule!(literal_expression, literal_expression_rule);
    srule!(char_literal, char_literal_rule);
    srule!(string_literal, string_literal_rule);
    srule!(raw_string_literal, raw_string_literal_rule);
    srule!(byte_literal, byte_literal_rule);
    srule!(byte_string_literal, byte_string_literal_rule);
    srule!(raw_byte_string_literal, raw_byte_string_literal_rule);
    srule!(integer_literal, integer_literal_rule);
    srule!(float_literal, float_literal_rule);
    srule!(bool_literal, bool_literal_rule);

    srule!(quote_escape, quote_escape_rule);
    srule!(ascii_escape, ascii_escape_rule);
    srule!(unicode_escape, unicode_escape_rule);
    srule!(oct_digit, oct_digit_rule);
    srule!(hex_digit, hex_digit_rule);
    srule!(string_continue, string_continue_rule);
    srule!(suffix, suffix_rule);
    srule!(suffix_no_e, suffix_no_e_rule);
    srule!(raw_string_content, raw_string_content_rule);
    srule!(ascii_for_char, ascii_for_char_rule);
    srule!(byte_escape, byte_escape_rule);
    srule!(ascii_for_string, ascii_for_string_rule);
    srule!(raw_byte_string_content, raw_byte_string_content_rule);
    srule!(ascii, ascii_rule);
    srule!(dec_literal, dec_literal_rule);
    srule!(bin_literal, bin_literal_rule);
    srule!(oct_literal, oct_literal_rule);
    srule!(hex_literal, hex_literal_rule);
    srule!(dec_digit, dec_digit_rule);
    srule!(bin_digit, bin_digit_rule);
    srule!(tuple_index, tuple_index_rule);
    srule!(float_exponent, float_exponent_rule);
    srule!(reserved_number, reserved_number_rule);

    srule!(return_statement, return_statement_rule);
    srule!(let_statement, let_statement_rule);
    srule!(expr_semi_statement, expr_semi_statement_rule);
    srule!(semi_statement, semi_statement_rule);

    srule!(if_statement, if_statement_rule);

    // trace levels
    const RULE_NAMES: i32 = 1;

    // define function to produce "panic" uniform messages of parse
    let panic_fn = |pos: Span<PPos>, fn_name: &str, message: &str| -> String {
        format!("{}: ({}) {}", pos, fn_name, message)
    };
    let panic = &panic_fn;

    // IDENTIFIERS

    let isolated_cr = SpanOf(('\r', Not('\n')));
    let isolated_cr = &isolated_cr;

    // unicode groups
    let xid_start = MapPValue(|span, ch| {
        if UnicodeXID::is_xid_start(ch) {
            Okay(span.clone(), span.end)
        } else {
            Error(panic(span, "xid_start", "expected character in the [:XID_Start:] unicode group"))
        }
    });
    let xid_start = &xid_start;

//    let xid_continue = MapPValue(|span, ch| {
//        if UnicodeXID::is_xid_continue(ch) {
//            Okay(span.clone(), span.end)
//        } else {
//            Error(panic(span, "xid_continue", "expected character in the [:XID_Continue:] unicode group"))
//        }
//    });
//    let xid_continue = &xid_continue;

    // - SUFFIX -

    suffix_rule.set(ident);

    suffix_no_e_rule.set(SpanOf((Not(OneOf(['e', 'E'])), suffix)));

    // --- whitespace ---

    // a rule that just consumes whitespace space
    w_rule.set(SpanOf(ZeroOrMore(OneOf2(..=32u32, 127u32))));

    // a rule to parse an ascii letter (lower case or upper case)
    alpha_rule.set(SpanOf(OneOf2(97..=122, 65..=90)));
    // a rule to parse an ascii numeric (0,1,2,etc.)
    numeric_rule.set(SpanOf(48..=57));
    // a rule to parse an identifier
    ident_rule.set(SpanOf(
        (
            OneOf2(alpha.clone(), '_'),
            ZeroOrMore(OneOf3(alpha.clone(), numeric.clone(), '_'))
        )
    ));

    return_type_rule.set(Map(
        Leader("->", (w, OneOf2(ident, '!')), |_, arrow_span, _| panic(arrow_span, "return_type", "missing return type")),
        |res| {
            res.map_value(|(_arrow, (_, any_of_two))| {
                match any_of_two {
                    AnyOf2::Child1(id) => RReturnType::Type(id),
                    AnyOf2::Child2(_) => RReturnType::Never,
                }
            })
        }
    ));

    // --- Literals ---

    literal_expression_rule.set(
        Funnel9(
            MapV(char_literal,            |c| RLit::Char(c)),
            MapV(string_literal,          |s| RLit::String(s)),
            MapV(raw_string_literal,      |r| RLit::RawString(r)),
            MapV(byte_literal,            |b| RLit::Byte(b)),
            MapV(byte_string_literal,     |b| RLit::ByteString(b)),
            MapV(raw_byte_string_literal, |r| RLit::RawByteString(r)),
            MapV(integer_literal,         |i| RLit::Integer(i)),
            MapV(float_literal,           |f| RLit::Float(f)),
            MapV(bool_literal,            |b| RLit::Bool(b)),
        )
    );

    // - Char Literal -

    char_literal_rule.set(MapV(
        Spanned((
            '\'',
            SpanOf(OneOf4(
                (Not(OneOf(['\'', '\\', '\n', '\r', '\t'])), AnyV()),
                quote_escape,
                ascii_escape,
                unicode_escape
            )),
            '\'',
            Maybe(suffix)
        )),
        |(span, (_, value, _, suffix))| {
            RCharLit { span, value, suffix }
        }
    ));

    quote_escape_rule.set(SpanOf(OneOf(["\\'", "\\\""])));

    ascii_escape_rule.set(
        SpanOf(OneOf6(
            ("\\x", oct_digit, hex_digit),
            "\\n",
            "\\r",
            "\\t",
            "\\\\",
            "\\0"
        ))
    );

    let _d = (hex_digit, ZeroOrMore('_'));
    let _d = &_d;
    unicode_escape_rule.set(
        ("\\u{",
            _d, Maybe(_d), Maybe(_d), Maybe(_d), Maybe(_d), Maybe(_d),
        '}')
    );

    // - STRING LITERAL -

    string_literal_rule.set(MapV(Spanned((
            '"',
            SpanOf(ZeroOrMore(OneOf5(
                (Not(OneOf2('"', '\\')), AnyV()),
                quote_escape,
                ascii_escape,
                unicode_escape,
                string_continue
            ))),
            '"',
            Maybe(suffix)
        )),
        |(span, (_, text, _, suffix))| {
            match suffix {
                Some(suffix) => {
                    RStrLit {
                        span,
                        text,
                        suffix: Some(suffix)
                    }
                },
                None => {
                    RStrLit {
                        span,
                        text,
                        suffix: None
                    }
                }
            }
        }
    ));

    string_continue_rule.set("\\\n");

    // - RAW STRING LITERALS -

    raw_string_literal_rule.set(MapV(
        Spanned(('r', raw_string_content, Maybe(suffix))),
        |(span, (_, text, suffix))| {
            match suffix {
                Some(suffix) => { RRawStrLit { span, text, suffix: Some(suffix) } },
                None => { RRawStrLit { span, text, suffix: None } },
            }
        }
    ));

    raw_string_content_rule.set(MapV(
        OneOf2(
            ('"', SpanOf(ZeroOrMore((Not(isolated_cr), Not('"'), AnyV()))), '"'),
            ('#', raw_string_content, '#')
        ),
        |two| {
            use AnyOf2::*;
            match two {
                Child1((_, text, _)) => text,
                Child2((_, text, _)) => text 
            }
        }
    ));

    // - BYTE LITERAL -

    byte_literal_rule.set(MapV(
        Spanned(('b', ',', SpanOf(OneOf2(ascii_for_char, byte_escape)), '\'', Maybe(suffix))),
        |(span, (_, _, value, _, suffix))| { RByteLit { span, value, suffix } }
    ));

    ascii_for_char_rule.set((Not(OneOf(['\\', '\n', '\r', '\t'])), 0x00..=0x7F));

    byte_escape_rule.set(
        OneOf8(
            ("\\x", hex_digit, hex_digit),
            "\\n",
            "\\r",
            "\\t",
            "\\\\",
            "\\0",
            "\\'",
            "\\\"",
        )
    );

    // - BYTE STRING LITERALS -

    byte_string_literal_rule.set(MapV(
        Spanned(("b\"", SpanOf(ZeroOrMore((ascii_for_string, byte_escape, string_continue))), '"', Maybe(suffix))),
        |(span, (_, value, _, suffix))| { RByteStrLit { span, value, suffix } }
    ));

    ascii_for_string_rule.set(
        (Not(OneOf3('"', '\\', isolated_cr)), 0x00..=0x7F)
    );

    // - RAW BYTE STRING LITERAL -

    raw_byte_string_literal_rule.set(MapV(
        Spanned(("br", raw_byte_string_content, Maybe(suffix))),
        |(span, (_, text, suffix))| {
            RRawByteStrLit { span, text, suffix }
        }
    ));

    raw_byte_string_content_rule.set(MapV(
        OneOf2(
            ('"', SpanOf(ZeroOrMore((Not('"'), ascii))), '"'),
            ('#', raw_byte_string_content, '#')
        ),
        |any_of_two| {
            use AnyOf2::*;
            match any_of_two {
                Child1((_, span, _)) => span,
                Child2((_, span, _)) => span,
            }
        }
    ));

    ascii_rule.set(0x00..=0x7F);

    // - INTEGER LITERALS -

    integer_literal_rule.set(MapV(Spanned((
            OneOf4(
                dec_literal,
                bin_literal,
                oct_literal,
                hex_literal
            ),
            Maybe(suffix_no_e)
        )),
        |(span, (any_of_four, suffix))| {
            use AnyOf4::*;
            match any_of_four {
                Child1(dec) => RIntLit::DecLit { span, lit: dec, suffix },
                Child2(bin) => RIntLit::BinLit { span, lit: bin, suffix },
                Child3(oct) => RIntLit::OctLit { span, lit: oct, suffix },
                Child4(hex) => RIntLit::HexLit { span, lit: hex, suffix },
            }
        }
    ));

    dec_literal_rule.set(MapV(
        SpanOf((
            dec_digit,
            ZeroOrMore(OneOf2(
                dec_digit,
                '_'
            ))
        )),
        |span| { RDecLit { span: span.clone(), value: span } }
    ));

    bin_literal_rule.set(MapV(
        Spanned(("0b", SpanOf((ZeroOrMore(OneOf2(bin_digit, '_')), bin_digit, ZeroOrMore(OneOf2(bin_digit, '_')))))),
        |(span, (_, value))| { RBinLit { span, value } }
    ));

    oct_literal_rule.set(MapV(
        Spanned(("0o", SpanOf((ZeroOrMore(OneOf2(oct_digit, '_')), oct_digit, ZeroOrMore(OneOf2(oct_digit, '_')))))),
        |(span, (_, value))| { ROctLit { span, value } }
    ));

    hex_literal_rule.set(MapV(
        Spanned(("0x", SpanOf((ZeroOrMore(OneOf2(hex_digit, '_')), hex_digit, ZeroOrMore(OneOf2(hex_digit, '_')))))),
        |(span, (_, value))| { RHexLit { span, value } }
    ));

    bin_digit_rule.set('0'..='1');

    oct_digit_rule.set('0'..='7');

    dec_digit_rule.set('0'..='9');

    hex_digit_rule.set(SpanOf(OneOf3('0'..='9', 'a'..='f', 'A'..='F')));

    tuple_index_rule.set(integer_literal);

    // - FLOAT LITERALS -

    float_literal_rule.set(MapV(
        Spanned(OneOf3(
            SpanOf((dec_literal, '.', Not(OneOf3('.', '_', xid_start)))),
            (SpanOf((dec_literal, '.', dec_literal)), Maybe(suffix_no_e)),
            (Spanned((SpanOf((dec_literal, Maybe(('.', dec_literal)))), float_exponent)), Maybe(suffix)),
        )),
        |(span, three)| {
            use AnyOf3::*;
            match three {
                Child1(value) => {
                    RFloatLit { span: span.clone(), value, exp: None, value_exp_span: span, suffix: None }
                },
                Child2((value, suffix)) => {
                    RFloatLit { span: span.clone(), value, exp: None, value_exp_span: span, suffix }
                },
                Child3(((value_exp_span, (value, exp)), suffix)) => {
                    RFloatLit { span, value, exp: Some(exp), value_exp_span, suffix }
                }
            }
        }
    ));

    float_exponent_rule.set(SpanOf((
        OneOf(['e', 'E']),
        Maybe(OneOf(['+', '-'])),
        ZeroOrMore(OneOf2(dec_digit, '_')),
        dec_digit,
        ZeroOrMore(OneOf2(dec_digit, '_'))
    )));

    reserved_number_rule.set(Map(
        Spanned(OneOf8(
            (bin_literal, '2'..='9'),
            (oct_literal, '8'..='9'),
            (
                OneOf3(
                    bin_literal,
                    oct_literal,
                    hex_literal
                ),
                '.',
                Not(OneOf3('.', '_', xid_start))
            ),
            (
                OneOf2(bin_literal, oct_literal),
                OneOf(['e', 'E'])
            ),
            (
                "0b",
                ZeroOrMore('_'),
                OneOf2(
                    End(),
                    (Not(bin_digit), AnyV())
                )
            ),
            (
                "0o",
                ZeroOrMore('_'),
                OneOf2(
                    End(),
                    (Not(oct_digit), AnyV())
                )
            ),
            (
                "0x",
                ZeroOrMore('_'),
                OneOf2(
                    End(),
                    (Not(hex_digit), AnyV())
                )
            ),
            (
                dec_literal,
                Maybe(('.', dec_literal)),
                OneOf2('e', 'E'),
                Maybe(OneOf2('+', '-')),
                OneOf2(
                    End(),
                    (Not(dec_digit), AnyV())
                )
            )
        )),
        |res| {
            (match res {
                Okay((span, _), _) => Panic(panic(span, "reserved_number", "this number is reserved and cannot be used")),
                res => res
            }).map_value(|_|())
        }
    ));

    // - BOOLEAN LITERAL -

    bool_literal_rule.set(Funnel2(
        MapV("true" , |span| RBoolLit::True  { span }),
        MapV("false", |span| RBoolLit::False { span }),
    ));

    // --- Expressions ---

    {
        expr_rule.set(
            LRJoin(mul_or_div, (w, OneOf2('+', '-'), w),
            |left, (_, op, _), right| {
                match op {
                    AnyOf2::Child1(span) => RExpr::Add(Box::new(left), span, Box::new(right)),
                    AnyOf2::Child2(span) => RExpr::Sub(Box::new(left), span, Box::new(right)),
                }
            })
        );

        mul_or_div_rule.set(
            LRJoin(power, (w, OneOf2('*', '/'), w),
            |left, (_, op, _), right| {
                match op {
                    AnyOf2::Child1(span) => RExpr::Mul(Box::new(left), span, Box::new(right)),
                    AnyOf2::Child2(span) => RExpr::Div(Box::new(left), span, Box::new(right)),
                }
            })
        );

        power_rule.set(
            LRJoin(value, (w, '^', w),
            |left, (_, op, _), right| {
                RExpr::Pow(Box::new(left), op, Box::new(right))
            })
        );

        value_rule.set(
            Funnel4(
                MapV(ident, |span| RExpr::Var(span)),
                MapV(literal_expression, |lit| RExpr::Lit(lit)),
                MapV(('(', w, expr, w, ')'), |(_, _, e, _, _)| e),
                MapV(block, |group| RExpr::Block(group))
            ),
        );
    }

    block_rule.set(
        MapV(
            Surround(
                    '{',
                        (w, ZeroOrMore((statement, w)), Maybe((expr, w))),
                    '}',
                    |_, _, e| e,
                    |_, ocbrace_span, _, _| panic(ocbrace_span, "block", "openning curly brace is missing its complementary closing curly brace to end the scope"),
                ),
            |(_lcbrace, (_, statements, expr), _rcbrace)| {
                RBlock {
                    statements: {
                        let mut statements: Vec<RStatement> = statements.into_iter().map(|(v,_)|v).collect();
                        if let Some(expr) = expr.map(|(e, _)| e) {
                            statements.push(RStatement::Expr { expr, semi: None });
                        }
                        statements
                    },
                }
            }
        )
    );

    type_tuple_rule.set(
        MapV(
            Surround(
                '(',
                    (
                        w,
                        Maybe(
                        (ty, w, ZeroOrMore((',', w, ty, w)))
                        )
                    ),
                ')',
                |_, _, e| e,
                |_, oparen_span, _, _| panic(oparen_span, "type_tuple", "missing closing parenthesis after this open parenthesis"),
            ),
            |(_, (_, maybe_types), _)| {
                match maybe_types {
                    Some((t1, _, types)) => {
                        let mut types: Vec<RType> = types.into_iter().map(|v|v.2).collect();
                        types.insert(0, t1);
                        RType::Tuple { types }
                    },
                    None => RType::Tuple { types: Vec::new() },
                }
            }
        )
    );

    ty_rule.set(
        MapV(
            OneOf2(
                type_tuple,
                (
                    ident,
                    Maybe((
                        w,
                        Surround(
                            '<',
                            ( 
                                w,
                                ZeroOrMore((ty, w))
                            ),
                            '>',
                        |_, oarrow_span, _| panic(oarrow_span, "ty", "expected values within this type bounds"),
                            |_, oarrow_span, _, _| panic(oarrow_span, "ty", "expected closing arrow ('>') after this openning arrow ('<')"),
                        )
                    ))
                )
            ),
            |any_of_two| {
                match any_of_two {
                    AnyOf2::Child1(ty) => {
                        ty
                    },
                    AnyOf2::Child2((ident, args)) => {
                        RType::Template {
                            name: ident,
                            args: match args {
                                Some((_, (_, (_, args), _))) => {
                                    args.into_iter().map(|v|v.0).collect()
                                },
                                None => Vec::new(),
                            },
                        }
                    }
                }
            }
        )
    );

    // --- STATEMENTS ---

    statement_rule.set(Funnel5(
            MapV(if_statement, |stmt| RStatement::If { stmt }),
            return_statement,
            let_statement,
            expr_semi_statement,
            semi_statement,
        ),
    );

    return_statement_rule.set(
        MapV(
            Leader(
                "return",
                (w, expr, Maybe((w, ';'))),
                |_, return_span, _| panic(return_span, "statement", "expected expression after this \"return\" keyword")
            ),
            |(return_span, (_, expr, maybe_semi))| {
                RStatement::Return {
                    return_span,
                    expr,
                    semi: match maybe_semi { Some(v) => Some(v.1), None => None }
                }
            }
        ),
    );

    let_statement_rule.set(
        MapV(
            Leader(
                "let",
                (w, ident, Maybe((w, ':', w, ty)), w, '=', w, expr, Maybe((w, ';'))),
                |_, let_span, _| panic(let_span, "statement", "expected_variable assignment after this let statement")
            ),
            |(let_span, (_, ident, maybe_type, _, eq_span, _, expr, maybe_semi))| {
                RStatement::Assign {
                    let_: let_span,
                    ident,
                    ty: match maybe_type {
                        Some((_, _colon_span, _, ty)) => Some(ty),
                        None => None,
                    },
                    equal_sign: eq_span,
                    equal_value: expr,
                    semicolon: match maybe_semi {
                        Some((_, span)) => Some(span),
                        None => None,
                    },
                }
            }
        ),
    );

    expr_semi_statement_rule.set(MapV(
        (expr, Maybe((w, ';'))),
        |(expr, maybe_semi)| {
            RStatement::Expr { expr, semi: maybe_semi.map(|(_, s)|s) }
        }
    ));

    semi_statement_rule.set(MapV(
        ";",
        |semi| {
            RStatement::SColon { semi }
        }),
    );

    if_statement_rule.set(MapV(Leader(
            "if", (w, expr, w, block,
                Maybe((w, Leader("else", (w, OneOf2(block, if_statement)),
                    |_, pos, _| panic(pos, "if_statement", "expected code block after this \"else\"")
                )))
            ),
            |_, pos, _| panic(pos, "if_statement", "expected expression and body of the if statement after this \"if\" keyword")
        ),
        |(_, (_, expr, _, block, maybe_else))| {
            let iff = RIf::If { prev: None, expr, block };

            if let Some((_, (_, (_, choice)))) = maybe_else {
                use AnyOf2::*;
                match choice {
                    Child1(block) => {
                        RIf::Else {
                            prev: Box::new(iff),
                            block,
                        }
                    },
                    Child2(if_stmt) => {
                        match if_stmt {
                            RIf::If { prev: _, expr, block } => {
                                RIf::If { prev: Some(Box::new(iff)), expr, block }
                            },
                            RIf::Else { prev: _, block } => { 
                                RIf::Else { prev: Box::new(iff), block }
                             }
                        }
                    },
                }
            } else {
                iff
            }
        }
    ));

    // --- Function ---

    // the rule to parse a function
    func_rule.set(
        MapV(
            Spanned(Leader(
                "fn",
                (w, Leader(ident,
                        (w,
                            Surround(
                                '(', (w, params, w), ')',
                                |_, oparen_span, _| panic(oparen_span, "func", "expected parameters in this function argument scope"),
                                |_, oparen_span, _, _| panic(oparen_span, "func", "expected closing parenthesis to match this open parenthesis")
                            ),
                            Req((w, return_type), |_, p, _| panic(Span::new(p.clone(), p), "func", "function requires a return type")),
                            Req((w, block), |_, p, _| panic(Span::new(p.clone(), p), "func", "function requires a function body"))
                        ),
                        |_, ident_span, _| panic(ident_span, "func", "expected function parameters and body after function identifier")
                    )
                ),
                |_, fn_span, _| panic(fn_span, "func", "expected correct function syntax after 'fn' keyword"))),
            |(span, (fn_span, (_, (id_span, (_, (_oparen, (_, params, _), _cparen), (_, ret_type), (_, body))))))| {
                RFn {
                    span,
                    fn_span,
                    id: id_span,
                    args: params,
                    ret_type,
                    body,
                }
            }
        )
    );

    // function parameter
    param_rule.set(MapV(
        Leader(
            ident,
            (w, Leader(
                    ':', (w, ty),
                    |_, colon_span, _| panic(colon_span, "param", "missing type after this colon"),
                )
            ),
            |_, id_span, _| panic(id_span, "param", "missing arg's type"),
        ),
        |(id, (_, (_colon, (_, ty))))| RArg { id, ty, }
    ));

    // a vector of function parameters
    params_rule.set(MapV(
        (Join(param, (w, ',', w)), Maybe((w, ','))),
        |(params, _)| params
    ));

    vis_rule.set(
        Funnel4(
            MapV(SpanOf(("pub", w, '(', w, "crate", w, ')')), |span| RVis::VisCrate { span }),
            MapV(SpanOf(("pub", w, '(', w, "self" , w, ')')), |span| RVis::VisSelf  { span }),
            MapV(SpanOf(("pub", w, '(', w, "super", w, ')')), |span| RVis::VisSuper { span }),
            MapV("pub", |span| RVis::VisPub { span }),
            //MapV(("pub", w, '(', w, "in", w, simple_path, w, ')'), |(_, _, _, _, _, _, path, _, _)| RVis::VisPath { span, path })
        ),
    );

    item_rule.set(
        MapV(Spanned((Maybe(vis), w, func)), |(span, (vis, _, val))| RItem::Fn { span, vis, val })
    );


    // the rule to parse a `File`

    file_rule.set(MapV((
            w,
            Maybe(("\\uFEFF", w)),
            Maybe((SpanOf(("#!", OneOrMore((Not('\n'), AnyV())))), w)),
            //ZeroOrMore((inner_attribute, w)),
            ZeroOrMore((item, w)),
            Req(End(), |_, pos: PPos, _| panic(Span::new(pos.clone(), pos.clone()), "file", "parser failed to reach the end of the file (from this pos)"))
        ),
        |(_, utf8bom, shebang, items, _)| {
            RCrate {
                utf8bom: utf8bom.map(|(s, _)|s),
                shebang: shebang.map(|(s, _)|s),
                items: items.into_iter().map(|(i, _)|i).collect()
            }
        }
    ));

    file.parse(ParseContext::new(&AnyMemTable::new(file_text), PPos::new(), RULE_NAMES))
}


#[cfg(test)]
mod tests {
    use crate::parser::parser::{RCrate, RFn};

    use super::parse_file;
    use super::super::ParseResult;
    use ParseResult::*;
    use super::PPos;

    #[test]
    fn test_stmt_semi_end_fn() {
        match parse_file(" fn a(x:i32, y:i32) -> u128 { 
            let u = 6; 
            8 / 4; 
            5 - 2; } ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_stmt_fn() {
        match parse_file(" fn hello() -> u8 {  (10 + 3) * 3 / 4 ^ (10); 10; 23; 0 } ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_expr_fn() {
        match parse_file(" fn hello() -> u8 {  (10 + 3) * 3 / 4 ^ (10) } ") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_add_fn() {
        match parse_file(" fn hello() -> u8 {10 + 3}") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_empty_fn() {
        match parse_file(" fn hello() -> u8 {0}") {
            Okay(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_empty() {
        match parse_file("") {
            Okay(value, advance) => {
                println!("{}: {:?}", advance, value);
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }
}