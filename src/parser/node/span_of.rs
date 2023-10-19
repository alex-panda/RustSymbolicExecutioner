use crate::parser::{Span, ZSTNode};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};


#[allow(non_snake_case)]
pub fn SpanOf<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> SpanOfNode<Child, Ok, Err, Store, Pos, V> {
    SpanOfNode { child, _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpanOfNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(crate) _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Span<Pos>, Err, Store, Pos, V> for SpanOfNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.child.parse_span(store, pos.clone()) {
            Okay(_) => Okay(Span::new(pos.clone(), pos)),
            OkayAdvance(_, advance) => OkayAdvance(Span::new(pos, advance.clone()), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        self.child.parse_span(store, pos)
    }
}