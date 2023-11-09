use super::{ParseStore, ParsePos, ParseValue, ParseResult, Span};


use ParseResult::*;
pub trait ParseNode<Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    /// 
    /// Parses a value of the given store starting at the given position.
    /// 
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos>;

    /// 
    /// Parses a span of the given store starting at the given position.
    /// 
    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.parse(store, pos.clone()) {
            Okay(_) => Okay(Span::new(pos.clone(), pos)),
            OkayAdvance(_, advance) => OkayAdvance(Span::new(pos, advance.clone()), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
}


impl <'a, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for &'a dyn ParseNode<Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        (**self).parse(store, pos)
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        (**self).parse_span(store, pos)
    }
}

impl <'a, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for &'a mut dyn ParseNode<Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        (**self).parse(store, pos)
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        (**self).parse_span(store, pos)
    }
}

impl <'a, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for &'a Child {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        (**self).parse(store, pos)
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        (**self).parse_span(store, pos)
    }
}