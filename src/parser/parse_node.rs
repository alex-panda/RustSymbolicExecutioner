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