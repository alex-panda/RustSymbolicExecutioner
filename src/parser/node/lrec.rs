//
//use std::{collections::{HashMap, hash_map::DefaultHasher}, any::Any, cell::{OnceCell, RefCell}, hash::{Hash, Hasher}};
//
//use crate::parser::{ZSTNode, Span};
//
//use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};
//
//thread_local! {
//    static TABLE: OnceCell<RefCell<HashMap<(usize, usize, usize), Box<dyn Any>>>> = OnceCell::new();
//}
//
//#[allow(non_snake_case)]
//pub fn LeftRec<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> LeftRecNode<Child, Ok, Err, Store, Pos, V> {
//    LeftRecNode { dummy: 0, child, _zst: ZSTNode::default() }
//}
//
//#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
//pub struct LeftRecNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
//    /// A Dummy `u8`` is used to assure that each `LeftRecNode` takes up at least a single byte of memory and therefore exists in a unique part of memory.
//    pub dummy: u8,
//    pub child: Child,
//    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
//}
//
//use ParseResult::*;
//impl <Ok: Clone, Err: Clone, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for LeftRecNode<Child, Ok, Err, Store, Pos, V> {
//    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
//        let key = ((store as *const _) as usize, (self as *const _) as usize, pos.key());
//        let out = TABLE.with(|table| {
//            let table = table.get_or_init(|| RefCell::new(HashMap::new())).borrow_mut();
//            table.get(&key).map(|v|v.downcast_ref::<ParseResult<Ok, Err, Pos>>().map(|v|v.clone()).unwrap())
//        });
//
//        if let Some(p) = out {
//            return p;
//        }
//
//        let res = self.child.parse(store, pos);
//    }
//    
//    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
//        let key = ((store as *const _) as usize, (self as *const _) as usize);
//        match self.child.parse_span(store, pos.clone()) {
//            Okay(_) => Okay(Span::new(pos.clone(), pos)),
//            OkayAdvance(_, advance) => Okay(Span::new(pos, advance)),
//            Error(error) => Error(error),
//            Panic(error) => Panic(error),
//        }
//    }
//}
//
//