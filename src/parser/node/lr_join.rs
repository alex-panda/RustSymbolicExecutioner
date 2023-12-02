use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};

/// 
/// # Left-to-Right Join Node
/// 
/// Returns a node that will parse one or more of its first child as long as
/// each consecutive parse of its first child has a successful parse of its
/// second child between it and the previous successfull parse of the first
/// child. Every two parses of the first child are combined, from left to right,
/// into a single result using the given function. As such, this node will
/// return only a single result that is the same type as that which its first
/// child produces. If, instead, you would like a list of joined results without
/// a function congealing them into a single result, use the `Join` node instead.
/// 
/// One reason to use this node would be to create a left-recursive AST without
/// left recursion. For example, `LRJoin(expr, '+', |left, _, right| Expr::Add { left, right })`
/// will parse one or more expressions so long as each expression has a `'+'`
/// between it and the previous one. Then, the node will pass the left-most
/// result in as the first argument of the given function, the `'+'` result as the
/// second argument of the function, and the second left-most child in as the third
/// argument of the function. What the function returns will then be considered
/// the left-most result and the process will repeat again until there is only
/// one child.
/// 
/// ```{text}
/// Conceptually, the first child's results become
/// vec![expr1, expr2, expr3, expr4]
/// which then the function of the node turns into the AST
///         Add
///         / \
///       Add expr4
///       / \
///     Add expr3
///     / \
/// expr1 expr2
/// after the function is called repeatedly.
/// 
/// As a breakdown, the result starts like this:
/// 
/// vec![expr1, expr2, expr3, expr4]
/// 
/// Empty AST
/// 
/// Then, after one call of the function, the result looks like this (`Add` is what was returned by the function and it is only mentally marked (in truth it is just an `Expr` type like everything else)):
/// vec![Add, expr3, expr4]
/// 
///     Add
///     / \
/// expr1 expr2
/// 
/// After another call, the result looks like this:
/// vec![Add1, expr4]
/// 
///       Add1
///       / \
///     Add expr3
///     / \
/// expr1 expr2
/// 
/// After another call, the result then looks like this:
/// vec![Add2]
///          Add2
///          / \
///       Add1 expr4
///       / \
///     Add expr3
///     / \
/// expr1 expr2
/// 
/// Since there is only one result left, `Add2` is returned as the final result.
/// ```
/// 
#[allow(non_snake_case)]
pub fn LRJoin<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, J: Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2, join: J) -> LRJoinNode<Child1, Child2, J, Ok1, Ok2, Err, Store, Pos, V> {
    LRJoinNode {
        child1,
        child2,
        join,
        _zst: ZSTNode::default(),
    }
}

pub struct LRJoinNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, J: Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1, 
    pub child2: Child2, 
    pub join: J,
    _zst: ZSTNode<(Ok1, Ok2), Err, Store, Pos, V>,
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, J: Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Ok1, Err, Store, Pos, V> for LRJoinNode<Child1, Child2, J, Ok1, Ok2, Err, Store, Pos, V> {
    fn parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok1, Err, Pos> {
        use ParseResult::*;

        // try to parse child 1
        let mut curr = match self.child1.parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                v
            },
            Error(error) => return Error(error),
            Panic(error) => return Panic(error),
        };

        let mut last = cxt.pos.clone();

        loop {
            // parse the delimiter
            let delim = match self.child2.parse(cxt.clone()) {
                Okay(v, advance) => {
                    cxt.pos = advance;
                    v
                },
                Error(_) => return Okay(curr, cxt.pos),
                Panic(error) => return Panic(error),
            };

            // parse the first child again now that we have seen the delimiter
            match self.child1.parse(cxt.clone()) {
                Okay(v, advance) => {
                    cxt.pos = advance;
                    curr = (self.join)(curr, delim, v); // join the last result and the current result
                },
                Error(_) => return Okay(curr, last),
                Panic(error) => return Panic(error),
            }

            last = cxt.pos.clone();
        }
    }
}

impl <Child1: Clone + ParseNode<Ok1, Err, Store, Pos, V>, Child2: Clone + ParseNode<Ok2, Err, Store, Pos, V>, J: Clone + Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for LRJoinNode<Child1, Child2, J, Ok1, Ok2, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child1: self.child1.clone(), child2: self.child2.clone(), join: self.join.clone(), _zst: self._zst.clone() }
    }
}