use crate::parser::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, AllChildrenFailedError, ZSTNode};

use ParseResult::*;
use zst::ZST;


#[allow(non_snake_case)]
pub fn OneOfTwo<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err: From<AllChildrenFailedError<Pos, Err, 2>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2) -> OneOfTwoNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    OneOfTwoNode { child1, child2, _zst: ZSTNode::default(),
        _ok2: ZST::default()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneOfTwoNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err: From<AllChildrenFailedError<Pos, Err, 2>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1,
    pub child2: Child2,
    _zst: ZSTNode<Ok1, Err, Store, Pos, V>,
    _ok2: ZST<Ok2>
}

impl <Ok1, Ok2, Err: From<AllChildrenFailedError<Pos, Err, 2>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>> ParseNode<AnyOfTwo<Ok1, Ok2>, Err, Store, Pos, V> for OneOfTwoNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<AnyOfTwo<Ok1, Ok2>, Err, Pos> {
        let error1 = match self.child1.parse(store, pos.clone()) {
            Okay(value) => return Okay(AnyOfTwo::One(value)),
            OkayAdvance(value, advance) => return OkayAdvance(AnyOfTwo::One(value), advance),
            Error(error) => error,
            Panic(error) => return Panic(error),
        };

        match self.child2.parse(store, pos.clone()) {
            Okay(value) => Okay(AnyOfTwo::Two(value)),
            OkayAdvance(value, advance) => OkayAdvance(AnyOfTwo::Two(value), advance),
            Error(error2) => Error(AllChildrenFailedError { pos, errors: [error1, error2] }.into()),
            Panic(error) => Panic(error),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnyOfTwo<O1, O2> {
    One(O1),
    Two(O2),
}


#[allow(non_snake_case)]
pub fn OneOfThree<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Child3: ParseNode<Ok3, Err, Store, Pos, V>, Ok1, Ok2, Ok3, Err: From<AllChildrenFailedError<Pos, Err, 3>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2, child3: Child3) -> OneOfThreeNode<Child1, Child2, Child3, Ok1, Ok2, Ok3, Err, Store, Pos, V> {
    OneOfThreeNode {
        child1,
        child2,
        child3,
        _zst: ZSTNode::default(),
        _ok2: ZST::default(),
        _ok3: ZST::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneOfThreeNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Child3: ParseNode<Ok3, Err, Store, Pos, V>, Ok1, Ok2, Ok3, Err: From<AllChildrenFailedError<Pos, Err, 3>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1,
    pub child2: Child2,
    pub child3: Child3,
    _zst: ZSTNode<Ok1, Err, Store, Pos, V>,
    _ok2: ZST<Ok2>,
    _ok3: ZST<Ok3>,
}

impl <Ok1, Ok2, Ok3, Err: From<AllChildrenFailedError<Pos, Err, 3>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Child3: ParseNode<Ok3, Err, Store, Pos, V>> ParseNode<AnyOfThree<Ok1, Ok2, Ok3>, Err, Store, Pos, V> for OneOfThreeNode<Child1, Child2, Child3, Ok1, Ok2, Ok3, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<AnyOfThree<Ok1, Ok2, Ok3>, Err, Pos> {
        let error1 = match self.child1.parse(store, pos.clone()) {
            Okay(value) => return Okay(AnyOfThree::One(value)),
            OkayAdvance(value, advance) => return OkayAdvance(AnyOfThree::One(value), advance),
            Error(error) => error,
            Panic(error) => return Panic(error),
        };

        let error2 = match self.child2.parse(store, pos.clone()) {
            Okay(value) => return Okay(AnyOfThree::Two(value)),
            OkayAdvance(value, advance) => return OkayAdvance(AnyOfThree::Two(value), advance),
            Error(error) => error,
            Panic(error) => return Panic(error),
        };

        match self.child3.parse(store, pos.clone()) {
            Okay(value) => Okay(AnyOfThree::Three(value)),
            OkayAdvance(value, advance) => OkayAdvance(AnyOfThree::Three(value), advance),
            Error(error3) => Error(AllChildrenFailedError { pos, errors: [error1, error2, error3] }.into()),
            Panic(error) => Panic(error),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnyOfThree<O1, O2, O3> {
    One(O1),
    Two(O2),
    Three(O3),
}