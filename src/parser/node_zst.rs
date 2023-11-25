use std::hash::Hash;

use zst::ZST;

use super::{ParseValue, ParsePos, ParseStore, ParseNode};

#[allow(unused)]
pub struct ZSTNode<Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub(crate) ok: ZST<Ok>,
    pub(crate) err: ZST<Err>,
    pub(crate) store: ZST<Store>,
    pub(crate) pos: ZST<Pos>,
    pub(crate) v: ZST<V>,
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Hash for ZSTNode<Ok, Err, Store, Pos, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        0.hash(state)
    }
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Clone for ZSTNode<Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Default for ZSTNode<Ok, Err, Store, Pos, V> {
    fn default() -> Self {
        Self { ok: Default::default(), err: Default::default(), store: Default::default(), pos: Default::default(), v: Default::default() }
    }
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Eq for ZSTNode<Ok, Err, Store, Pos, V> { }

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> PartialEq for ZSTNode<Ok, Err, Store, Pos, V> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> PartialOrd for ZSTNode<Ok, Err, Store, Pos, V> {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Ord for ZSTNode<Ok, Err, Store, Pos, V> {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}