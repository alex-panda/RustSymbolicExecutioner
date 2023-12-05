use zst::ZST;

use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};

/// 
/// Returns a node that parses the given child node at the current parse
/// position and then maps all `ParseResult::Okay` results to a new value using
/// the given function.
/// 
#[allow(non_snake_case)]
pub fn MapV<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk>(child: Child, f: F) -> MapVNode<Child, F, Ok, Err, Store, Pos, V, OOk> {
    MapVNode {
        child,
        func: f,
        _zst: ZSTNode::default(),
        _phantom_ook: ZST::default(),
    }
}

pub struct MapVNode<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk> {
    pub child: Child, 
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
    _phantom_ook: ZST<OOk>,
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk> ParseNode<OOk, Err, Store, Pos, V> for MapVNode<Child, F, Ok, Err, Store, Pos, V, OOk> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<OOk, Err, Pos> {
        use ParseResult::*;
        match self.child.parse(cxt) {
            Okay(value, advance) => Okay((self.func)(value), advance),
            Error(err) => Error(err),
            Panic(err) => Panic(err),
        }
    }
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, F: Clone + Fn(Ok) -> OOk, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, OOk> Clone for MapVNode<Child, F, Ok, Err, Store, Pos, V, OOk> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), func: self.func.clone(), _zst: self._zst.clone(), _phantom_ook: Default::default() }
    }
}