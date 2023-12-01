use crate::parser::{ZSTNode, ParseContext};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn Trace<F: Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(trace: i32, f: F) -> TraceNode<F, Err, Store, Pos, V> {
    TraceNode {
        trace,
        func: f,
        _zst: ZSTNode::default(),
    }
}

pub struct TraceNode<F: Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub trace: i32,
    pub func: F,
    _zst: ZSTNode<(), Err, Store, Pos, V>,
}

impl <F: Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for TraceNode<F, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        if self.trace >= cxt.trace {
            (self.func)(cxt.pos.clone());
        }
        ParseResult::Okay((), cxt.pos)
    }
}

impl <F: Clone + Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for TraceNode<F, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { trace: self.trace, func: self.func.clone(), _zst: self._zst.clone() }
    }
}

