use crate::parser::{Span, UnexpectedEndError, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// Returns a node that consumes the current value of the parse, only failing if
/// the parse is out of values (i.e. we are at or past the end of the material
/// being parsed).
/// 
#[allow(non_snake_case)]
pub fn AnyV() -> AnyVNode {
    AnyVNode
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyVNode;

use ParseResult::*;
impl <Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Span<Pos>, Err, Store, Pos, V> for AnyVNode {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();
        match cxt.store.value_at(&mut curr_pos) {
            Some(_) => Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos),
            None => Error(UnexpectedEndError { pos: cxt.pos }.into()),
        }
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        self.parse(cxt)
    }
}
