use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};


/// 
/// Returns a node that will map the current position of the parse to a
/// `ParseResult` using the given function.
/// 
#[allow(non_snake_case)]
pub fn MapP<F: Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(f: F) -> MapPNode<F, Ok, Err, Store, Pos, V> {
    MapPNode {
        func: f,
        _zst: ZSTNode::default(),
    }
}

pub struct MapPNode<F: Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <F: Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for MapPNode<F, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        (self.func)(cxt.store, cxt.pos)
    }
}

impl <F: Clone + Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for MapPNode<F, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { func: self.func.clone(), _zst: self._zst.clone() }
    }
}