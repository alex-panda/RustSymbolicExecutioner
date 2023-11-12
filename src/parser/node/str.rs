use crate::parser::{Span, UnexpectedEndError, UnexpectedValueError};

use super::super::{ParseNode, ParsePos, ParseStore, ParseResult};

use ParseResult::*;
impl <Err: From<UnexpectedEndError<Pos>> + From<UnexpectedValueError<Pos, char>>, Store: ParseStore<Pos, char>, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for str {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = pos.clone();
        for target_char in self.chars() {
            match store.value_at(&mut curr_pos) {
                Some(actual_char) => {
                    if target_char != actual_char {
                        return Error(UnexpectedValueError { pos, found: actual_char, expected: target_char }.into());
                    }
                },
                None => return Error(UnexpectedEndError { pos: curr_pos }.into()),
            }
        }

        OkayAdvance(Span::new(pos, curr_pos.clone()), curr_pos)
    }
}


impl <Err: From<UnexpectedEndError<Pos>> + From<UnexpectedValueError<Pos, char>>, Store: ParseStore<Pos, char>, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for String {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        <&str as ParseNode<Span<Pos>, Err, Store, Pos, char>>::parse(&self.as_str(), store, pos)
    }
}
