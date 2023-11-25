
use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, Span};

/// 
/// A node that parses its first child zero or more times, but only so long as
/// the second child parses between each consecutive parse of the first child
/// e.g. `Join('e', ',')` will parse sucessfully when given `"e"` or
/// `"e,e,e,e"`, but not "eeee" or ",,,".
/// 
/// Because the second node is assumed to only be a delimiter, only a vector of
/// the first node's results is returned.
/// 
#[allow(non_snake_case)]
pub fn Join<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child1: Child1, child2: Child2) -> JoinNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    JoinNode {
        child1,
        child2,
        _zst: ZSTNode::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JoinNode<Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub child1: Child1, 
    pub child2: Child2, 
    _zst: ZSTNode<(Ok1, Ok2), Err, Store, Pos, V>,
}

impl <Child1: ParseNode<Ok1, Err, Store, Pos, V>, Child2: ParseNode<Ok2, Err, Store, Pos, V>, Ok1, Ok2, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Vec<Ok1>, Err, Store, Pos, V> for JoinNode<Child1, Child2, Ok1, Ok2, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, mut pos: Pos) -> ParseResult<Vec<Ok1>, Err, Pos> {
        use ParseResult::*;
        let mut curr_pos = pos.clone();
        let mut out = Vec::new();

        // parse the delimited node
        match self.child1.parse(store, curr_pos.clone()) {
            Okay(ok, adv) => {
                curr_pos = adv;
                out.push(ok);
            },
            Error(_) => return Okay(out, pos),
            Panic(err) => return Panic(err),
        }

        pos = curr_pos.clone();

        loop {
            // parse the delimiter node
            match self.child2.parse(store, curr_pos.clone()) {
                Okay(_, adv) => {
                    curr_pos = adv;
                },
                Error(_) => return Okay(out, pos),
                Panic(err) => return Panic(err),
            }

            // parse the delimited node
            match self.child1.parse(store, curr_pos.clone()) {
                Okay(ok, adv) => {
                    curr_pos = adv;
                    out.push(ok);
                },
                Error(_) => return Okay(out, pos.clone()),
                Panic(err) => return Panic(err),
            }

            pos = curr_pos.clone();
        }
    }

    fn parse_span(&self, store: &Store, mut pos: Pos) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        use ParseResult::*;
        let start_pos = pos.clone();
        let mut curr_pos = pos.clone();

        // parse the delimited node
        match self.child1.parse_span(store, curr_pos.clone()) {
            Okay(_, adv) => {
                curr_pos = adv;
            },
            Error(_) => return Okay(Span::new(start_pos, pos.clone()), pos),
            Panic(err) => return Panic(err),
        }

        pos = curr_pos.clone();

        loop {
            // parse the delimiter node
            match self.child2.parse_span(store, curr_pos.clone()) {
                Okay(_, adv) => {
                    curr_pos = adv;
                },
                Error(_) => return Okay(Span::new(start_pos, pos.clone()), pos),
                Panic(err) => return Panic(err),
            }

            // parse the delimited node
            match self.child1.parse_span(store, curr_pos.clone()) {
                Okay(_, adv) => {
                    curr_pos = adv;
                },
                Error(_) => return Okay(Span::new(start_pos, pos.clone()), pos.clone()),
                Panic(err) => return Panic(err),
            }

            pos = curr_pos.clone();
        }
    }
}