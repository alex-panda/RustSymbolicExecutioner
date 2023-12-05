use crate::parser::{ExpectedEndError, ParseContext};

use super::super::{ParseValue, ParseStore, ParsePos, ParseNode, ParseResult};

/// 
/// Returns a node that only parses successfully if there is not parse value at
/// the current position (i.e. we are at or past the end of the material being
/// parsed).
/// 
#[allow(non_snake_case)]
pub fn End() -> EndNode {
    EndNode
}

pub struct EndNode;

impl <Err: From<ExpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for EndNode {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        match cxt.peek() {
            Some(_) => ParseResult::Error(ExpectedEndError { pos: cxt.pos }.into()),
            None => ParseResult::Okay((), cxt.pos),
        }
    }
}