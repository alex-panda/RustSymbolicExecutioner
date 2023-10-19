use crate::parser::ZSTNode;

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
            ParseResult::Okay(value) => Okay(Some(value)),
            ParseResult::OkayAdvance(value, advance) => OkayAdvance(Some(value), advance),
            ParseResult::Error(_) => Okay(None),
            ParseResult::Panic(error) => Panic(error),
        }
    }
}