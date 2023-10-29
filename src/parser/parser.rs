use std::rc::Rc;

use super::{ParseResult, Span, ZeroOrMore, ParseNode, OneOfTwo, SpanOf, Map, OneOfThree, Spanned, OneOrMore, AnyOfThree, Maybe, AnyOfTwo};


#[derive(PartialEq, Clone)]
pub struct Arg {
    pub id: Span<usize>,
    pub ty: Span<usize>,
}

#[derive(PartialEq, Clone)]
pub struct FuncBody {

}

#[derive(PartialEq, Clone)]
pub enum ReturnType {
    /// Return the given type.
    Type(Span<usize>),
    /// Function never returns.
    Never,
}

#[derive(PartialEq)]
pub struct Func {
    pub span: Span<usize>,
    pub fn_: Span<usize>,
    pub id: Span<usize>,
    pub args: Vec<Arg>,
    pub body: FuncBody,
}

#[derive(PartialEq)]
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
                    AnyOfThree::One(_) => Vec::<Arg>::new(),
                    AnyOfThree::Two((params, _)) => Vec::from([params]),
                    AnyOfThree::Three((param1, other_params)) => {
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
                    AnyOfTwo::One(id) => ReturnType::Type(id),
                    AnyOfTwo::Two(_) => ReturnType::Never,
                }
            })
        }
    );

    // the rule to parse a `Func`
    let func_rule = Rc::new(
        Map(
            Spanned(("fn", w.clone(), ident.clone(), w.clone(), '(', w.clone(), params.clone(), w.clone(), ')', w.clone(), return_type, w.clone(), '{', w.clone(), '}')),
            |vals| {
                vals.map_value(|(span, (f, _, ident, _, oparen, _, params, _, cparen, _, ret_type, _, ocbrace, _, ccbrace))| {
                    Func {
                        span,
                        fn_: f,
                        id: ident,
                        args: Vec::new(),
                        body: FuncBody { },
                    }
                })
            }
        )
    );

    // the rule to parse a `File`
    let file_rule = Rc::new(
        Map(
            ZeroOrMore((w.clone(), func_rule.clone(), w.clone())),
            |_: ParseResult<Vec<(Span<usize>, Func, Span<usize>)>, String, usize>| {
                Okay(File {
                    funcs: Vec::new(),
                })
            }
        )
    );

    file_rule.parse(&file, 0usize)
}


#[cfg(test)]
mod tests {
    use crate::parser::parser::File;

    use super::parse_file;
    use super::super::ParseResult;
    use ParseResult::*;

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


