use super::{ParseStore, ParsePos, ParseValue, ParseResult, Span, ParseContext};

use ParseResult::*;
pub trait ParseNode<Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    /// 
    /// Parses a value from the given store starting at the given position.
    /// 
    /// `store` is the data that is being parsed.
    /// 
    /// `pos` is the current position of the parse.
    /// 
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos>;

    /// 
    /// Does the parse, returning a span (used for optimization where, since we
    /// just want the span and not any objects made from it we can just do the
    /// parse).
    /// 
    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.parse(cxt.clone()) {
            Okay(_, advance) => Okay(Span::new(cxt.pos, advance.clone()), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
}

impl <'a, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + ?Sized> ParseNode<Ok, Err, Store, Pos, V> for &'a Child {
    fn parse<'b>(&self, cxt: ParseContext<'b, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        (**self).parse(cxt)
    }

    fn parse_span<'b>(&self, cxt: ParseContext<'b, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        (**self).parse_span(cxt)
    }
}