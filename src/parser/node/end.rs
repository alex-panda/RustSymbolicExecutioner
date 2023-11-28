use crate::parser::{ExpectedEndError, ParseContext};

use super::super::{ParseValue, ParseStore, ParsePos, ParseNode, ParseResult};

#[allow(non_snake_case)]
pub fn End() -> EndNode {
    EndNode
}

pub struct EndNode;

impl <Err: From<ExpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for EndNode {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        match cxt.peek() {
            Some(_) => ParseResult::Error(ExpectedEndError { pos: cxt.pos }.into()),
            None => ParseResult::Okay((), cxt.pos),
        }
    }
}