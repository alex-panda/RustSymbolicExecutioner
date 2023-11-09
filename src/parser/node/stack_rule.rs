use std::cell::RefCell;
use crate::parser::{Span, ParsePos, ParseStore, ParseValue, ParseNode, ParseResult, ZSTNode};

/// 
/// A stack rule i.e. a `Rule` that lives on the stack.
/// 
#[allow(non_snake_case)]
#[inline]
pub fn SRule<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>() -> StackRuleNode<Child, Ok, Err, Store, Pos, V> {
    StackRuleNode::new()
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StackRuleNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    child: RefCell<Option<Child>>,
    pub(crate) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}


impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> StackRuleNode<Child, Ok, Err, Store, Pos, V> {
    #[inline]
    pub fn new_set(child: Child) -> Self {
        StackRuleNode {
            child: RefCell::new(Some(child)),
            _zst: ZSTNode::default()
        }
    }

    #[inline]
    pub fn new() -> Self {
        StackRuleNode {
            child: RefCell::new(None),
            _zst: ZSTNode::default()
        }
    }

    #[inline]
    pub fn set(&self, child: Child) {
        *self.child.borrow_mut() = Some(child);
    }
}

impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for StackRuleNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        if let Some(child) = &*self.child.borrow() {
            child.parse(store, pos)
        } else {
            panic!("a `StackParseRule` was not initialized by the time its `parse` function was called");
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        if let Some(child) = &*self.child.borrow() {
            child.parse_span(store, pos)
        } else {
            panic!("a `StackParseRule` was not initialized by the time its `parse_span` function was called");
        }
    }
}