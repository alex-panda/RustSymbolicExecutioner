use crate::parser::{ZSTNode, Span};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

#[allow(non_snake_case)]
pub fn Is<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> IsNode<Child, Ok, Err, Store, Pos, V> {
    IsNode { child, _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IsNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for IsNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self.child.parse(store, pos.clone()) {
            Okay(value) => Okay(value),
            OkayAdvance(value, _) => Okay(value),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
    
    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.child.parse_span(store, pos.clone()) {
            Okay(_) => Okay(Span::new(pos.clone(), pos)),
            OkayAdvance(_, advance) => Okay(Span::new(pos, advance)),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
}

