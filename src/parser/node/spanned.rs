use crate::parser::{Span, ParsePos, ParseStore, ParseValue, ParseNode, ParseResult, ZSTNode, ParseContext};


/// 
/// Returns a node that returns a tuple containing both the span of every
/// successful parse of its child node and the result of every successful parse
/// of its child node.
/// 
#[allow(non_snake_case)]
pub fn Spanned<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> SpannedNode<Child, Ok, Err, Store, Pos, V> {
    SpannedNode { child, _zst: ZSTNode::default() }
}

pub struct SpannedNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(crate) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}


use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<(Span<Pos>, Ok), Err, Store, Pos, V> for SpannedNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(Span<Pos>, Ok), Err, Pos> {
        match self.child.parse(cxt.clone()) {
            Okay(value, advance) => Okay((Span::new(cxt.pos.clone(), advance.clone()), value ), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        self.child.parse_span(cxt)
    }
}


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone> Clone for SpannedNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}

