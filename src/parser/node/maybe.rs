use crate::parser::{ZSTNode, Span};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn Maybe<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> MaybeNode<Child, Ok, Err, Store, Pos, V> {
    MaybeNode { child, zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Option<Ok>, Err, Store, Pos, V> for MaybeNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Option<Ok>, Err, Pos> {
        match self.child.parse(store, pos) {
            Okay(value) => Okay(Some(value)),
            OkayAdvance(value, advance) => OkayAdvance(Some(value), advance),
            Error(_) => Okay(None),
            Panic(error) => Panic(error),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.child.parse_span(store, pos.clone()) {
            Okay(_) => Okay(Span::new(pos.clone(), pos)),
            OkayAdvance(_, advance) => OkayAdvance(Span::new(pos, advance.clone()), advance),
            Error(_) => Okay(Span::new(pos.clone(), pos)),
            Panic(error) => Panic(error),
        }
    }
}