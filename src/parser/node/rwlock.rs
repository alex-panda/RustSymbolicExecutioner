use std::sync::RwLock;

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, RwLockReadError};


impl <Ok, Err: From<RwLockReadError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for RwLock<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        match self.read() {
            Ok(child_ref) => child_ref.parse(store, pos),
            Err(_) => ParseResult::Panic(RwLockReadError.into()),
        }
    }
}