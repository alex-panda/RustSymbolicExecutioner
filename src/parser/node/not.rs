use crate::parser::{ZSTNode, Span};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, UnexpectedSuccessError};

///
/// A negative lookahead node i.e. a node that only parses successfully if the
/// given child node fails to parse. It will return `ParseResult::Okay(())`
/// when the child failst to parse and
/// `ParseResult::Error(UnexpectedSuccessError.into())` when the child node
/// parses successfully.
/// 
#[allow(non_snake_case)]
pub fn Not<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<UnexpectedSuccessError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> NotNode<Child, Ok, Err, Store, Pos, V> {
    NotNode { child, _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<UnexpectedSuccessError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;
impl <Ok, Err: From<UnexpectedSuccessError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<(), Err, Store, Pos, V> for NotNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<(), Err, Pos> {
        match self.child.parse_span(store, pos.clone()) {
            Okay(_, _) => Error(UnexpectedSuccessError { pos }.into()),
            Error(_) => Okay((), pos),
            Panic(err) => Panic(err),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        match self.child.parse_span(store, pos.clone()) {
            Okay(_, _) => Error(UnexpectedSuccessError { pos }.into()),
            Error(_) => Okay(Span::new(pos.clone(), pos.clone()), pos),
            Panic(err) => Panic(err),
        }
    }
}
