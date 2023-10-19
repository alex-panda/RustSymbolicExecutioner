use crate::parser::ZSTNode;

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, NoAdvanceError, FailedFirstParseError};

#[allow(non_snake_case)]
pub fn OneOrMore<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> OneOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    OneOrMoreNode { child, _zst: ZSTNode::default(), }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneOrMoreNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;

impl <Ok, Err: From<NoAdvanceError<Pos>> + From<FailedFirstParseError<Pos, Err>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Vec<Ok>, Err, Store, Pos, V> for OneOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, mut pos: Pos) -> ParseResult<Vec<Ok>, Err, Pos> {
        let mut accume: Vec<Ok> = Vec::new();

        // try the first parse
        match self.child.parse(store, pos.clone()) {
            Okay(_) => return Panic(NoAdvanceError { pos }.into()),
            OkayAdvance(okay, advance) => {
                if pos.key() == advance.key() { return Panic(NoAdvanceError { pos }.into()); }
                accume.push(okay);
                pos = advance;
            },
            Error(cause) => return Error(FailedFirstParseError { pos, cause }.into()),
            Panic(err) => return Panic(err),
        }

        // try all subsequent parses
        loop {
            match self.child.parse(store, pos.clone()) {
                Okay(_) => return Panic(NoAdvanceError { pos }.into()),
                OkayAdvance(okay, advance) => {
                    if pos.key() == pos.key() { return Panic(NoAdvanceError { pos }.into()); }
                    accume.push(okay);
                    pos = advance;
                },
                Error(_) => return OkayAdvance(accume, pos),
                Panic(err) => return Panic(err),
            }
        }
    }
}