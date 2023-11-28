use crate::parser::{Span, UnexpectedEndError, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// A node that matches and consumes any value of the parse, only failing if
/// there is no value in the parse.
/// 
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyV;

use ParseResult::*;
impl <Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Span<Pos>, Err, Store, Pos, V> for AnyV {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();
        match cxt.store.value_at(&mut curr_pos) {
            Some(_) => Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos),
            None => Error(UnexpectedEndError { pos: cxt.pos }.into()),
        }
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        self.do_parse(cxt)
    }
}
