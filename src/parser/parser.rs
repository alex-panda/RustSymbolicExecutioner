use super::{AnyOfSix, ParseResult, Span, ZeroOrMore, ParseNode, OneOfTwo, SpanOf, Map, OneOfThree, Spanned, OneOrMore, AnyOfThree, Maybe, AnyOfTwo, MapV, OneOfSix, OneOfFour, AnyOfFour, Dyn, SRule};


#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub id: Span<usize>,
    pub ty: Span<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    statements: Vec<Statement>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    /// The name of the type.
    name: Span<usize>,
    /// The arguments for the type.
    args: Vec<Self>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(Integer),
    Float(Float),
    Group(Group),
    Add(Box<Expr>, Span<usize>, Box<Expr>),
    Sub(Box<Expr>, Span<usize>, Box<Expr>),
    Div(Box<Expr>, Span<usize>, Box<Expr>),
    Mul(Box<Expr>, Span<usize>, Box<Expr>),
    Pow(Box<Expr>, Span<usize>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// An expression with an optional semicolon after it (semicolon can only be omitted if it is at the end of a group).
    Expr {
        expr: Expr,
        semi: Option<Span<usize>>
    },
    /// A return statement.
    Return {
        return_span: Span<usize>,
        expr: Expr,
        semi: Option<Span<usize>>
    },
    /// A semicolon
    SColon {
        semi: Span<usize>
    },
    /// Assign a variable a value
    Assign {
        /// The `let` keyword span.
        let_: Span<usize>,
        /// The variable name.
        ident: Span<usize>,
        /// The optionally-specified type.
        ty: Option<Type>,
        /// The `=` sign.
        equal_sign: Span<usize>,
        /// The expression after the `=`.
        equal_value: Expr,
        /// The optional semicolon after the assignment statement (can only be
        /// omitted if on last line of group).
        semicolon: Option<Span<usize>>,
    },
}

#[derive(Debug, Eq, Clone)]
pub enum FloatType {
    F32(Span<usize>),
    F64(Span<usize>),
}

impl PartialEq for FloatType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::F32(_), Self::F32(_)) => true,
            (Self::F64(_), Self::F64(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UntType {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}

#[derive(Debug, Eq, Clone)]
pub enum IntegerType {
    /// Integer was specified to have a signed type.
    Signed(Span<usize>, IntType),
    /// Integer was specified to have an unsigned type
    Unsigned(Span<usize>, UntType),
}

impl PartialEq for IntegerType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Signed(_, l1), Self::Signed(_, r1)) => l1 == r1,
            (Self::Unsigned(_, l1), Self::Unsigned(_, r1)) => l1 == r1,
            _ => false
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    /// The span of the entire integer (including its sign and type (if either are given)).
    pub span: Span<usize>,
    /// The sign of the integer.
    pub sign: Sign,
    /// the span of the sign (if directly given).
    pub sign_span: Option<Span<usize>>,
    /// The numerics of the integer.
    pub numerics: Span<usize>,
    /// The type of the integer (if given).
    pub ty: Option<IntegerType>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    /// The span of the entire float.
    pub span: Span<usize>,
    /// The number of the float.
    pub numerics: Span<usize>,
    /// The span of the sign of the float (if specified directly).
    pub sign_span: Option<Span<usize>>,
    /// The sign of the float.
    pub sign: Sign,
    /// The type of the float (if given).
    pub ty: Option<FloatType>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnType {
    /// Returns the given type.
    Type(Span<usize>),
    /// Function never returns.
    Never,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub span: Span<usize>,
    pub fn_span: Span<usize>,
    pub id: Span<usize>,
    pub args: Vec<Arg>,
    pub body: Group,
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub funcs: Vec<Func>,
}

use ParseResult::*;

pub fn parse_file(file: &str) -> ParseResult<File, String, usize> {
    // a rule that just consumes whitespace space
    let w = SpanOf(ZeroOrMore(OneOfTwo(..=32u32, 127u32)));

    // a rule to parse an ascii letter (lower case or upper case)
    let alpha = SpanOf(OneOfTwo(97..=122, 65..=90));
    // a rule to parse an ascii numeric (0,1,2,etc.)
    let numeric = SpanOf(48..=57);
    // a rule to parse an identifier
    let ident = SpanOf(
        (
            OneOfTwo(alpha.clone(), '_'),
            ZeroOrMore(OneOfThree(alpha.clone(), numeric.clone(), '_'))
        )
    );

    let param = Map(
        (ident.clone(), w.clone(), ':', w.clone(), ident.clone()),
        |res| {
            res.map_value(|(id, _, _colon, _, ty)| {
                Arg { id, ty }
            })
        }
    );
    let params = Map(
        OneOfThree(
            w.clone(),
            (param.clone(), w.clone()),
            (param.clone(), OneOrMore((w.clone(), ',', w.clone(), param.clone()))),
        ),
        |res| {
            res.map_value(|any_of_three| {
                match any_of_three {
                    AnyOfThree::Child1(_) => Vec::<Arg>::new(),
                    AnyOfThree::Child2((params, _)) => Vec::from([params]),
                    AnyOfThree::Child3((param1, other_params)) => {
                        let mut accume = Vec::with_capacity(1 + other_params.len());
                        accume.push(param1);
                        for (_, _comma, _, param) in other_params {
                            accume.push(param);
                        }
                        accume
                    },
                }
            })
        }
    );

    let return_type = Map(
        ("->", w.clone(), OneOfTwo(ident.clone(), '!')),
        |res| {
            res.map_value(|(_arrow, _, any_of_two)| {
                match any_of_two {
                    AnyOfTwo::Child1(id) => ReturnType::Type(id),
                    AnyOfTwo::Child2(_) => ReturnType::Never,
                }
            })
        }
    );

    let unt_type = 
        MapV(
            OneOfSix(
                "u8",
                "u16",
                "u32",
                "u64",
                "u128",
                "usize",
            ),
            |res| {
                use AnyOfSix::*;
                match res {
                    Child1(span) => IntegerType::Unsigned(span, UntType::U8),
                    Child2(span) => IntegerType::Unsigned(span, UntType::U16),
                    Child3(span) => IntegerType::Unsigned(span, UntType::U32),
                    Child4(span) => IntegerType::Unsigned(span, UntType::U64),
                    Child5(span) => IntegerType::Unsigned(span, UntType::U128),
                    Child6(span) => IntegerType::Unsigned(span, UntType::USize),
                }
            }
        );

    let int_type = 
        MapV(
            OneOfSix(
                "i8",
                "i16",
                "i32",
                "i64",
                "i128",
                "isize",
            ),
            |res| {
                use AnyOfSix::*;
                match res {
                    Child1(span) => IntegerType::Signed(span, IntType::I8),
                    Child2(span) => IntegerType::Signed(span, IntType::I16),
                    Child3(span) => IntegerType::Signed(span, IntType::I32),
                    Child4(span) => IntegerType::Signed(span, IntType::I64),
                    Child5(span) => IntegerType::Signed(span, IntType::I128),
                    Child6(span) => IntegerType::Signed(span, IntType::ISize),
                }
            }
        );

    let integer = MapV(
        Spanned((
            Maybe(OneOfTwo('+', '-')),
            SpanOf(OneOrMore(numeric.clone())),
            Maybe(OneOfTwo(
                int_type.clone(),
                unt_type.clone(),
            ))
        )),
        |(span, (sign, numerics, t))| {
            let (sign_span, sign) = match sign.clone() {
                Some(v) => match v {
                    AnyOfTwo::Child1(span) => (Some(span), Sign::Positive),
                    AnyOfTwo::Child2(span) => (Some(span), Sign::Negative),
                },
                None => (None, Sign::Positive)
            };

            Integer {
                span,
                sign,
                sign_span,
                numerics,
                ty: match t {
                    Some(value) => Some(match value {
                        AnyOfTwo::Child1(ty) => ty,
                        AnyOfTwo::Child2(ty) => ty,
                    }), 
                    None => None
                },
            }
        }
    );

    let float = MapV(
        Spanned((Maybe(OneOfTwo('-', '+')), SpanOf((OneOrMore(numeric.clone()), '.', OneOrMore(numeric.clone()))), Maybe(OneOfTwo("f32", "f64")))),
        |(span, (sign, numerics, ty))| {
            let (sign_span, sign) = match sign {
                Some(v) => match v {
                    AnyOfTwo::Child1(span) => (Some(span), Sign::Negative),
                    AnyOfTwo::Child2(span) => (Some(span), Sign::Positive),
                },
                None => (None, Sign::Positive),
            };

            let ty = match ty {
                Some(v) => Some(match v {
                    AnyOfTwo::Child1(span) => FloatType::F32(span),
                    AnyOfTwo::Child2(span) => FloatType::F64(span),
                }),
                None => None
            };

            Float {
                span,
                numerics,
                sign_span,
                sign,
                ty,
            }
        }
    );


    // create `expr` (it requires a number of recursive child nodes)
    let group = SRule();
    let expr = SRule();
    let value = SRule();
    let power = SRule();
    let mul_or_div = SRule();
    let add_or_sub = SRule();
    let ty = SRule();
    let statement = SRule();

    {
        expr.set(
            MapV(
                (&mul_or_div, &w, OneOfTwo('+', '-'), &w, Dyn(&expr)),
                |(value, _, add_or_sub, _, expr)| {
                    match add_or_sub {
                        AnyOfTwo::Child1(span) => Expr::Add(Box::new(value), span, Box::new(expr)),
                        AnyOfTwo::Child2(span) => Expr::Sub(Box::new(value), span, Box::new(expr)),
                    }
                }
            )
        );

        add_or_sub.set(
            MapV(
                (&mul_or_div, &w, OneOfTwo('+', '-'), &w, Dyn(&add_or_sub)),
                |(value, _, mul_or_div, _, expr)| {
                    match mul_or_div {
                        AnyOfTwo::Child1(span) => Expr::Mul(Box::new(value), span, Box::new(expr)),
                        AnyOfTwo::Child2(span) => Expr::Div(Box::new(value), span, Box::new(expr)),
                    }
                }
            )
        );

        mul_or_div.set(
            MapV(
                (&power, &w, OneOfTwo('*', '/'), &w, Dyn(&mul_or_div)),
                |(value, _, mul_or_div, _, expr)| {
                    match mul_or_div {
                        AnyOfTwo::Child1(span) => Expr::Mul(Box::new(value), span, Box::new(expr)),
                        AnyOfTwo::Child2(span) => Expr::Div(Box::new(value), span, Box::new(expr)),
                    }
                }
            )
        );

        power.set(
            MapV(
                (&value, &w, '^', &w, Dyn(&power)),
                |(value, _, pow, _, expr)| {
                Expr::Pow(Box::new(value), pow, Box::new(expr))
            })
        );

        value.set(
            MapV(
                OneOfFour(
                    &integer,
                    &float,
                    ('(', &w, Dyn(&expr), &w, ')'),
                    &group
                ),
                |res| {
                    match res {
                        AnyOfFour::Child1(int) => Expr::Int(int),
                        AnyOfFour::Child2(float) => Expr::Float(float),
                        AnyOfFour::Child3((_oparen, _, expr, _, _cparen)) => expr,
                        AnyOfFour::Child4(group) => Expr::Group(group),
                    }
                }
            )
        );
    }

    group.set(
        MapV(
            ('{', &w, ZeroOrMore((Dyn(&statement), &w)), '}'),
            |(_lcbrace, _, statements, _rcbrace)| {
                Group {
                    statements: statements.into_iter().map(|v|v.0).collect(),
                }
            }
        )
    );

    ty.set(
        MapV(
            (
                &ident,
                Maybe((
                    &w,
                    '<',
                    &w,
                    ZeroOrMore((Dyn(&ty), &w)),
                    '>'
                ))
            ),
            |(ident, args)| {
                Type {
                    name: ident,
                    args: match args {
                        Some((_, _, _, args, _)) => {
                            args.into_iter().map(|v|v.0).collect()
                        },
                        None => Vec::new(),
                    },
                }
            }
        )
    );

    statement.set(
        MapV(
            OneOfFour(
                ("return", &w, &expr, Maybe((&w, ';'))),
                ("let", &w, &ident, Maybe((&w, ':', &w, &ty)), &w, '=', &w, &expr, Maybe((&w, ';'))),
                (&expr, Maybe((&w, ';'))),
                ";",
            ),
            |any_of_four| {
                match any_of_four {
                    // return
                    AnyOfFour::Child1((return_span, _, expr, maybe_semi)) => {
                        Statement::Return {
                            return_span,
                            expr,
                            semi: match maybe_semi { Some(v) => Some(v.1), None => None }
                        }
                    },
                    // let
                    AnyOfFour::Child2((let_span, _, ident, maybe_type, _, eq_span, _, expr, maybe_semi)) => {
                        Statement::Assign {
                            let_: let_span,
                            ident,
                            ty: match maybe_type {
                                Some((_, colon_span, _, ty)) => Some(ty),
                                None => None,
                            },
                            equal_sign: eq_span,
                            equal_value: expr,
                            semicolon: match maybe_semi {
                                Some((_, span)) => Some(span),
                                None => None,
                            },
                        }
                    },
                    // expr
                    AnyOfFour::Child3((expr, maybe_semi)) => {
                        Statement::Expr {
                            expr,
                            semi: match maybe_semi {
                                Some((_, semi)) => Some(semi),
                                None => None
                            }
                        }
                    },
                    // semi
                    AnyOfFour::Child4(semi_span) => {
                        Statement::SColon { semi: semi_span }
                    },
                }
            }
        )
    );

    // the rule to parse a `Func`
    let func_rule = 
        MapV(
            Spanned(("fn", &w, &ident, &w, '(', &w, &params, &w, ')', &w, &return_type, &w, '{', &w, OneOrMore(&statement), &w, '}')),
            |(span, (f, _, ident, _, oparen, _, params, _, cparen, _, ret_type, _, ocbrace, _, statements, _, ccbrace))| {
                Func {
                    span,
                    fn_span: f,
                    id: ident,
                    args: Vec::new(),
                    body: Group { statements },
                }
            }
        );

    // the rule to parse a `File`
    let file_rule = 
        Map(
            ZeroOrMore((&w, &func_rule, &w)),
            |res: ParseResult<Vec<(Span<usize>, Func, Span<usize>)>, String, usize>| {
                res.map_value(|v| {
                    File {
                        funcs: v.into_iter().map(|v|v.1).collect(),
                    }
                })
            }
        );

    file_rule.parse(&file, 0usize)
}


#[cfg(test)]
mod tests {
    use crate::parser::parser::{File, Func};

    use super::parse_file;
    use super::super::ParseResult;
    use ParseResult::*;

    #[test]
    fn test_empty_fn() {
        match parse_file(" \t  fn \t hello {} \t \r") {
            Okay(value) => {
                print!("{:?}", value)
            },
            OkayAdvance(value, advance) => {
                print!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_empty() {
        match parse_file("") {
            Okay(value) => {
                assert!(value == File { funcs: Vec::new() });
            },
            OkayAdvance(value, advance) => {
                assert!(value == File { funcs: Vec::new() });
                assert!(advance == 0);
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }
}


