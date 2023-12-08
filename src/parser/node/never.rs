use crate::parser::{ZSTNode, ParseContext, NeverError};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// Returns a node that never parses its child node and always fails to parse. The main
/// use of this node is for debugging purposes.
/// 
#[allow(unused)]
#[allow(non_snake_case)]
pub fn Never<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<NeverError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> NeverNode<Child, Ok, Err, Store, Pos, V> {
    NeverNode { child, _zst: ZSTNode::default() }
}

pub struct NeverNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<NeverError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;
impl <Ok, Err: From<NeverError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for NeverNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, _cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        Error(Err::from(NeverError))
    }

    fn parse_span<'a>(&self, _cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        Error(Err::from(NeverError))
    }
}

impl <Ok, Err: From<NeverError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone> Clone for NeverNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}