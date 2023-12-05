use core::fmt::Debug;
use crate::parser::{ZSTNode, Span, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// Returns a node that prints (in debug mode) every result that the given child
/// parses. This node mainly exists for debugging purposes.
/// 
#[allow(non_snake_case)]
pub fn DPrint<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Debug, Err: Debug, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> DPrintNode<Child, Ok, Err, Store, Pos, V> {
    DPrintNode { child, _zst: ZSTNode::default() }
}

pub struct DPrintNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Debug, Err: Debug, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

impl <Ok: Debug, Err: Debug, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for DPrintNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        let res = self.child.parse(cxt.clone());
        println!("{}: {:?}", cxt.pos, res);
        res
    }
    
    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let res = self.child.parse_span(cxt.clone());
        println!("{}: {:?}", cxt.pos, res);
        res
    }
}


impl <Ok: Debug, Err: Debug, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone> Clone for DPrintNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), _zst: self._zst.clone() }
    }
}