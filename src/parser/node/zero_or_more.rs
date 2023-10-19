use crate::parser::ZSTNode;

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, NoAdvanceError};

#[allow(non_snake_case)]
pub fn ZeroOrMore<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> ZeroOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    ZeroOrMoreNode { child, _zst: ZSTNode::default(), }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZeroOrMoreNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;

impl <Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Vec<Ok>, Err, Store, Pos, V> for ZeroOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, mut pos: Pos) -> ParseResult<Vec<Ok>, Err, Pos> {
        let mut accume: Vec<Ok> = Vec::new();

        loop {
            match self.child.parse(store, pos.clone()) {
                Okay(_) => return Panic(NoAdvanceError { pos }.into()),
                OkayAdvance(okay, advance) => {
                    if pos.key() == advance.key() { return Panic(NoAdvanceError { pos }.into()); }
                    accume.push(okay);
                    pos = advance;
                },
                Error(_) => return OkayAdvance(accume, pos),
                Panic(err) => return Panic(err),
            }
        }
    }
}



