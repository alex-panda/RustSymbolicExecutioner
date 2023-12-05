use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};

/// 
/// # Left-to-Right Join Node
/// 
/// Returns a node that will parse one or more of its first child as long as
/// all consecutive parses of its first child have a successful parse of its
/// second child (the delimiter) between them. After the first child is parsed
/// one or more times, the resulting vector of results is combined into a single
/// result (the overall result that this node will return) by repeatedly
/// calling the given function on the left-most two results in the vector.
/// 
/// ## Examples
/// 
/// ### Simulating Left-Recursive Parsing
/// 
/// One reason to use this node would be to create a left-recursive AST without
/// left recursion. For example, `LRJoin(expr, '+', |left, delim, right| Expr::Add { left, right })`
/// will parse one or more expressions so long as each expression has a plus-sign character (`'+'`)
/// between it and the previous parse. Then, the node will pass the left-most
/// result of the vector in as the first argument of the given function, the plus-sign character result in as the
/// second argument of the given function, and the second left-most result in as the third
/// argument of the given function. What the function returns will then be considered
/// the left-most result of the parse and the process will repeat again until there is only
/// one child.
/// 
/// #### Simulating Left-Recursive Parsing Breakdown
/// 
/// ```{text}
/// // Conceptually, the childrens' results are parsed into two vectors like so:
/// 
/// vec![expr1, expr2, expr3, expr4] // the vector of child1 results
/// 
/// vec![delim_1_2, delim_2_3, delim_3_4] // the vector of child2 results (1 less result because the second child is only parsed between parses of the first child)
/// 
/// // Then, the function is repeatedly called using the left-most values of the vector of child1 results to turn it into a left-recursive AST.
/// 
///          add2
///          / \
///       add1 expr4
///       / \
///     add expr3
///     / \
/// expr1 expr2
/// 
/// // As a breakdown, the results start like this:
/// 
/// vec![expr1, expr2, expr3, expr4]
/// 
/// vec![delim_1_2, delim_2_3, delim_3_4]
/// 
/// // Then, after one call of the function with the left-most nodes, the results looks like this (where `add` is the AST node that was returned by the function call and the function throws away the `delim_1_2` result given to it):
/// 
/// vec![add, expr3, expr4]
/// 
/// vec![delim_2_3, delim_3_4]
/// 
///     add
///     / \
/// expr1 expr2
/// 
/// // After another call, the results look like this:
/// 
/// vec![add1, expr4]
/// 
/// vec![delim_3_4]
/// 
///       add1
///       / \
///     add expr3
///     / \
/// expr1 expr2
/// 
/// // After yet another call, the results look like this:
/// 
/// vec![add2]
/// 
/// vec![]
/// 
///          add2
///          / \
///       add1 expr4
///       / \
///     add expr3
///     / \
/// expr1 expr2
/// 
/// // Since there is now only one result in the vector, `add2` is returned as the final result of the node.
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

        // successful parse of child1 so continue on to next parses seperated by
        // child2 (the delimiter)

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