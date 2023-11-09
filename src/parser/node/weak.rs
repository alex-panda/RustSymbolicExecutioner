use std::{rc::{self}, sync::{self}};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, Span};


impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for rc::Weak<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).parse(store, pos),
            None => panic!("expected `std::rc::Weak`'s contained `StackParseNode` to still be alive"),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).parse_span(store, pos),
            None => panic!("expected `std::rc::Weak`'s contained `ParseNode` to still be alive"),
        }
    }
}


impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for sync::Weak<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).parse(store, pos),
            None => panic!("expected `std::sync::Weak`'s contained `ParseNode` to still be alive"),
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).parse_span(store, pos),
            None => panic!("expected `std::sync::Weak`'s contained `ParseNode` to still be alive"),
        }
    }
}