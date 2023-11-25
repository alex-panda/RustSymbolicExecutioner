use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError, Span};


use ParseResult::*;

impl <Err: From<UnexpectedValueError<Pos, char>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, char>, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for u8 {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = pos.clone();
        match store.value_at(&mut curr_pos) {
            Some(char) => {
                if char == *self as _ {
                    Okay(Span::new(pos, curr_pos.clone()), curr_pos)
                } else {
                    Error(UnexpectedValueError { pos, found: char, expected: *self as _ }.into())
                }
            },
            None => Error(UnexpectedEndError { pos }.into()),
        }
    }
}
