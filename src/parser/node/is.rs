use crate::parser::{ZSTNode, Span, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// Returns a positive lookahead node that checks if the given child node parses
/// at the current parse position, succeeding if it does and failing if it does
/// not. If the child node parses successfully, this node assures that the parse
/// does not advance.
/// 
#[allow(non_snake_case)]
pub fn Is<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> IsNode<Child, Ok, Err, Store, Pos, V> {
    IsNode { child, _zst: ZSTNode::default() }
}

pub struct IsNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<(), Err, Store, Pos, V> for IsNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        match self.child.parse(cxt.clone()) {
            Okay(_, _) => Okay((), cxt.pos),
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