use crate::parser::ZSTNode;

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, UnexpectedSuccessError};


#[allow(non_snake_case)]
pub fn Not<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<UnexpectedSuccessError<Pos, Ok>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> NotNode<Child, Ok, Err, Store, Pos, V> {
    NotNode { child, _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<UnexpectedSuccessError<Pos, Ok>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <Ok, Err: From<UnexpectedSuccessError<Pos, Ok>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<(), Err, Store, Pos, V> for NotNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<(), Err, Pos> {
        match self.child.parse(store, pos.clone()) {
            ParseResult::Okay(v) | ParseResult::OkayAdvance(v, _)
                => ParseResult::Error(UnexpectedSuccessError { pos, child_result: Some(v) }.into()),
            ParseResult::Error(_) => ParseResult::Okay(()),
            ParseResult::Panic(err) => ParseResult::Panic(err),
        }
    }
}
