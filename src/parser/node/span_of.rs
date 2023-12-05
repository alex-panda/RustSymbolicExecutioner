use crate::parser::{Span, ZSTNode, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};


/// 
/// Returns a node that maps all successfull parses of its child node to the
/// span of the successfully parsed material.
/// 
#[allow(non_snake_case)]
pub fn SpanOf<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> SpanOfNode<Child, Ok, Err, Store, Pos, V> {
    SpanOfNode { child, _zst: ZSTNode::default() }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpanOfNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(crate) _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Span<Pos>, Err, Store, Pos, V> for SpanOfNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.child.parse_span(cxt.clone()) {
            Okay(_, advance) => Okay(Span::new(cxt.pos, advance.clone()), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        self.child.parse_span(cxt)
    }
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for SpanOfNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}
