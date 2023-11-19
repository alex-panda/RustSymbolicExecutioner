//
//use std::{collections::HashMap, any::Any, cell::{OnceCell, RefCell}, rc::Rc};
//
//use crate::parser::{ZSTNode, Span};
//
//use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};
//
//#[allow(non_snake_case)]
//pub fn LeftRec<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> LeftRecNode<Child, Ok, Err, Store, Pos, V> {
//    LeftRecNode { dummy: 0, child, _zst: ZSTNode::default() }
//}
//
//#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
//pub struct LeftRecNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Clone, Err: Clone, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
//    /// A Dummy `u8`` is used to assure that each `LeftRecNode` takes up at
//    /// least a single byte of memory and therefore exists in a unique address
//    /// in memory.
//    pub dummy: u8,
//    pub child: Child,
//    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
//}
//
//use ParseResult::*;
//impl <Ok: Clone + 'static, Err: Clone + 'static, Store: ParseStore<Pos, V>, Pos: ParsePos + 'static, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for LeftRecNode<Child, Ok, Err, Store, Pos, V> {
//    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
//        let key = ((self as *const _) as usize, pos.key());
//
//        if let Some(p) = out {
//            return p;
//        }
//
//        let mut pos = pos;
//        loop {
//            
//
//            let res = self.child.parse(store, pos.clone());
//        }
//    }
//}
//
//type LR = Rc<RefCell<LRData>>;
//
//struct LRData {
//    seed: Box<dyn Any>,
//    rule_id: usize,
//    head: Option<LR>,
//    next: Option<LR>,
//}
//
//trait LRecEnabled {
//    fn lr_stack_push(&self, lr: LR);
//    fn mem_insert(&self, pos: usize, lr: LR);
//
//    fn mem_get(&self, key: (usize, usize)) -> &dyn Any;
//}
//