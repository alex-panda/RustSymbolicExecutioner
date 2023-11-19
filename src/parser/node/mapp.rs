use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore};


/// 
/// Returns a node that will map the current position of the parse to a
/// `ParseResult` using the given function.
/// 
#[allow(non_snake_case)]
pub fn MapP<F: Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(f: F) -> MapPNode<F, Ok, Err, Store, Pos, V> {
    MapPNode {
        func: f,
        _zst: ZSTNode::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapPNode<F: Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <F: Fn(&Store, Pos) -> ParseResult<Ok, Err, Pos>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for MapPNode<F, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        (self.func)(store, pos)
    }
}