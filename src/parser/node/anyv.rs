use crate::parser::{Span, UnexpectedEndError};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// A node that matches and consumes any value of the parse, only failing if
/// there is no value in the parse.
/// 
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyV;

use ParseResult::*;
impl <Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Span<Pos>, Err, Store, Pos, V> for AnyV {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = pos.clone();
        match store.value_at(&mut curr_pos) {
            Some(_) => OkayAdvance(Span::new(pos, curr_pos.clone()), curr_pos),
            None => Error(UnexpectedEndError { pos }.into()),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        self.parse(store, pos)
    }
}
