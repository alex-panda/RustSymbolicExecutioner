use crate::parser::{ExpectedEndError, ZSTNode};

use super::super::{ParseValue, ParseStore, ParsePos, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn End<Err: From<ExpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>() -> EndNode<Err, Store, Pos, V> {
    EndNode { _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndNode<Err: From<ExpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    _zst: ZSTNode<(), Err, Store, Pos, V>
}

impl <Err: From<ExpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for EndNode<Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<(), Err, Pos> {
        match store.peek_at(&pos) {
            Some(_) => ParseResult::Error(ExpectedEndError { pos }.into()),
            None => ParseResult::Okay(()),
        }
    }
}