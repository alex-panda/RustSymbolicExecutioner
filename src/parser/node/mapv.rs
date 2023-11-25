use zst::ZST;

use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore};


#[allow(non_snake_case)]
pub fn MapV<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, OOk>(child: Child, f: F) -> MapVNode<Child, F, Ok, Err, Store, Pos, V, OOk> {
    MapVNode {
        child,
        func: f,
        _zst: ZSTNode::default(),
        _phantom_ook: ZST::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapVNode<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, OOk> {
    pub child: Child, 
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
    _phantom_ook: ZST<OOk>,
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, OOk> ParseNode<OOk, Err, Store, Pos, V> for MapVNode<Child, F, Ok, Err, Store, Pos, V, OOk> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<OOk, Err, Pos> {
        use ParseResult::*;
        match self.child.parse(store, pos) {
            Okay(value, advance) => Okay((self.func)(value), advance),
            Error(err) => Error(err),
            Panic(err) => Panic(err),
        }
    }
}