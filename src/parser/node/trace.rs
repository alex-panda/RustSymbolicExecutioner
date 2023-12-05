use crate::parser::{ZSTNode, ParseContext};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

/// 
/// Returns a node that calls the given function with the current parse position
/// every time it is parsed. Its main use is for debugging purposes as it allows
/// `println!()` statements to be more easily inserted into the parse tree.
/// 
#[allow(non_snake_case)]
pub fn Trace<F: Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(f: F) -> TraceNode<F, Err, Store, Pos, V> {
    TraceNode {
        func: f,
        _zst: ZSTNode::default(),
    }
}

pub struct TraceNode<F: Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub func: F,
    _zst: ZSTNode<(), Err, Store, Pos, V>,
}

impl <F: Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for TraceNode<F, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        (self.func)(cxt.pos.clone());
        ParseResult::Okay((), cxt.pos)
    }
}

impl <F: Clone + Fn(Pos), Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for TraceNode<F, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { func: self.func.clone(), _zst: self._zst.clone() }
    }
}

