use crate::parser::ParseContext;

use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError, Span};


use ParseResult::*;

impl <Err: From<UnexpectedValueError<Pos, char>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for u8 {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, char>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();
        match cxt.store.value_at(&mut curr_pos) {
            Some(char) => {
                if char == *self as _ {
                    Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
                } else {
                    Error(UnexpectedValueError { pos: cxt.pos, found: char, expected: *self as _ }.into())
                }
            },
            None => Error(UnexpectedEndError { pos: cxt.pos }.into()),
        }
    }
}
