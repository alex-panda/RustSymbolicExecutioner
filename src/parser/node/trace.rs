use crate::parser::ZSTNode;

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn Trace<F: Fn(), Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(f: F) -> TraceNode<F, Err, Store, Pos, V> {
    TraceNode {
        func: f,
        _zst: ZSTNode::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraceNode<F: Fn(), Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub func: F,
    _zst: ZSTNode<(), Err, Store, Pos, V>,
}

impl <F: Fn(), Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for TraceNode<F, Err, Store, Pos, V> {
    fn parse(&self, _: &Store, _: Pos) -> ParseResult<(), Err, Pos> {
        (self.func)();
        ParseResult::Okay(())
    }
}


