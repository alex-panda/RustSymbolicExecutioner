use crate::parser::{Span, ParsePos, ParseStore, ParseValue, ParseNode, ParseResult, ZSTNode};

#[allow(non_snake_case)]
pub fn Spanned<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> SpannedNode<Child, Ok, Err, Store, Pos, V> {
    SpannedNode { child, _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(crate) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}


use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<(Span<Pos>, Ok), Err, Store, Pos, V> for SpannedNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<(Span<Pos>, Ok), Err, Pos> {
        match self.child.parse(store, pos.clone()) {
            Okay(value, advance) => Okay((Span::new(pos.clone(), advance.clone()), value ), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        self.child.parse_span(store, pos.clone())
    }
}



