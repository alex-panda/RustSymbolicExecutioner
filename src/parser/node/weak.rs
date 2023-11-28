use std::{rc::{self}, sync::{self}};

use crate::parser::ParseContext;

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, Span};


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for rc::Weak<Child> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).do_parse(cxt),
            None => panic!("expected `std::rc::Weak`'s contained `StackParseNode` to still be alive"),
        }
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).do_parse_span(cxt),
            None => panic!("expected `std::rc::Weak`'s contained `ParseNode` to still be alive"),
        }
    }
}


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for sync::Weak<Child> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).do_parse(cxt),
            None => panic!("expected `std::sync::Weak`'s contained `ParseNode` to still be alive"),
        }
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.upgrade() {
            Some(rc) => (*rc).do_parse_span(cxt),
            None => panic!("expected `std::sync::Weak`'s contained `ParseNode` to still be alive"),
        }
    }
}