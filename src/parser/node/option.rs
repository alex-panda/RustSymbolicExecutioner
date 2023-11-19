use super::super::{ParseNode, ParseValue, ParsePos, ParseStore, ParseResult, Span, ExpectedChildError};


impl <Ok, Err: From<ExpectedChildError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for Option<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self {
            Some(child) => child.parse(store, pos),
            None => ParseResult::Error(ExpectedChildError.into()),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self {
            Some(child) => child.parse_span(store, pos),
            None => ParseResult::Error(ExpectedChildError.into()),
        }
    }
}


