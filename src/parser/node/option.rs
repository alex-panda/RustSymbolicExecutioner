use crate::parser::{ParseNode, ParseValue, ParsePos, ParseStore, ParseResult, Span};


impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for Option<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self {
            Some(child) => child.parse(store, pos),
            None => panic!("`Option` used as a `ParseNode` but did not contain a child node"),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self {
            Some(child) => child.parse_span(store, pos),
            None => panic!("`Option` used as a `ParseNode` but did not contain a child node"),
        }
    }
}


