use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, ParseContext};

/// 
/// # Right-to-Left Join Node
/// 
/// Returns a node that will parse one or more of its first child as long as
/// every two consecutive parses of its first child have a successful parse of its
/// second child (the delimiter) between them. After the first child is parsed
/// one or more times, the resulting vector of results is combined into a single
/// result (the overall result that this node then returns) by repeatedly
/// calling the given function on the right-most results in the vector.
/// 
/// ## Examples
/// 
/// ### Simulating Left-Recursive Parsing
/// 
/// One reason to use this node would be to create a right-recursive AST without
/// right recursion. For example, `RLJoin(expr, '+', |left, plus_span, right| Expr::Add { left, plus_span, right })`
/// will parse one or more expressions so long as each expression has a plus-sign character (`'+'`)
/// between it and the previous parse. Then, the node will pass the second right-most
/// result of the vector in as the first argument of the given function, the plus-sign character result in as the
/// second argument of the given function, and the right-most child's result in as the third
/// argument of the function. What the function returns will then be considered
/// the right-most result of the parse and the process will repeat again until there is only
/// one child.
/// 
/// #### Simualating Left-Recursive Parsing Breakdown
/// 
/// ```{text}
/// Conceptually, the first child's results are parsed into two vectors like so:
/// 
/// vec![expr1, expr2, expr3, expr4] // the vector of child1 results
/// 
/// vec![delim_1_2, delim_2_3, delim_3_4] // the vector of child2 results (1 less result because they are between the child1 results)
/// 
/// Then, the function is repeatedly called on the right-most values of
/// the vector to turn it into a left-recursive AST.
/// 
///         add2
///         / \
///     expr1 add1
///           / \
///       expr2 add
///             / \
///         expr3 expr4
/// 
/// As a breakdown, the results start like this:
/// 
/// vec![expr1, expr2, expr3, expr4]
/// 
/// vec![delim_1_2, delim_2_3, delim_3_4]
/// 
/// Then, after one call of the function with the right-most nodes, the results
/// looks like this (where `add` is the AST node that was returned by the
/// function call and it ignores the `delim_2_3` result given to it):
/// 
/// vec![expr1, expr2, add]
/// 
/// vec![delim_1_2, delim_2_3]
/// 
///     add
///     / \
/// expr3 expr4
/// 
/// After another call, the results look like this:
/// 
/// vec![expr1, add1]
/// 
/// 
/// vec![delim_1_2]
/// 
///       add1
///       / \
///   expr2 add
///        / \
///     expr3 expr4
/// 
/// After another call, the results look like this:
/// 
/// vec![add2]
/// 
/// vec![]
/// 
///          add2
///          / \
///      expr1 add1
///            / \
///        expr2 add
///              / \
///          expr3 expr4
/// 
/// Since there is only one result left in the vector, `Add2` is returned as the final result.
/// ```
/// 
#[allow(non_snake_case)]
pub fn RLJoin<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, J: Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2, join: J) -> RLJoinNode<Child1, Child2, J, Ok1, Ok2, Err, Store, Pos, V> {
    RLJoinNode {
        child1,
        child2,
        join,
        _zst: ZSTNode::default(),
    }
}

pub struct RLJoinNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, J: Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1, 
    pub child2: Child2, 
    pub join: J,
    _zst: ZSTNode<(Ok1, Ok2), Err, Store, Pos, V>,
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, J: Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Ok1, Err, Store, Pos, V> for RLJoinNode<Child1, Child2, J, Ok1, Ok2, Err, Store, Pos, V> {
    fn parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok1, Err, Pos> {
        use ParseResult::*;

        let mut okays = Vec::new();
        let mut delims = Vec::new();

        // try to parse child 1
        match self.child1.parse(cxt.clone()) {
            Okay(v, advance) => {
                cxt.pos = advance;
                okays.push(v);
            },
            Error(error) => return Error(error),
            Panic(error) => return Panic(error),
        }

        let mut last = cxt.pos.clone();

        // successful parse of child, now parse delimiter and any consecutive
        // results

        loop {
            // parse the delimiter
            let delim = match self.child2.parse(cxt.clone()) {
                Okay(v, advance) => {
                    cxt.pos = advance;
                    v
                },
                Error(_) => break,
                Panic(error) => return Panic(error),
            };

            // parse the first child again now that we have seen the delimiter
            match self.child1.parse(cxt.clone()) {
                Okay(v, advance) => {
                    cxt.pos = advance;
                    okays.push(v);
                },
                Error(_) => break,
                Panic(error) => return Panic(error),
            }

            delims.push(delim);
            last = cxt.pos.clone();
        }

        // join the results
        let mut curr = okays.pop().unwrap();
        while okays.len() > 0 {
            curr = (self.join)(okays.pop().unwrap(), delims.pop().unwrap(), curr);
        }

        // return the resulting node
        return Okay(curr, last)
    }
}

impl <Child1: Clone + ParseNode<Ok1, Err, Store, Pos, V>, Child2: Clone + ParseNode<Ok2, Err, Store, Pos, V>, J: Clone + Fn(Ok1, Ok2, Ok1) -> Ok1, Ok1, Ok2, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for RLJoinNode<Child1, Child2, J, Ok1, Ok2, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child1: self.child1.clone(), child2: self.child2.clone(), join: self.join.clone(), _zst: self._zst.clone() }
    }
}