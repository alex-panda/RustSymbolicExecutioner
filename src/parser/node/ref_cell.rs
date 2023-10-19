use std::cell::RefCell;

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, RefCellReadError};


impl <Ok, Err: From<RefCellReadError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for RefCell<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self.try_borrow() {
            Ok(reference) => reference.parse(store, pos),
            Err(_) => ParseResult::Panic(RefCellReadError.into()),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        match self.try_borrow() {
            Ok(reference) => reference.parse_span(store, pos),
            Err(_) => ParseResult::Panic(RefCellReadError.into()),
        }
    }
}