use crate::parser::{ZSTNode, Span};

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

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = pos.clone();

        // try first parse
        match self.child.parse_span(store, curr_pos.clone()) {
            Okay(_) => return Panic(NoAdvanceError { pos }.into()),
            OkayAdvance(_, advance) => {
                if curr_pos.key() == advance.key() { return Panic(NoAdvanceError { pos }.into()) }
                curr_pos = advance
            },
            Error(cause) => return Error(FailedFirstParseError { pos, cause }.into()),
            Panic(error) => return Panic(error),
        }

        // try all subsequent parses
        loop {
            match self.child.parse_span(store, curr_pos.clone()) {
                Okay(_) => return Panic(NoAdvanceError { pos }.into()),
                OkayAdvance(_, advance) => {
                    if curr_pos.key() == advance.key() { return Panic(NoAdvanceError { pos }.into()) }
                    curr_pos = advance
                },
                Error(_) => return OkayAdvance(Span::new(pos, curr_pos.clone()), curr_pos),
                Panic(error) => return Panic(error),
            }
        }
    }
}