use std::cell::RefCell;
use crate::parser::{Span, ParsePos, ParseStore, ParseValue, ParseNode, ParseResult, ZSTNode, ExpectedChildError, RefCellReadError, EmptyRuleError};

/// 
/// A stack rule i.e. a `Rule` that lives on the stack.
/// 
#[allow(non_snake_case)]
#[inline]
pub fn SRule<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<EmptyRuleError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>() -> StackRuleNode<Child, Ok, Err, Store, Pos, V> {
    StackRuleNode::new()
}

pub struct StackRuleNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<EmptyRuleError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    child: RefCell<Option<Child>>,
    pub(crate) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<EmptyRuleError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> StackRuleNode<Child, Ok, Err, Store, Pos, V> {
    #[inline]
    pub fn with(child: Child) -> Self {
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

    pub fn din<'a>(&'a self) -> &dyn ParseNode<Ok, Err, Store, Pos, V> {
        self
    }

    #[inline]
    pub fn set(&self, child: Child) {
        *self.child.borrow_mut() = Some(child);
    }
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err: From<EmptyRuleError>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for StackRuleNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        if let Some(child) = &*self.child.borrow() {
            child.parse(store, pos)
        } else {
            ParseResult::Error(EmptyRuleError.into())
        }
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        if let Some(child) = &*self.child.borrow() {
            child.parse_span(store, pos)
        } else {
            ParseResult::Error(EmptyRuleError.into())
        }
    }
}
    