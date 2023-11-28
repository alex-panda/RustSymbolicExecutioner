use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};

/// 
/// Returns a node that assures that, if the first node parses, both of the
/// other two node will also parse. If either the middle child or last child
/// fail to parse after the first child parses successfully, a new error
/// created using one of the given functions and returned wrapped in a
/// `ParseResult::Panic`.
/// 
/// The purpose of this node is to be for when you have node surrounded by two
/// other nodes --- a starter node and an end node. This is commonly seen in
/// scopes such as `('{', ZeroOrMore(&statement), '}')` where zero or more
/// statements are allowed as long as they are within two curly braces.
/// The `Surround`` node, then, says that if the first node parses
/// successfully (`'{'`), both the second node (`ZeroOrMore(&statement)`) and
/// the third node ('`}`') must parse as well, otherwise the parse panics
/// (returns `ParseResult::Panic`), ending the parse as a whole. This is useful
/// for error handling as it is often advantageous to report logical errors such
/// as a missing ending curly brace.
/// 
#[allow(non_snake_case)]
pub fn Surround<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Child3: ParseNode<Ok3, Err, Store, Pos, V>, MiddleFail: Fn(&Store, Ok1, Err) -> Err, EndFail: Fn(&Store, Ok1, Ok2, Err) -> Err, Ok1, Ok2, Ok3, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2, child3: Child3, mfail: MiddleFail, efail: EndFail) -> SurroundNode<Child1, Child2, Child3, MiddleFail, EndFail, Ok1, Ok2, Ok3, Err, Store, Pos, V> {
    SurroundNode {
        child1,
        child2,
        child3,
        middle_fail: mfail,
        end_fail: efail,
        _zst: ZSTNode::default(),
    }
}

pub struct SurroundNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Child3: ParseNode<Ok3, Err, Store, Pos, V>, MiddleFail: Fn(&Store, Ok1, Err) -> Err, EndFail: Fn(&Store, Ok1, Ok2, Err) -> Err, Ok1, Ok2, Ok3, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1, 
    pub child2: Child2, 
    pub child3: Child3, 
    pub middle_fail: MiddleFail,
    pub end_fail: EndFail,
    _zst: ZSTNode<(Ok1, Ok2, Ok3), Err, Store, Pos, V>,
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Child3: ParseNode<Ok3, Err, Store, Pos, V>, MiddleFail: Fn(&Store, Ok1, Err) -> Err, EndFail: Fn(&Store, Ok1, Ok2, Err) -> Err, Ok1, Ok2, Ok3, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(Ok1, Ok2, Ok3), Err, Store, Pos, V> for SurroundNode<Child1, Child2, Child3, MiddleFail, EndFail, Ok1, Ok2, Ok3, Err, Store, Pos, V> {
    fn do_parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(Ok1, Ok2, Ok3), Err, Pos> {
        use ParseResult::*;
        let ok1 = match self.child1.do_parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                v
            },
            Error(error) => return Error(error),
            Panic(error) => return Panic(error),
        };

        let ok2 = match self.child2.do_parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                v
            },
            Error(error) => return Panic((self.middle_fail)(cxt.store, ok1, error)),
            Panic(error) => return Panic(error),
        };

        let ok3 = match self.child3.do_parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                v
            },
            Error(error) => return Panic((self.end_fail)(cxt.store, ok1, ok2, error)),
            Panic(error) => return Panic(error),
        };

        Okay((ok1, ok2, ok3), cxt.pos)
    }
}


impl <Child1: ParseNode<Ok1, Err, Store, Pos, V> + Clone, Child2: ParseNode<Ok2, Err, Store, Pos, V> + Clone, Child3: ParseNode<Ok3, Err, Store, Pos, V> + Clone, MiddleFail: Clone + Fn(&Store, Ok1, Err) -> Err, EndFail: Clone + Fn(&Store, Ok1, Ok2, Err) -> Err, Ok1, Ok2, Ok3, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for SurroundNode<Child1, Child2, Child3, MiddleFail, EndFail, Ok1, Ok2, Ok3, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child1: self.child1.clone(), child2: self.child2.clone(), child3: self.child3.clone(), middle_fail: self.middle_fail.clone(), end_fail: self.end_fail.clone(), _zst: self._zst.clone() }
    }
}