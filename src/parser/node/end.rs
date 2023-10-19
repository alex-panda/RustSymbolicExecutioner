use crate::parser::{UnexpectedEndError, ZSTNode};

use super::super::{ParseValue, ParseStore, ParsePos, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn End<Ok, Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>() -> EndNode<Ok, Err, Store, Pos, V> {
    EndNode { _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndNode<Ok, Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

impl <Ok, Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for EndNode<Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<(), Err, Pos> {
        match store.peek_at(&pos) {
            Some(_) => ParseResult::Okay(()),
            None => ParseResult::Error(UnexpectedEndError { pos }.into()),
        }
    }
}