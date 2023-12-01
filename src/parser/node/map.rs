use zst::ZST;

use crate::parser::{ZSTNode, ParseContext};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn Map<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(ParseResult<Ok, Err, Pos>) -> ParseResult<OOk, OErr, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk, OErr>(child: Child, f: F) -> MapNode<Child, F, Ok, Err, Store, Pos, V, OOk, OErr> {
    MapNode {
        child,
        func: f,
        _zst: ZSTNode::default(),
        _phantom_ook: ZST::default(),
        _phantom_oerr: ZST::default(),
    }
}

pub struct MapNode<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(ParseResult<Ok, Err, Pos>) -> ParseResult<OOk, OErr, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk, OErr> {
    pub child: Child, 
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
    _phantom_ook: ZST<OOk>,
    _phantom_oerr: ZST<OErr>,
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(ParseResult<Ok, Err, Pos>) -> ParseResult<OOk, OErr, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk, OErr> ParseNode<OOk, OErr, Store, Pos, V> for MapNode<Child, F, Ok, Err, Store, Pos, V, OOk, OErr> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<OOk, OErr, Pos> {
        (self.func)(self.child.parse(cxt))
    }
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, F: Clone + Fn(ParseResult<Ok, Err, Pos>) -> ParseResult<OOk, OErr, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk, OErr> Clone for MapNode<Child, F, Ok, Err, Store, Pos, V, OOk, OErr> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), func: self.func.clone(), _zst: self._zst.clone(), _phantom_ook: Default::default(), _phantom_oerr: Default::default() }
    }
}

