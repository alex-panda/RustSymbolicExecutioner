use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};

/// 
/// Returns a node that expects that if its first child parses then its second
/// child must parse as well. If the first child parses but the second one does
/// not, then the given function is used to map the successful first child's
/// result and the failure second child's result into the `Err` that this node
/// will wrap in a `ParseResult::Panic` and return.
/// 
#[allow(non_snake_case)]
pub fn Leader<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, F: Fn(&Store, Ok1, Err) -> Err, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2, f: F) -> LeaderNode<Child1, Child2, F, Ok1, Ok2, Err, Store, Pos, V> {
    LeaderNode {
        child1,
        child2,
        func: f,
        _zst: ZSTNode::default(),
    }
}

pub struct LeaderNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, F: Fn(&Store, Ok1, Err) -> Err, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1, 
    pub child2: Child2, 
    pub func: F,
    _zst: ZSTNode<(Ok1, Ok2), Err, Store, Pos, V>,
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, F: Fn(&Store, Ok1, Err) -> Err, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(Ok1, Ok2), Err, Store, Pos, V> for LeaderNode<Child1, Child2, F, Ok1, Ok2, Err, Store, Pos, V> {
    fn parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(Ok1, Ok2), Err, Pos> {
        use ParseResult::*;

        // try to parse child 1
        let ok1 = match self.child1.parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                v
            },
            Error(error) => return Error(error),
            Panic(error) => return Panic(error),
        };

        // child1 parsed so we expect child2 to also parse
        let ok2 = match self.child2.parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                v
            },
            Error(error) => return Panic((self.func)(cxt.store, ok1, error)),
            Panic(error) => return Panic(error),
        };

        Okay((ok1, ok2), cxt.pos)
    }
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V> + Clone, Child2: ParseNode<Ok2, Err, Store, Pos, V> + Clone, F: Clone + Fn(&Store, Ok1, Err) -> Err, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for LeaderNode<Child1, Child2, F, Ok1, Ok2, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child1: self.child1.clone(), child2: self.child2.clone(), func: self.func.clone(), _zst: self._zst.clone() }
    }
}