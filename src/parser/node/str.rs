use crate::parser::{Span, UnexpectedEndError, UnexpectedValueError, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseResult};

use ParseResult::*;
impl <Err: From<UnexpectedEndError<Pos>> + From<UnexpectedValueError<Pos, char>>, Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for str {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, char>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();
        for target_char in self.chars() {
            match cxt.store.value_at(&mut curr_pos) {
                Some(actual_char) => {
                    if target_char != actual_char {
                        return Error(UnexpectedValueError { pos: cxt.pos, found: actual_char, expected: target_char }.into());
                    }
                },
                None => return Error(UnexpectedEndError { pos: curr_pos }.into()),
            }
        }

        Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
    }
}


impl <Err: From<UnexpectedEndError<Pos>> + From<UnexpectedValueError<Pos, char>>, Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for String {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, char>) -> ParseResult<Span<Pos>, Err, Pos> {
        ParseNode::do_parse(self.as_str(), cxt)
    }
}
