use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore};


/// 
/// A node that requires its child to parse successfully and returns
/// `ParseResult::Panic` if it does not.
/// 
/// Returns a node that requires its child to parse successfully, mapping the
/// childs `ParseResult::Error` to a `ParseResult::Panic` using the given
/// function if the child fails to parse.
/// 
#[allow(non_snake_case)]
pub fn Req<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(&Store, Pos, Err) -> Err, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child, f: F) -> ReqNode<Child, F, Ok, Err, Store, Pos, V> {
    ReqNode {
        child,
        func: f,
        _zst: ZSTNode::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReqNode<Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(&Store, Pos, Err) -> Err, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child: Child, 
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <Child: ParseNode<Ok, Err, Store, Pos, V>, F: Fn(&Store, Pos, Err) -> Err, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for ReqNode<Child, F, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        use ParseResult::*;
        match self.child.parse(store, pos.clone()) {
            Okay(value, advance) => Okay(value, advance),
            Error(err) => Panic((self.func)(store, pos, err)),
            Panic(err) => Panic(err),
        }
    }
}