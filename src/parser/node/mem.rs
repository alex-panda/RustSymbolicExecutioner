use std::{collections::HashMap, cell::RefCell, any::Any};
use crate::parser::{ZSTNode, ParseContext};
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
/// A trait for a type that acts as a memoization table for a `MemNode`.
/// 
/// If `Err(..)` is returned by any of the functions, then the return error will
/// end the parse by being wrapped in a `ParseResult::Panic(..)` and then
/// returned.
/// 
/// It is okay for the implementor to not actually store any value passed to the
/// `mem_set` function or for `mem_get` to return `None` for a value that is
/// actually stored in the memoization table.
/// 
pub trait MemTable<Ok, Err, Pos: ParsePos> {
    /// 
    /// Sets the `ParseResult` for the given key, returning an `Err(..)` if a
    /// `ParseResult::Panic(..)` worthy error occurs.
    /// 
    /// The "key" consists of an ID that is unique for each `Mem` node and then
    /// the key for the position of the parse.
    /// 
    fn mem_set(&self, key: (usize, Pos::Key), value: ParseResult<Ok, Err, Pos>) -> Result<(), Err>;
    /// 
    /// Gets the `ParseResult` for the given key, returning an `Err(..)` if a
    /// `ParseResult::Panic(..)` worthy error occurs.
    /// 
    fn mem_get<O, F: FnOnce(Option<&ParseResult<Ok, Err, Pos>>) -> O>(&self, key: &(usize, Pos::Key), f: F) -> Result<O, Err>;
}

/// 
/// Returns a `Mem` node that will memoize the parse result of its child node for each
/// position that its child parses. This allows for closer-to-linear-time parses
/// as any memoized rule will never parse its child at the same position twice.
/// This is acheived by the `Mem` node parsing its child once at any given
/// position -- the first time the node parses at the position -- and then returning
/// clones of the result all times after the first.
/// 
/// Note: The node ruturned by this function ONLY implements memoization. It
/// does not allow for direct or indirect left recursion. Use `LRec` if you
/// would like a node that both memoizes the parse result and allows for
/// indirect and direct left recursion.
/// 
/// In order to use a `Mem` node, a few things must be held true and kept true:
///  - `Ok` must be `Clone`
///  - `Err` must be `Clone`
///  - `Store` must implement `MemTable<Ok, Err, Pos>` (where `Ok`, `Err`,
///         and `Pos` must match this particular `Mem` node's `Ok`, `Err`, and
///         `Pos` types)
///  - A NEW MEMOIZATION TABLE MUST BE GIVEN AT THE START OF EACH PARSE. Passing
///         in the same memoization table for more than one parse will cause
///         undefined behavior as the memoized values from the last parse will
///         effect the memoized values of the current parse. The same memoization
///         table can only be passed in for more than one parse if either the
///         memoization table's values are cleared between parses or if the
///         parse node is not moved between parses and the `ParseStore`
///         being parsed for both parses is exactly the same.
/// 
/// Depending on the implementation of the `MemTable`, a heap allocator is not
/// necessary for this node to function. This is because, assuming the
/// `MemTable` implementation has a fixed amount of memory and runs out of
/// memory to store its child's results, the `MemTable` implementation can
/// simply stop storing any new parse results passed into the `mem_set`
/// function. Keep in mind, however, that any result not memoized will mean that
/// the next time the node parses at the same position it will parse its child
/// node again rather than returning a clone of the previously-memoized result
/// (because the previously-memoized result was thrown away instead of actually
/// being memoized).
/// 
#[allow(non_snake_case)]
pub fn Mem<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone, Store: ParseStore<Pos, V> + MemTable<Ok, Err, Pos> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> MemNode<Child, Ok, Err, Store, Pos, V> {
    MemNode { byte: 0, child, _zst: ZSTNode::default() }
}

pub struct MemNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone, Store: ParseStore<Pos, V> + MemTable<Ok, Err, Pos> + ?Sized, Pos: ParsePos, V: ParseValue> {
    /// Dummy byte to ensure that every `Mem` node takes up at least 1 byte of
    /// memory and therefore occupies a unique place in memory. That way we can
    /// use the location of the byte as the unique ID of the `Mem` node (at
    /// least for the duration of the parse since the node cannot be moved
    /// during the parse).
    byte: u8, 
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
use zst::ZST;
impl <Ok: Clone, Err: Clone, Store: ParseStore<Pos, V> + MemTable<Ok, Err, Pos> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for MemNode<Child, Ok, Err, Store, Pos, V> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        // The key is the node's unique ID (i.e. the location in memory of the
        // `Mem` node) and the key of the parse position.
        let key = (((&self.byte) as *const _) as usize, cxt.pos.key());

        // Check if this node has previously produced a result at this position.
        // If so, return the previously-produced result.
        match cxt.store.mem_get(&key, |found| Some((found?).clone())) {
            Ok(Some(res)) => return res, // return previously-memoized result
            Ok(None) => { /* do nothing */ }, // the rest of this function's body is this branch's code
            Err(err) => return Panic(err), // return `ParseResult::Panic(..)`-worthy error
        }

        // No previous result produced by child node at this position was
        // memoized so parse the child node now and then save and return its result.
        let child_res = self.child.parse(cxt.clone());

        // return the result of parsing at this position
        match cxt.store.mem_set(key, child_res.clone()) {
            Ok(ok) => child_res,
            Err(err) => Panic(err),
        }
    }
}
