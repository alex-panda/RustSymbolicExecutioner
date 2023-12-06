
use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, Span, NoAdvanceError, ParseContext, FailedFirstParseError};

/// 
/// A node that parses its first child one or more times, but only so long as
/// the second child parses between each consecutive parse of the first child.
/// 
/// Because the second node is assumed to be an unimportant delimiter, only a
/// vector of the first node's results is returned as the successful result of this
/// node.
/// 
/// # Example
/// 
/// The node `Join('e', ',')` will parse according to the following table ("x" represents that the prase failed).
/// 
/// | Input   | Parsed  | Remaining |
/// |---------|---------|-----------|
/// | ""      | x       | x         |
/// | "e"     | "e"     | ""        |
/// | "e,"    | "e"     | ","       |
/// | "e,e"   | "e,e"   | ""        |
/// | "e,e,"  | "e,e"   | ","       |
/// | "e,e,e" | "e,e,e" | ""        |
/// | "eeeee" | "e"     | "eeee"    |
/// | ",eeee" | x       | x         |
/// | ",,,,," | x       | x         |
/// 
#[allow(non_snake_case)]
pub fn OJoin<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2) -> JoinNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    JoinNode {
        child1,
        child2,
        _zst: ZSTNode::default(),
    }
}

pub struct JoinNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1, 
    pub child2: Child2, 
    _zst: ZSTNode<(Ok1, Ok2), Err, Store, Pos, V>,
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err: From<NoAdvanceError<Pos>> + From<FailedFirstParseError<Pos, Err>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Vec<Ok1>, Err, Store, Pos, V> for JoinNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    fn parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Vec<Ok1>, Err, Pos> {
        use ParseResult::*;
        let mut curr_pos = cxt.pos.clone();
        let mut out = Vec::new();

        // parse the delimited node
        match self.child1.parse(cxt.with_pos(curr_pos.clone())) {
            Okay(ok, adv) => {
                if adv.key() == cxt.pos.key() { return Panic(Err::from(NoAdvanceError { pos: cxt.pos })) }
                curr_pos = adv;
                out.push(ok);
            },
            Error(err) => return Error(Err::from(FailedFirstParseError { pos: cxt.pos, cause: err })),
            Panic(err) => return Panic(err),
        }

        cxt.pos = curr_pos.clone();

        loop {
            // parse the delimiter node
            match self.child2.parse(cxt.with_pos(curr_pos.clone())) {
                Okay(_, adv) => {
                    curr_pos = adv;
                },
                Error(_) => return Okay(out, cxt.pos),
                Panic(err) => return Panic(err),
            }

            // parse the delimited node
            match self.child1.parse(cxt.with_pos(curr_pos.clone())) {
                Okay(ok, adv) => {
                    if adv.key() == cxt.pos.key() { return Panic(Err::from(NoAdvanceError { pos: cxt.pos })) }
                    curr_pos = adv;
                    out.push(ok);
                },
                Error(_) => return Okay(out, cxt.pos.clone()),
                Panic(err) => return Panic(err),
            }

            cxt.pos = curr_pos.clone();
        }
    }

    fn parse_span<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        use ParseResult::*;
        let start_pos = cxt.pos.clone();
        let mut curr_pos = cxt.pos.clone();

        // parse the delimited node
        match self.child1.parse_span(cxt.with_pos(curr_pos.clone())) {
            Okay(_, adv) => {
                if adv.key() == cxt.pos.key() { return Panic(Err::from(NoAdvanceError { pos: cxt.pos })) }
                curr_pos = adv;
            },
            Error(_) => return Okay(Span::new(cxt.pos.clone(), cxt.pos.clone()), cxt.pos),
            Panic(err) => return Panic(err),
        }

        cxt.pos = curr_pos.clone();

        loop {
            // parse the delimiter node
            match self.child2.parse_span(cxt.with_pos(curr_pos.clone())) {
                Okay(_, adv) => {
                    curr_pos = adv;
                },
                Error(_) => return Okay(Span::new(start_pos, cxt.pos.clone()), cxt.pos),
                Panic(err) => return Panic(err),
            }

            // parse the delimited node
            match self.child1.parse_span(cxt.with_pos(curr_pos.clone())) {
                Okay(_, adv) => {
                    if adv.key() == cxt.pos.key() { return Panic(Err::from(NoAdvanceError { pos: cxt.pos })) }
                    curr_pos = adv;
                },
                Error(_) => return Okay(Span::new(start_pos, curr_pos.clone()), cxt.pos.clone()),
                Panic(err) => return Panic(err),
            }

            cxt.pos = curr_pos.clone();
        }
    }
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V> + Clone, Child2: ParseNode<Ok2, Err, Store, Pos, V> + Clone, Ok1, Ok2, Err: From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for JoinNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child1: self.child1.clone(), child2: self.child2.clone(), _zst: self._zst.clone() }
    }
}