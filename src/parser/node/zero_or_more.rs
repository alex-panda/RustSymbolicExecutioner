use crate::parser::{ZSTNode, Span, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, NoAdvanceError};

/// 
/// Returns a parse node that will parse its child zero or more times and then
/// return a vector of the resulting successful values.
/// 
#[allow(non_snake_case)]
pub fn ZeroOrMore<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> ZeroOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    ZeroOrMoreNode { child, _zst: ZSTNode::default(), }
}

pub struct ZeroOrMoreNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;

impl <Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Vec<Ok>, Err, Store, Pos, V> for ZeroOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Vec<Ok>, Err, Pos> {
        let mut accume: Vec<Ok> = Vec::new();

        loop {
            match self.child.parse(cxt.clone()) {
                Okay(okay, advance) => {
                    if advance.key() == cxt.pos.key() { return Panic(NoAdvanceError { pos: cxt.pos }.into()); }
                    accume.push(okay);
                    cxt.pos = advance;
                },
                Error(_) => return Okay(accume, cxt.pos),
                Panic(err) => return Panic(err),
            }
        }
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();
        loop {
            match self.child.parse_span(cxt.with_pos(curr_pos.clone())) {
                Okay(_, advance) => {
                    if advance.key() == cxt.pos.key() { return Panic(NoAdvanceError { pos: cxt.pos }.into()); }
                    curr_pos = advance;
                },
                Error(_) => return Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos),
                Panic(err) => return Panic(err),
            }
        }
    }
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for ZeroOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}

