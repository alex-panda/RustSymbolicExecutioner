use std::sync::RwLock;

use crate::parser::{ParseContext, Span};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, RwLockReadError};


impl <Ok, Err: From<RwLockReadError>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for RwLock<Child> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        match self.read() {
            Ok(child_ref) => child_ref.do_parse(cxt),
            Err(_) => ParseResult::Panic(RwLockReadError.into()),
        }
    }
    
    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.read() {
            Ok(child_ref) => child_ref.do_parse_span(cxt),
            Err(_) => ParseResult::Panic(RwLockReadError.into()),
        }
    }
}