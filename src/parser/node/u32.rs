use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError};

use ParseResult::*;

impl <Err: From<UnexpectedValueError<Pos, u32>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, char>, Pos: ParsePos> ParseNode<char, Err, Store, Pos, char> for u32 {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<char, Err, Pos> {
        let mut curr_pos = pos.clone();
        match store.value_at(&mut curr_pos) {
            Some(char) => {
                if char as u32 == *self {
                    OkayAdvance(char, curr_pos)
                } else {
                    Error(UnexpectedValueError { pos, found: char as u32, expected: *self }.into())
                }
            },
            None => Error(UnexpectedEndError { pos }.into()),
        }
    }
}

