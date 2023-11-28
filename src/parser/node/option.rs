use crate::parser::ParseContext;

use super::super::{ParseNode, ParseValue, ParsePos, ParseStore, ParseResult, Span, ExpectedChildError};


impl <Ok, Err: From<ExpectedChildError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for Option<Child> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        match self {
            Some(child) => child.do_parse(cxt),
            None => ParseResult::Error(ExpectedChildError.into()),
        }
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self {
            Some(child) => child.do_parse_span(cxt),
            None => ParseResult::Error(ExpectedChildError.into()),
        }
    }
}


