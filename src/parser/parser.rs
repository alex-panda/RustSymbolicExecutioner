use std::rc::Rc;

use super::{ParseResult, Span, ZeroOrMore, ParseNode, OneOfTwo, SpanOf, Map, OneOfThree, Spanned};


#[derive(PartialEq)]
pub struct Arg {
    id: Span<usize>,
    ty: Span<usize>,
}

#[derive(PartialEq)]
pub struct FuncBody {

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
    let w = SpanOf(ZeroOrMore(OneOfTwo(..32u32, 127u32)));

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

    // the rule to parse a `Func`
    let func_rule = Rc::new(
        Map(
            Spanned((w.clone(), "fn", w.clone(), ident.clone(), w.clone(), '(', w.clone(), ')', w.clone(), '{', w.clone(), '}', w.clone())),
            |vals| {
                vals.map_value(|(span, (_, f, _, ident, _, oparen, _, cparen, _, ocbrace, _, ccbrace, _))| {
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
            ZeroOrMore(func_rule.clone()),
            |_: ParseResult<Vec<Func>, String, usize>| {
                Okay(File {
                    funcs: Vec::new(),
                })
            }
        )
    );

    file_rule.parse(&file, 0)
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


