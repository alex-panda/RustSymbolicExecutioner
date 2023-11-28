use crate::parser::{ZSTNode, Span, ParseContext};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};

/// 
/// Returns a node that will map every `ParseResult::Error` that the child node
/// parses into `ParseResult::Okay(None)`, every `ParseResult::Okay(Ok)` into
/// `ParseResult::Okay(Some(Ok))`, and every `ParseResult::OkayAdvance(Ok, Pos)` into
/// `ParseResult::OkayAdvance(Some(Ok), Pos)`.
/// 
#[allow(non_snake_case)]
pub fn Maybe<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>(child: Child) -> MaybeNode<Child, Ok, Err, Store, Pos, V> {
    MaybeNode { child, zst: ZSTNode::default() }
}

pub struct MaybeNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    pub child: Child,
    zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

use ParseResult::*;
impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Option<Ok>, Err, Store, Pos, V> for MaybeNode<Child, Ok, Err, Store, Pos, V> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Option<Ok>, Err, Pos> {
        match self.child.do_parse(cxt.clone()) {
            Okay(value, advance) => Okay(Some(value), advance),
            Error(_) => Okay(None, cxt.pos),
            Panic(error) => Panic(error),
        }
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.child.do_parse_span(cxt.clone()) {
            Okay(_, advance) => Okay(Span::new(cxt.pos, advance.clone()), advance),
            Error(_) => Okay(Span::new(cxt.pos.clone(), cxt.pos.clone()), cxt.pos),
            Panic(error) => Panic(error),
        }
    }
}


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone> Clone for MaybeNode<Child, Ok, Err, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { child: self.child.clone(), zst: self.zst.clone() }
    }
}