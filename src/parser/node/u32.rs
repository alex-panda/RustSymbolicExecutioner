use crate::parser::{Span, ParseContext};

use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError};

use ParseResult::*;

impl <Err: From<UnexpectedValueError<Pos, u32>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for u32 {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, char>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();
        match cxt.store.value_at(&mut curr_pos) {
            Some(char) => {
                if (char as u32) == *self {
                    Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
                } else {
                    Error(UnexpectedValueError { pos: cxt.pos, found: char as u32, expected: *self }.into())
                }
            },
            None => Error(UnexpectedEndError { pos: cxt.pos }.into()),
        }
    }
}

