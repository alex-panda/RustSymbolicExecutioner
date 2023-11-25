use crate::parser::{ZSTNode, Span};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, NoAdvanceError};

/// 
/// Returns a parse node that will parse its child zero or more times and then
/// return a `Vec` of the resulting successful values.
/// 
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
                Okay(okay, advance) => {
                    if pos.key() == advance.key() { return Panic(NoAdvanceError { pos }.into()); }
                    accume.push(okay);
                    pos = advance;
                },
                Error(_) => return Okay(accume, pos),
                Panic(err) => return Panic(err),
            }
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        let mut curr_pos = pos.clone();
        loop {
            match self.child.parse_span(store, curr_pos.clone()) {
                Okay(_, advance) => {
                    if pos.key() == advance.key() { return Panic(NoAdvanceError { pos }.into()); }
                    curr_pos = advance;
                },
                Error(_) => return Okay(Span::new(pos, curr_pos.clone()), curr_pos),
                Panic(err) => return Panic(err),
            }
        }
    }
}



