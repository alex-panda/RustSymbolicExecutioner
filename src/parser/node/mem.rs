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
    _zst: ZST<V>,
}

impl <Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> AnyMemTable<Store, Pos, V> {
    pub fn new(store: Store) -> Self {
        Self { store, table: RefCell::new(HashMap::new()), _zst: ZST::default() }
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
/// If `Err(err)` is returned by any of this trait's functions, then the return
/// error will end the parse by being wrapped in a `ParseResult::Panic(err)` and
/// then returned.
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
/// position that its child parses at. This allows parses to be closer to linear time
/// as any memoized rule will never parse its child at the same position twice.
/// This is acheived by the `Mem` node parsing its child once at the current parse
/// position, saving the node in the memoization table, and then returning a
/// clone of the memoized result every time this node parses at the saved parse
/// position.
/// 
/// Note: The node returned by this function ONLY implements memoization. It
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
///         table can only be passed in for more than one parse if the
///         memoization table's values are cleared between parses.
/// 
/// Depending on the implementation of the memoization table, a heap allocator
/// is not necessary for this node to function. This is because this node does
/// not require that its memoization table be consistent. The memoization table
/// is allowed to forget values and/or fail to store values. Because of this, a
/// fixed-sized implementation of the memoization table is possible as it can
/// simply stop storing new parse results when it runs out of memory to do so.
/// 
/// In addition, this node (unlike the `LRec` node) does not require that all of
/// its kind share the same memoization table. As such, the `ParseStore` can
/// implement `MemTable` for every `Ok` and `Err` type combination required by
/// the parse tree, therefore removing any need for dynamic allocation/dispatch
/// that would otherwise be necessary so that one `MemTable` can hold
/// multiple `Ok` and `Err` type combinations.
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
            Ok(_) => child_res,
            Err(err) => Panic(err),
        }
    }
}
