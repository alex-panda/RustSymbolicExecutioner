use crate::parser::{ZSTNode, Span, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

#[allow(non_snake_case)]
pub fn Is<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> IsNode<Child, Ok, Err, Store, Pos, V> {
    IsNode { child, _zst: ZSTNode::default() }
}

pub struct IsNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for IsNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        match self.child.parse(cxt.clone()) {
            Okay(value, _) => Okay(value, cxt.pos),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
    
    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.child.parse_span(cxt.clone()) {
            Okay(_, advance) => Okay(Span::new(cxt.pos.clone(), advance), cxt.pos),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
}


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone> Clone for IsNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}