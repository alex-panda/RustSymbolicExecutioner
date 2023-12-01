use std::cell::RefCell;

use crate::parser::ParseContext;

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, RefCellReadError};


impl <Ok, Err: From<RefCellReadError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for RefCell<Child> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        match self.try_borrow() {
            Ok(reference) => reference.parse(cxt),
            Err(_) => ParseResult::Panic(RefCellReadError.into()),
        }
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        match self.try_borrow() {
            Ok(reference) => reference.parse_span(cxt),
            Err(_) => ParseResult::Panic(RefCellReadError.into()),
        }
    }
}
