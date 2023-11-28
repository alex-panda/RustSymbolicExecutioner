use core::fmt::Debug;
use std::{collections::HashMap, cell::RefCell, any::Any};
use crate::parser::{ZSTNode, NoAdvanceError, ParseContext};
use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

/// 
/// A struct that implements `MemTable` such that it can wrap any `Store` to
/// make it implement `MemTable` for all `Ok` and `Err` combinations. This is
/// done by internally storing the values using `dyn Any`. That being said, it
/// requires that `Ok`, `Err`, and `Pos` all have static lifetimes.
/// 
pub struct AnyMemTable<Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub store: Store,
    pub table: RefCell<HashMap<(usize, Pos::Key), Box<dyn Any>>>,
    zst: ZST<V>,
}

impl <Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> AnyMemTable<Store, Pos, V> {
    pub fn new(store: Store) -> Self {
        Self { store, table: RefCell::new(HashMap::new()), zst: ZST::default() }
    }
}

impl <Ok: 'static, Err: 'static, Store: ParseStore<Pos, V>, Pos: ParsePos + 'static, V: ParseValue> MemTable<Ok, Err, Pos> for AnyMemTable<Store, Pos, V> {
    fn mem_set(&self, key: (usize, <Pos as ParsePos>::Key), value: ParseResult<Ok, Err, Pos>) -> Result<(), Err> {
        self.table.borrow_mut().insert(key, Box::new(value));
        Ok(())
    }

    fn mem_get<O, F: FnOnce(Option<&ParseResult<Ok, Err, Pos>>) -> O>(&self, key: &(usize, <Pos as ParsePos>::Key), f: F) -> Result<O, Err> {
        Ok(if let Some(v) = self.table.borrow().get(key) {
            f(v.downcast_ref())
        } else {
            f(None)
        })
    }
}

impl <Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseStore<Pos, V> for AnyMemTable<Store, Pos, V> {
    fn value_at(&self, pos: &mut Pos) -> Option<V> {
        self.store.value_at(pos)
    }
}

/// 
/// A trait for a type that acts as a memoization table for an `LRecNode`.
/// 
pub trait MemTable<Ok, Err, Pos: ParsePos> {
    /// 
    /// Sets the `ParseResult` for the given key, returning an `Err(..)` if a
    /// `ParseResult::Panic(..)` worthy error occurred.
    /// 
    fn mem_set(&self, key: (usize, Pos::Key), value: ParseResult<Ok, Err, Pos>) -> Result<(), Err>;
    /// 
    /// Gets the `ParseResult` for the given key, returning an `Err(..)` if a
    /// `ParseResult::Panic(..)` worthy error occurred.
    /// 
    fn mem_get<O, F: FnOnce(Option<&ParseResult<Ok, Err, Pos>>) -> O>(&self, key: &(usize, Pos::Key), found: F) -> Result<O, Err>;
}

/// 
/// Returns an `LRecNode` that will memoize the parse result of the current
/// position in the parse. This also enables direct left-hand
/// recursion when made the root of each left-recursive subtree of the parse AST.
/// 
/// In order to use an `LRec` node, a few things must be held true and kept true:
///  - `Ok` must be `Clone`
///  - `Err` must be `Clone`
///  - `Store` must implement `MemTable<Ok, Err, Pos>` (where `Ok`, `Err`,
///         and `Pos` types match this particular `LRec` node's `Ok`, `Err`, and
///         `Pos` types)
///  - A new memoization table must be passed in for EACH PARSE. Passing in the same
///         memoization table for more than one parse will cause undefined behavior
///         as the memoized values from the last parse will effect the memoized
///         values of the current parse.
/// 
#[allow(non_snake_case)]
pub fn LRec<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Debug + Clone, Err: Debug + Clone + From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + MemTable<Ok, Err, Pos> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> LRecNode<Child, Ok, Err, Store, Pos, V> {
    LRecNode { dummy: 0, child, _zst: ZSTNode::default() }
}

#[repr(C)]
pub struct LRecNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Debug + Clone, Err: Debug + Clone + From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + MemTable<Ok, Err, Pos> + ?Sized, Pos: ParsePos, V: ParseValue> {
    // Dummy u8 to assure that every `LRec` node takes up at least 1 byte of
    // memory and therefore occupies a unique place in memory that we can use
    // like its ID. (This only works because the byte is before the Child node
    // and `repr(C)` is used to make sure that Rust does not reorder the byte to
    // be after the child node).
    dummy: u8, 
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
use zst::ZST;
impl <Ok: Debug + Clone + 'static, Err: Debug + Clone + From<NoAdvanceError<Pos>> + 'static, Store: ParseStore<Pos, V> + MemTable<Ok, Err, Pos> + ?Sized, Pos: ParsePos + 'static, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for LRecNode<Child, Ok, Err, Store, Pos, V> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        // the key is the node's unique ID (i.e. the location in memory of the
        // `LRecNode`) and the key of the parse position.
        let key = (((self as *const _) as *const ()) as usize, cxt.pos.key());

        // Check if this node has previously produced a result at this position.
        // If so, return the previously-produced result.
        match cxt.store.mem_get(&key, |found| Some((found?).clone())) {
            Ok(Some(res)) => return res, // return previously-memoized result
            Ok(None) => { /* do nothing */ }, // the rest of this function's body is this branch's code
            Err(err) => return Panic(err), // return `ParseResult::Panic(..)`-worthy error
        }

        // No previous result so prime the parse table with a failure
        // result. That way all recursive parses of this node at this
        // position will fail, meaning that only terminal nodes
        // (non-recursive nodes) can parse at this position.
        if let Err(err) = cxt.store.mem_set(key.clone(), Error(Err::from(NoAdvanceError { pos: cxt.pos.clone() }))) {
            return Panic(err);
        }

        // save the current position as the last advanced position
        let mut last_advance = cxt.pos.clone();

        // Continually try to parse the child at this position. The first parse,
        // because of the error memoized at this position, will only allow
        // terminal nodes to parse at this position. Then all subsequent parses
        // at this position can use that parsed terminal node as their left child
        // node and build from there. We stop parsing the child node when it no
        // longer advances the parse anymore or it produces an error.
        loop {
            // parse the child node and handle its result
            match self.child.do_parse(cxt.clone()) {
                Okay(ok, adv) => {
                    if adv.key() == last_advance.key() {
                        // did not advance the parse
                        break;
                    }

                    // advanced the parse

                    last_advance = adv.clone(); // save the new position
                    if let Err(err) = cxt.store.mem_set(key.clone(), Okay(ok, adv)) { // save the result of the advance
                        return Panic(err);
                    }
                },
                Error(_) => { break; },
                Panic(e) => { return Panic(e); },
            }
        }

        // return the result of parsing at this position
        match cxt.store.mem_get(&key, |found| {
            // handle gotten value
            match found {
                Some(res) => res.clone(), // we found value, so just clone and return it
                None => panic!("`LRec` node stored a result earlier but did not receive it when getting it from the memoization table now"),
            }

        // we must handle the case where `mem_get` returned an `Err`
        }) {
            Ok(ok) => ok,
            Err(err) => Panic(err),
        }
    }
}
