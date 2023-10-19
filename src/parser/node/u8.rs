use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError};


use ParseResult::*;

impl <Err: From<UnexpectedValueError<Pos, char>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, char>, Pos: ParsePos> ParseNode<char, Err, Store, Pos, char> for u8 {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<char, Err, Pos> {
        let mut curr_pos = pos.clone();
        match store.value_at(&mut curr_pos) {
            Some(char) => {
                if char == *self as _ {
                    OkayAdvance(char, curr_pos)
                } else {
                    Error(UnexpectedValueError { pos, found: char, expected: *self as _ }.into())
                }
            },
            None => Error(UnexpectedEndError { pos }.into()),
        }
    }
}
