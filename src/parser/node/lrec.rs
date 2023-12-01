use std::{cell::RefCell, rc::Rc, collections::{HashSet, HashMap}, hash::Hash, any::Any};

use crate::parser::{ZSTNode, ParseContext, LRecError};
use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

pub struct TLRecMemTable<Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub store: Store,
    pub mem_table: RefCell<HashMap<(usize, Pos::Key), R<Entry<Pos>>>>,
    pub lr_stack: RefCell<Vec<R<LR>>>,
    pub lr_head: RefCell<HashMap<Pos::Key, R<Head>>>,
    _v: ZST<V>
}

impl <Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> TLRecMemTable<Store, Pos, V> {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            mem_table: RefCell::new(HashMap::new()),
            lr_stack: RefCell::new(Vec::new()),
            lr_head: RefCell::new(HashMap::new()),
            _v: ZST::default()
        }
    }
}


impl <Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> LRecMemTable<Pos> for TLRecMemTable<Store, Pos, V> {
    fn lr_mem_get<F: FnOnce(Option<&R<Entry<Pos>>>) -> O, O>(&self, key: &(usize, Pos::Key), f: F) -> O {
        if let Some(v) = self.mem_table.borrow().get(key) {
            f(Some(v))
        } else {
            f(None)
        }
    }

    fn lr_mem_set(&self, key: (usize, Pos::Key), lr: R<Entry<Pos>>) {
        self.mem_table.borrow_mut().insert(key, lr);
    }

    fn lr_stack_push(&self, lr: R<LR>) {
        self.lr_stack.borrow_mut().push(lr);
    }

    fn lr_stack_pop(&self) -> Option<R<LR>> {
        self.lr_stack.borrow_mut().pop().map(|lr|lr.clone())
    }

    fn lr_stack_reverse_iter<F: FnMut(&R<LR>) -> bool>(&self, mut f: F) {
        for item in self.lr_stack.borrow().iter().rev() {
            if f(item) {
                break
            }
        }
    }

    fn lr_head_set(&self, pos: Pos::Key, head: R<Head>) {
        let _ = self.lr_head.borrow_mut().insert(pos, head);
    }

    fn lr_head_get(&self, pos: &Pos::Key) -> Option<R<Head>> {
        self.lr_head.borrow().get(pos).map(|h|h.clone())
    }

    fn lr_head_rem(&self, pos: &Pos::Key) {
        let _ = self.lr_head.borrow_mut().remove(pos);
    }
}

impl <Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseStore<Pos, V> for TLRecMemTable<Store, Pos, V> {
    fn value_at(&self, pos: &mut Pos) -> Option<V> {
        self.store.value_at(pos)
    }
}

/// 
/// A trait for implementing all the features than an `LRec` node needs during
/// each parse.
/// 
/// Overall, the `LRec` node needs a few things:
///  - A memoiziation map. This is for memoizing the results of the child parse
///    node and returning clones of the child parse node's result whenever the
///    child would otherwise parse the same position more than once.
///  - An `LR` stack. Each `LR` keeps track of what the current stack of `LRec`
///    nodes are working on parsing the current position.
///  - A `Head` stack. Each `Head` keeps track of what node is the overarching
///    head of the current position i.e. the node that is actually parsing at
///    the current position.
/// 
/// The methods provided by this trait allow these things to be accessed in a
/// programatic way that the implementor of the trait can control the concrete
/// storage of.
/// 
/// This node requires heap allocations to be made regardless of how
/// `LRecMemTable` is implemented.
/// 
pub trait LRecMemTable<Pos: ParsePos> {

    // --- Memoization Methods ---

    fn lr_mem_get<F: FnOnce(Option<&R<Entry<Pos>>>) -> O, O>(&self, key: &(usize, Pos::Key), f: F) -> O;
    fn lr_mem_set(&self, key: (usize, Pos::Key), lr: R<Entry<Pos>>);

    // --- LR Stack Methods ---

    fn lr_stack_push(&self, lr: R<LR>);
    fn lr_stack_pop(&self) -> Option<R<LR>>;
    fn lr_stack_reverse_iter<F: FnMut(&R<LR>) -> bool>(&self, f: F);

    // --- Head Methods ---

    fn lr_head_set(&self, pos: Pos::Key, head: R<Head>);
    fn lr_head_get(&self, pos: &Pos::Key) -> Option<R<Head>>;
    fn lr_head_rem(&self, pos: &Pos::Key);
}

pub struct R<T> {
    inner: Rc<RefCell<T>>,
}

impl <T> R<T> {
    pub fn new(v: T) -> Self {
        Self { inner: Rc::new(RefCell::new(v)) }
    }

    pub fn with<O, F: FnOnce(&T) -> O>(&self, f: F) -> O {
        f(&*self.inner.borrow())
    }

    pub fn with_mut<O, F: FnOnce(&mut T) -> O>(&self, f: F) -> O {
        f(&mut *self.inner.borrow_mut())
    }

    pub fn mem_loc(&self) -> usize {
        (self.inner.as_ptr() as *const ()) as usize
    }
}

impl R<Head> {
    fn nterm(&self) -> usize {
        self.with(|h|h.nterm)
    }

    fn set_rule(&self, rule: usize) {
        self.with_mut(|h|h.nterm = rule)
    }

    fn involved_set_insert(&self, rule: usize) -> bool {
        self.with_mut(|s|s.involved_set.insert(rule))
    }
    
    fn involved_set_contains(&self, rule: usize) -> bool {
        self.with(|s|s.involved_set.contains(&rule))
    }

    fn involved_set_remove(&self, rule: usize) {
        self.with_mut(|s|s.involved_set.remove(&rule));
    }

    fn evaluation_set_insert(&self, rule: usize) -> bool {
        self.with_mut(|s|s.evaluation_set.insert(rule))
    }

    fn evaluation_set_contains(&self, rule: usize) -> bool {
        self.with(|s|s.evaluation_set.contains(&rule))
    }

    fn evaluation_set_remove(&self, rule: usize) {
        self.with_mut(|s|s.evaluation_set.remove(&rule));
    }

    fn set_evaluation_set(&self, set: HashSet<usize>) {
        self.with_mut(|h|h.evaluation_set = set);
    }

    fn set_involved_set(&self, set: HashSet<usize>) {
        self.with_mut(|h|h.involved_set = set);
    }

    fn evaluation_set(&self) -> HashSet<usize> {
        self.with(|h|h.evaluation_set.clone())
    }

    fn involved_set(&self) -> HashSet<usize> {
        self.with(|h|h.involved_set.clone())
    }
}

impl <T> Clone for R<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

#[derive(Clone)]
pub enum EntryVal {
    Error(Rc<dyn Any>),
    Okay(Rc<dyn Any>),
    LR(R<LR>),
}

#[derive(Clone)]
pub struct Entry<Pos> {
    pub res: EntryVal,
    pub pos: Pos,
}

impl <Pos: ParsePos> R<Entry<Pos>> {
    pub fn with_lr<O, F: FnOnce(&LR) -> O>(&self, f: F) -> Option<O> {
        self.with(|e| {
            match &e.res {
                EntryVal::Error(_) => None,
                EntryVal::Okay(_) => None,
                EntryVal::LR(lr) => Some(lr.with(|lr|f(lr))),
            }
        })
    }

    pub fn lr(&self) -> Option<R<LR>> {
        self.with(|e| {
            match &e.res {
                EntryVal::Error(_) => None,
                EntryVal::Okay(_) => None,
                EntryVal::LR(lr) => Some(lr.clone()),
            }
        })
    }

    pub fn lr_head(&self) -> Option<R<Head>> {
        match self.with_lr(|lr|lr.head.clone()) {
            Some(head) => head,
            None => None,
        }
    }

    pub fn lr_seed(&self) -> Option<Result<Rc<dyn Any>, Rc<dyn Any>>> {
        match self.with_lr(|lr|lr.seed.clone()) {
            Some(seed) => seed,
            None => None,
        }
    }

    pub fn lr_nterm(&self) -> Option<usize> {
        self.with_lr(|lr| lr.nterm)
    }

    pub fn set_pos(&self, pos: Pos) {
        self.with_mut(|e| e.pos = pos)
    }

    pub fn set_res(&self, res: EntryVal) {
        self.with_mut(|e| e.res = res)
    }

    pub fn res(&self) -> EntryVal {
        self.with(|e| e.res.clone())
    }

    pub fn pos(&self) -> Pos {
        self.with(|e|e.pos.clone())
    }
}

#[derive(Clone)]
pub struct LR {
    pub seed: Option<Result<Rc<dyn Any>, Rc<dyn Any>>>,
    pub nterm: usize,
    pub head: Option<R<Head>>,
}

impl R<LR> {
    pub fn seed(&self) -> Option<Result<Rc<dyn Any>, Rc<dyn Any>>> {
        self.with(|lr|lr.seed.clone())
    }

    pub fn head(&self) -> Option<R<Head>> {
        self.with(|lr|lr.head.clone())
    }

    pub fn nterm(&self) -> usize {
        self.with(|lr|lr.nterm)
    }

    pub fn set_seed(&self, seed: Option<Result<Rc<dyn Any>, Rc<dyn Any>>>) {
        self.with_mut(|lr|lr.seed = seed)
    }

    pub fn set_head(&self, head: Option<R<Head>>) {
        self.with_mut(|lr|lr.head = head)
    }

    pub fn set_nterm(&self, nterm: usize) {
        self.with_mut(|lr|lr.nterm = nterm)
    }
}

pub struct Head {
    pub nterm: usize,
    pub involved_set: HashSet<usize>,
    pub evaluation_set: HashSet<usize>,
}

/// 
/// Returns an `LRec` node that will memoize the result of parsing its child
/// node for each position it parses. This node also enables both direct and
/// indirect left recursion when made the root of each left-recursive
/// subtree of the parse AST. If you would like to just memoize the result of
/// the child for each position it parses without the overhead required to
/// enable left-recursion, use the `Mem` node instead.
/// 
/// In order to use an `LRec` node, a few things must be held true and kept true:
///  - `Ok` must be `Clone`
///  - `Err` must be `Clone`
///  - `Pos::Key` must be `Ord`
///  - `Store` must also implement `LRecMemTable<Ok, Err, Pos>` (where `Ok`, `Err`,
///         and `Pos` types match this particular `LRec` node's `Ok`, `Err`, and
///         `Pos` types)
///  - ALL DIRECTLY OR INDIRECTLY LEFT-RECURSIVE `LRec` NODES MUST SHARE THE SAME
///         `LRecMemTable`. To do this, all `LRec` nodes need to have the same `Ok` and error `Err` types.
///         This can be achieved by translating (when necessary)
///         the child parse type of each `LRec` node to a common type (such as an enum
///         or `Box<dyn Any>`) using a `Map` node and then from that common parse type
///         to the desired one when it is returned from the `LRec` node during
///         the parse using another `Map` node.
///         An example of this can be found in the `ALRec` node as it translates
///         the child parse node's result `T: Clone` to a `Box<dyn Any>`, passes that to
///         the `LRec` node, and then translates the `Box<dyn Any>` back to a `T: Clone`
///         type whenever it is returned from the `LRec` node.
///  - A NEW MEMOIZATION TABLE MUST BE PASSED IN AT THE START OF EACH PARSE.
///         Passing in the same memoization table for more than one parse will
///         cause undefined behavior as the memoized values from the last parse
///         will effect the memoized values of the current parse. The same
///         memoization table can only be reused if either its values are
///         cleared before each parse or if the `ParseStore` being parsed holds
///         exactly the same values for each consecutive parse (although parsing
///         equal `ParseStore`s with the same `MemTable` is only
///         allowed if the node is not moved between parses).
///  - The node must NOT be moved during the parse (using a RefCell or otherwise).
/// 
#[allow(non_snake_case)]
pub fn LRec<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone + From<LRecError<Pos>>, Store: ParseStore<Pos, V> + LRecMemTable<Pos> + ?Sized, Pos: ParsePos<Key = K>, V: ParseValue, K: Clone + Hash + Eq + Ord>(child: Child) -> LRecNode<Child, Ok, Err, Store, Pos, V, K> {
    LRecNode { byte: 0, child, _zst: ZSTNode::default() }
}

pub struct LRecNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone + From<LRecError<Pos>>, Store: ParseStore<Pos, V> + LRecMemTable<Pos> + ?Sized, Pos: ParsePos<Key=K>, V: ParseValue, K: Clone + Hash + Eq + Ord> {
    /// Dummy byte to ensure that every `LRec` node takes up at least 1 byte of
    /// memory and therefore occupies a unique place in memory. That way we can
    /// use the location of the byte as the unique ID of the `LRec` node (at
    /// least for the duration of the parse since it cannot be moved during the
    /// parse).
    byte: u8, 
    pub child: Child,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
use zst::ZST;
impl <Ok: Clone + 'static, Err: Clone + From<LRecError<Pos>> + 'static, Store: ParseStore<Pos, V> + LRecMemTable<Pos> + ?Sized, Pos: ParsePos<Key = K>, V: ParseValue, K: Clone + Hash + Eq + Ord, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for LRecNode<Child, Ok, Err, Store, Pos, V, K> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        // Get the unique ID of this `LRec` node.
        let nterm = ((&self.byte) as *const _) as usize;

        println!("{}: LRec", nterm);

        // the key is the node's unique ID (i.e. the location in memory of the
        // `LRecNode`) and the key of the parse position.
        let key = (nterm, cxt.pos.key());

        let table: &Store = cxt.store;

        // --- Grow ---

        let lrgrow = |entry: R<Entry<Pos>>, head: R<Head>| {
            println!("{}: lrgrow", nterm);
            table.lr_head_set(cxt.pos.key(), head.clone());

            loop {
                head.set_evaluation_set(head.involved_set());

                match self.child.parse(cxt.clone()) {
                    Okay(ok, pos) => {
                        if pos.key() <= entry.pos().key() {
                            break;
                        }

                        entry.set_res(EntryVal::Okay(Rc::new(ok)));
                        entry.set_pos(pos);
                    },
                    Error(_) => break,
                    Panic(err) => return Panic(err),
                }
            }

            table.lr_head_rem(&cxt.pos.key());

            match entry.res() {
                EntryVal::Error(err) => match err.downcast_ref::<Err>() {
                    Some(err) => Error(err.clone()),
                    None => panic!("lrgrow of `LRec` node expected `Err`"),
                },
                EntryVal::Okay(ok) => match ok.downcast_ref::<Ok>() {
                    Some(ok) => Okay(ok.clone(), entry.pos()),
                    None => panic!("lrgrow of `LRec` node expected `Ok`"),
                },
                EntryVal::LR(_) => panic!("lrgrow of `LRec` node expected `ParseResult`, not `LR`"),
            }
        };

        let lrstart = |lr: R<LR>| {
            println!("{}: lrstart", nterm);

            if lr.head().is_none() {
                lr.set_head(Some(R::new(Head { nterm, involved_set: HashSet::new(), evaluation_set: HashSet::new() })));
            }

            table.lr_stack_reverse_iter(|item| {
                if (item.head().is_some() && lr.head().is_some()) && (item.head().map(|h|h.mem_loc()) == lr.head().map(|h|h.mem_loc())) {
                    return true; // break
                }

                item.set_head(lr.head());
                lr.head().map(|h|h.involved_set_insert(item.nterm()));

                return false; // continue
            });
        };

        // --- Answer ---

        let lranswer = |entry: R<Entry<Pos>>| {
            println!("{}: lranswer", nterm);

            let head = match entry.lr_head() {
                Some(head) => head,
                None => panic!("`LRec` parse node expected the LR to have a head!"),
            };

            // if the node is this node than we return its result
            if head.nterm() != nterm {
                return match entry.res() {
                    EntryVal::Error(_) => panic!("lranswer of `LRec` expected seed, found `EntryVal::Error`"),
                    EntryVal::Okay(_) => panic!("lranswer of `LRec` expected seed, found `EntryVal::Okay`"),
                    EntryVal::LR(lr) => match lr.seed() {
                        Some(Ok(ok)) => Okay(
                            match ok.downcast_ref::<Ok>() {
                                Some(ok) => ok.clone(),
                                None => panic!("lranswer of `LRec` expected `Ok` type for `EntryVal`"),
                            },
                            entry.pos()
                        ),
                        Some(Err(err)) => Error(
                            match err.downcast_ref::<Err>() {
                                Some(err) => err.clone(),
                                None => panic!("lranswer of `LRec` expected `Err` type for `EntryVal`"),
                            },
                        ),
                        None => panic!("lranswer expected seed, found `None`"),
                    },
                };
            }

            // we are looking at a child node of this node

            // set the entry's result to be the seed of its lr
            entry.set_res(match entry.lr_seed() {
                Some(Ok(ok)) => EntryVal::Okay(ok),
                Some(Err(err)) => EntryVal::Error(err),
                None => panic!("lranswer of `LRec` expected the entry to contain an `LR`")
            });

            // if the entry is an error, return the error
            if let EntryVal::Error(err) = entry.res() {
                return Error(
                    match err.downcast_ref::<Err>() {
                        Some(err) => err.clone(),
                        None => panic!("lranswer of `LRec` node expected `Err` type"),
                    }
                );
            }

            return lrgrow(entry, head);
        };

        // --- Recall ---

        // a closure that checks to see if there is any previous result at this
        // position or if we are in the middle of left recursion
        let recall = |nterm: usize, pos: Pos| {
            println!("{}: recall", nterm);
            // get the memoized result of any past parse of this node at this parse location
            let entry = table.lr_mem_get(&key, |res| Some((res?).clone()));
            // get the current head of this parse location (if there is one)
            let head = table.lr_head_get(&pos.key());
            
            // if no head then return the entry (which may be None)
            let head = match head {
                Some(head) => head,
                None => return Ok(entry),
            };

            // if entry is None but the term is not involved with the head's parse/recursion then return None
            if entry.is_none() && ((head.nterm() != nterm) && ((!head.involved_set_contains(nterm)))) {
                return Ok(None);
            }

            // at this point, there must be a head and the current node must be involved in it

            // if the node should be evaluated, then evaluate it and return its
            // parse result, otherwise just return the entry
            if head.evaluation_set_contains(nterm) {

                // remove the node from the evaluate nodes (since we are evaluating now)
                head.evaluation_set_remove(nterm);

                // evaluate the node and return its result
                Ok(match self.child.parse(cxt.with_pos(pos.clone())) {
                    Okay(ok, pos) => Some(R::new(Entry { res: EntryVal::Okay(Rc::new(ok)), pos })),
                    Error(err) => Some(R::new(Entry { res: EntryVal::Error(Rc::new(err)), pos })),
                    Panic(err) => return Err(err),
                })
            } else {
                // return entry (it may be None)
                Ok(entry)
            }
        };

        // --- Apply Rule ---

        // try to recall any previous parse at this position
        let entry: Option<R<Entry<Pos>>> = match recall(nterm, cxt.pos.clone()) {
            Ok(entry) => entry, // we had a result
            Err(err) => return Panic(err), // if Err, then Panic
        };

        if entry.is_none() {
            println!("{}: entry.is_none()", nterm);
            // create a new LR
            let lr = R::new(LR { nterm, seed: None, head: None });

            // push the LR onto the stack
            table.lr_stack_push(lr.clone());

            // create a new entry with an LR that will fail any child parses of this node at this position
            let entry = R::new(Entry { res: EntryVal::LR(lr.clone()), pos: cxt.pos.clone() });

            // mark this as a failure in the mem table so that any child parses
            // of this node will return failure instead of left-recursing
            table.lr_mem_set(key, entry.clone());

            // parse child node
            let res = self.child.parse(cxt.clone());

            // pop the `LR`
            table.lr_stack_pop();

            // get the current position
            let (res, pos) : (Result<Rc<dyn Any>, Rc<dyn Any>>, Pos) = match res {
                Okay(ok, pos) => (Ok(Rc::new(ok)), pos.clone()),
                Error(err) => (Err(Rc::new(err)), cxt.pos.clone()),
                Panic(err) => return Panic(err),
            };

            // set the entry's current position
            entry.set_pos(pos.clone());

            // get the lr if there is an LR and it has a head
            if lr.head().is_some() {
                lr.set_seed(Some(res.clone()));
                return lranswer(entry);
            }

            entry.set_res(match res {
                Ok(ok) => EntryVal::Okay(ok),
                Err(err) => EntryVal::Error(err),
            });

        } else if let Some((entry, lr)) = entry.clone()
                .map(|e| e.clone().lr().map(|lr| (e, lr))).flatten() {

            // left-recursion
            lrstart(lr.clone());

            // map the entry to a ParseResult and return the ParseResult
            return match lr.seed() {
                Some(Ok(ok)) => Okay(
                    match ok.downcast_ref::<Ok>() {
                        Some(ok) => ok.clone(),
                        None => panic!("`LRec` expect `Ok` type"),
                    },
                    entry.pos()
                ),
                Some(Err(err)) => Error(
                    match err.downcast_ref::<Err>() {
                        Some(err) => err.clone(),
                        None => panic!("`LRec` expected `Err` type"),
                    }
                ),
                None => Panic(Err::from(LRecError { pos: cxt.pos.clone() })),
            }
        }

        // map the entry to a ParseResult and then return the ParseResult
        match entry {
            Some(entry) => match entry.res() {
                EntryVal::Error(err) => match err.downcast_ref::<Err>() {
                    Some(err) => Error(err.clone()),
                    None => panic!("`LRec` node expected `Err`"),
                },
                EntryVal::Okay(ok) => match ok.downcast_ref::<Ok>() {
                    Some(ok) => Okay(ok.clone(), entry.pos()),
                    None => panic!("`LRec` node expected `Ok`"),
                },
                EntryVal::LR(_) => panic!("`LRec` node expected `ParseResult`, not `LR`"),
            },
            None => Error(Err::from(LRecError { pos: cxt.pos.clone() })),
        }
    }
}
