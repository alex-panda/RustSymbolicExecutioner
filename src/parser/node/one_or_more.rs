use crate::parser::{ZSTNode, Span, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, NoAdvanceError, FailedFirstParseError};

#[allow(non_snake_case)]
pub fn OneOrMore<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> OneOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    OneOrMoreNode { child, _zst: ZSTNode::default(), }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneOrMoreNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;

impl <Ok, Err: From<NoAdvanceError<Pos>> + From<FailedFirstParseError<Pos, Err>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Vec<Ok>, Err, Store, Pos, V> for OneOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    fn do_parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Vec<Ok>, Err, Pos> {
        let mut accume: Vec<Ok> = Vec::new();

        // try the first parse
        match self.child.do_parse(cxt.clone()) {
            Okay(okay, advance) => {
                if advance.key() == cxt.pos.key() { return Panic(NoAdvanceError { pos: cxt.pos }.into()); }
                accume.push(okay);
                cxt.pos = advance;
            },
            Error(cause) => return Error(FailedFirstParseError { pos: cxt.pos, cause }.into()),
            Panic(err) => return Panic(err),
        }

        // try all subsequent parses
        loop {
            match self.child.do_parse(cxt.clone()) {
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

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();

        // try first parse
        match self.child.do_parse_span(cxt.clone()) {
            Okay(_, advance) => {
                if advance.key() == cxt.pos.key() { return Panic(NoAdvanceError { pos: cxt.pos.clone() }.into()); }
                curr_pos = advance
            },
            Error(cause) => return Error(FailedFirstParseError { pos: cxt.pos, cause }.into()),
            Panic(error) => return Panic(error),
        }

        // try all subsequent parses
        loop {
            match self.child.do_parse_span(cxt.clone()) {
                Okay(_, advance) => {
                    if advance.key() == cxt.pos.key() { return Panic(NoAdvanceError { pos: cxt.pos }.into()); }
                    curr_pos = advance
                },
                Error(_) => return Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos),
                Panic(error) => return Panic(error),
            }
        }
    }
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, Ok, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for OneOrMoreNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}