use crate::parser::{ZSTNode, ParseContext};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn Trace<F: Fn(), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(f: F) -> TraceNode<F, Err, Store, Pos, V> {
    TraceNode {
        func: f,
        _zst: ZSTNode::default(),
    }
}

pub struct TraceNode<F: Fn(), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub func: F,
    _zst: ZSTNode<(), Err, Store, Pos, V>,
}

impl <F: Fn(), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for TraceNode<F, Err, Store, Pos, V> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        (self.func)();
        ParseResult::Okay((), cxt.pos)
    }
}

impl <F: Clone + Fn(), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for TraceNode<F, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { func: self.func.clone(), _zst: self._zst.clone() }
    }
}

