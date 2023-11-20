use crate::parser::AnyOf6;

use super::{ParseResult, Span, ZeroOrMore, ParseNode, SpanOf, Map, OneOf3, Spanned, OneOrMore, AnyOf3, Maybe, AnyOf2, MapV, OneOf6, SRule, Leader, Surround, End, Req, OneOf5, AnyOf5, AnyOf4, OneOf4, OneOf2};


#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub id: Span<usize>,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    statements: Vec<Statement>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Array {
        /// The type of every item in the array (since arrays can only hold 1
        /// type of item, this is that type).
        item_type: Box<Self>,
        /// The number of items in the array.
        item_number: usize,
    },
    Tuple {
        /// The types in the tuple, stored in the same order as given.
        types: Vec<Self>,
    },
    Template {
        /// The name of the type.
        name: Span<usize>,
        /// The arguments for the type.
        args: Vec<Self>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Var(Span<usize>),
    Path(Vec<Span<usize>>),
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
    pub ret_type: ReturnType,
    pub body: Group,
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub funcs: Vec<Func>,
}

use ParseResult::*;

pub fn parse_file(file: &str) -> ParseResult<File, String, usize> {
    // create `expr` (it requires a number of recursive child nodes)
    let group = SRule();
    let expr = SRule();
    let value = SRule();
    let power = SRule();
    let mul_or_div = SRule();
    let ty = SRule();
    let statement = SRule();
    let type_tuple = SRule();

    // a rule that just consumes whitespace space
    let w = SpanOf(ZeroOrMore(OneOf2(..=32u32, 127u32)));

    // a rule to parse an ascii letter (lower case or upper case)
    let alpha = SpanOf(OneOf2(97..=122, 65..=90));
    // a rule to parse an ascii numeric (0,1,2,etc.)
    let numeric = SpanOf(48..=57);
    // a rule to parse an identifier
    let ident = SpanOf(
        (
            OneOf2(alpha.clone(), '_'),
            ZeroOrMore(OneOf3(alpha.clone(), numeric.clone(), '_'))
        )
    );

    let param = Map(
        Leader(
            &ident,
            (&w, Leader(
                    ':', (&w, &ty),
                    |colon_span, _, _| format!("{}: (param) missing type after this colon", colon_span),
                )
            ),
            |id_span, _, _| format!("{}: (param) missing arg's type", id_span),
        ),
        |res| {
            res.map_value(|(id, (_, (_colon, (_, ty))))| {
                Arg {
                    id,
                    ty,
                }
            })
        }
    );

    let params = Map(
        OneOf3(
            (param.clone(), OneOrMore((w.clone(), Leader(',', (w.clone(), param.clone()), |_, comma_span, _| format!("{}: (params) erroneous comma", comma_span))))),
            param.clone(),
            ()
        ),
        |res| {
            res.map_value(|any_of_three| {
                match any_of_three {
                    AnyOf3::Child1((param1, other_params)) => {
                        let mut accume = Vec::with_capacity(1 + other_params.len());
                        accume.push(param1);
                        for (_, (_comma, (_, param))) in other_params {
                            accume.push(param);
                        }
                        accume
                    },
                    AnyOf3::Child2(params) => Vec::from([params]),
                    AnyOf3::Child3(_) => Vec::new(),
                }
            })
        }
    );

    let return_type = Map(
        Leader("->", (&w, OneOf2(&ident, '!')), |arrow_span, _, _| format!("{}: (return_type) missing return type", arrow_span)),
        |res| {
            res.map_value(|(_arrow, (_, any_of_two))| {
                match any_of_two {
                    AnyOf2::Child1(id) => ReturnType::Type(id),
                    AnyOf2::Child2(_) => ReturnType::Never,
                }
            })
        }
    );

    let unt_type = 
        MapV(
            OneOf6(
                "u8",
                "u16",
                "u32",
                "u64",
                "u128",
                "usize",
            ),
            |res| {
                use AnyOf6::*;
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
            OneOf6(
                "i8",
                "i16",
                "i32",
                "i64",
                "i128",
                "isize",
            ),
            |res| {
                use AnyOf6::*;
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
            Maybe(OneOf2('+', '-')),
            SpanOf(OneOrMore(numeric.clone())),
            Maybe(OneOf2(
                int_type.clone(),
                unt_type.clone(),
            ))
        )),
        |(span, (sign, numerics, t))| {
            let (sign_span, sign) = match sign.clone() {
                Some(v) => match v {
                    AnyOf2::Child1(span) => (Some(span), Sign::Positive),
                    AnyOf2::Child2(span) => (Some(span), Sign::Negative),
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
                        AnyOf2::Child1(ty) => ty,
                        AnyOf2::Child2(ty) => ty,
                    }), 
                    None => None
                },
            }
        }
    );

    let float = MapV(
        Spanned((Maybe(OneOf2('-', '+')), SpanOf((OneOrMore(numeric.clone()), '.', OneOrMore(numeric.clone()))), Maybe(OneOf2("f32", "f64")))),
        |(span, (sign, numerics, ty))| {
            let (sign_span, sign) = match sign {
                Some(v) => match v {
                    AnyOf2::Child1(span) => (Some(span), Sign::Negative),
                    AnyOf2::Child2(span) => (Some(span), Sign::Positive),
                },
                None => (None, Sign::Positive),
            };

            let ty = match ty {
                Some(v) => Some(match v {
                    AnyOf2::Child1(span) => FloatType::F32(span),
                    AnyOf2::Child2(span) => FloatType::F64(span),
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

    {
        expr.set(
            MapV(
                (&mul_or_div, Maybe((&w, OneOf2('+', '-'), &w, expr.din()))),
                |(value, maybe_add_or_sub)| {
                    match maybe_add_or_sub {
                        Some((_, add_or_sub, _, expr)) => {
                            match add_or_sub {
                                AnyOf2::Child1(span) => Expr::Add(Box::new(value), span, Box::new(expr)),
                                AnyOf2::Child2(span) => Expr::Sub(Box::new(value), span, Box::new(expr)),
                            }
                        },
                        None => value,
                    }
                }
            )
        );

        mul_or_div.set(
            MapV(
                (&power, Maybe((&w, OneOf2('*', '/'), &w, mul_or_div.din()))),
                |(value, maybe_mul_or_div)| {
                    match maybe_mul_or_div {
                        Some((_, mul_or_div, _, expr)) => {
                            match mul_or_div {
                                AnyOf2::Child1(span) => Expr::Add(Box::new(value), span, Box::new(expr)),
                                AnyOf2::Child2(span) => Expr::Sub(Box::new(value), span, Box::new(expr)),
                            }
                        },
                        None => value,
                    }
                }
            )
        );

        power.set(
            MapV(
                (&value, Maybe((&w, '^', &w, power.din()))),
                |(value, maybe_pow)| {
                match maybe_pow {
                    Some((_, pow, _, expr)) => {
                        Expr::Pow(Box::new(value), pow, Box::new(expr))
                    },
                    None => value
                }
            })
        );

        value.set(
            MapV(
                OneOf5(
                    &ident,
                    &integer,
                    &float,
                    ('(', &w, expr.din(), &w, ')'),
                    &group
                ),
                |res| {
                    match res {
                        AnyOf5::Child1(ident) => Expr::Var(ident),
                        AnyOf5::Child2(int) => Expr::Int(int),
                        AnyOf5::Child3(float) => Expr::Float(float),
                        AnyOf5::Child4((_oparen, _, expr, _, _cparen)) => expr,
                        AnyOf5::Child5(group) => Expr::Group(group),
                    }
                }
            )
        );
    }

    group.set(
        MapV(
            Surround(
                    '{', (&w, ZeroOrMore((statement.din(), &w))), '}',
                    |_, _, e| e,
                    |_, ocbrace_span, _, _| format!("{}: openning curly brace is missing its complementary closing curly brace to end the scope", ocbrace_span),
                ),
            |(_lcbrace, (_, statements), _rcbrace)| {
                Group {
                    statements: statements.into_iter().map(|v|v.0).collect(),
                }
            }
        )
    );

    type_tuple.set(
        MapV(
            Surround(
                '(',
                    (
                        &w,
                        Maybe(
                        (ty.din(), &w, ZeroOrMore((',', &w, &ty, &w)))
                        )
                    ),
                ')',
                |_, _, e| e,
                |_, oparen_span, _, _| format!("{}: missing closing parenthesis after this open parenthesis", oparen_span),
            ),
            |(oparen_span, (_, maybe_types), cparen_span)| {
                match maybe_types {
                    Some((t1, _, types)) => {
                        let mut types: Vec<Type> = types.into_iter().map(|v|v.2).collect();
                        types.insert(0, t1);
                        Type::Tuple { types }
                    },
                    None => Type::Tuple { types: Vec::new() },
                }
            }
        )
    );

    ty.set(
        MapV(
            OneOf2(
                type_tuple.din(),
                (
                    &ident,
                    Maybe((
                        &w,
                        Surround(
                            '<',
                            ( 
                                &w,
                                ZeroOrMore((ty.din(), &w))
                            ),
                            '>',
                        |_, oarrow_span, _| format!("{}: (ty) expected values within this type bounds", oarrow_span),
                            |_, oarrow_span, _, _| format!("{}: (ty) expected closing arrow ('>') after this openning arrow ('<')", oarrow_span),
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
                        Type::Template {
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

    statement.set(
        MapV(
            OneOf4(
                Leader("return", (&w, &expr, Maybe((&w, ';'))), |_, return_span, _| format!("{}: expected expression after this \"return\" keyword", return_span)),
                Leader("let", (&w, &ident, Maybe((&w, ':', &w, &ty)), &w, '=', &w, &expr, Maybe((&w, ';'))), |_, let_span, _| format!("{} expected_variable assignment after this let statement", let_span)),
                (&expr, Maybe((&w, ';'))),
                ";",
            ),
            |any_of_four| {
                match any_of_four {
                    // return
                    AnyOf4::Child1((return_span, (_, expr, maybe_semi))) => {
                        Statement::Return {
                            return_span,
                            expr,
                            semi: match maybe_semi { Some(v) => Some(v.1), None => None }
                        }
                    },
                    // let
                    AnyOf4::Child2((let_span, (_, ident, maybe_type, _, eq_span, _, expr, maybe_semi))) => {
                        Statement::Assign {
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
                    },
                    // expr
                    AnyOf4::Child3((expr, maybe_semi)) => {
                        Statement::Expr {
                            expr,
                            semi: match maybe_semi {
                                Some((_, semi)) => Some(semi),
                                None => None
                            }
                        }
                    },
                    // semi
                    AnyOf4::Child4(semi_span) => {
                        Statement::SColon { semi: semi_span }
                    },
                }
            }
        )
    );

    // the rule to parse a `Func`
    let func_rule = 
        MapV(
            Spanned(Leader(
                "fn",
                (&w, Leader(&ident,
                        (&w,
                            Surround(
                                '(', (&w, &params, &w), ')',
                                |_, oparen_span, _| format!("{}: (func_rule) expected parameters in this function argument scope", oparen_span),
                                |_, oparen_span, _, _| format!("{}: (func_rule) expected closing parenthesis to match this open parenthesis", oparen_span)
                            ),
                            Req((&w, &return_type), |_, _, _| format!("(func_rule) function requires a return type")),
                            Req((&w, &group), |_, _, _| format!("(func_rule) function requires a function body"))
                        ),
                        |_, ident_span, _| format!("{}: (func_rule) expected function parameters and body after function identifier", ident_span)
                    )
                ),
                |_, fn_span, _| format!("{}: (func_rule) expected correct function syntax after 'fn' keyword", fn_span))),
            |(span, (fn_span, (_, (id_span, (_, (_oparen, (_, params, _), _cparen), (_, ret_type), (_, body))))))| {
                Func {
                    span,
                    fn_span,
                    id: id_span,
                    args: params,
                    ret_type,
                    body,
                }
            }
        );

    // the rule to parse a `File`
    let file_rule = 
        Map(
            (ZeroOrMore((&w, &func_rule)), &w, Req(End(), |_, _, _| format!("the parse did not make it to the end of the file"))),
            |res: ParseResult<(Vec<(Span<usize>, Func)>, Span<usize>, ()), String, usize>| {
                res.map_value(|(v, _, _)| {
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
    fn test_stmt_semi_end_fn() {
        match parse_file(" fn a(x:i32, y:i32) -> u128 { 
            let u = 6; 
            8 / 4; 
            5 - 2; } ") {
            Okay(value) => {
                println!("{:?}", value)
            },
            OkayAdvance(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_stmt_fn() {
        match parse_file(" fn hello() -> u8 {  (10 + 3) * 3 / 4 ^ (10); 10; 23; 0 } ") {
            Okay(value) => {
                println!("{:?}", value)
            },
            OkayAdvance(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_expr_fn() {
        match parse_file(" fn hello() -> u8 {  (10 + 3) * 3 / 4 ^ (10) } ") {
            Okay(value) => {
                println!("{:?}", value)
            },
            OkayAdvance(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_add_fn() {
        match parse_file(" fn hello() -> u8 {10 + 3}") {
            Okay(value) => {
                println!("{:?}", value)
            },
            OkayAdvance(value, advance) => {
                println!("{:?} {:?}", value, advance)
            },
            Error(error) => panic!("Error: {}", error),
            Panic(error) => panic!("Panic: {}", error),
        }
    }

    #[test]
    fn test_empty_fn() {
        match parse_file(" fn hello() -> u8 {0}") {
            Okay(value) => {
                println!("{:?}", value)
            },
            OkayAdvance(value, advance) => {
                println!("{:?} {:?}", value, advance)
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


