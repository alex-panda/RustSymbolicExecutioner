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
    /// `id` is the unique ID of the parse. All concurrent parses must have
    /// unique IDs. If all parses being done are consecutive (i.e. each one must
    /// finish before the next one can begin) then the same id (such as `0`) can
    /// be given to all parses.
    /// 
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        self.do_parse(ParseContext::new(store, pos))
    }

    /// 
    /// Parses a span of the given store starting at the given position.
    /// 
    /// Look at `parse`'s documentation for more details.
    /// 
    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        self.do_parse_span(ParseContext::new(store, pos))
    }

    /// 
    /// The function that actually does the `parse`.
    /// 
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos>;

    /// 
    /// Does the parse, returning a span (used for optimization where, since we
    /// just want the span, the parse does not need to create objects).
    /// 
    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        match self.do_parse(cxt.clone()) {
            Okay(_, advance) => Okay(Span::new(cxt.pos, advance.clone()), advance),
            Error(error) => Error(error),
            Panic(error) => Panic(error),
        }
    }
}

impl <'a, Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + ?Sized> ParseNode<Ok, Err, Store, Pos, V> for &'a Child {
    fn do_parse<'b>(&self, cxt: ParseContext<'b, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        (**self).do_parse(cxt)
    }

    fn do_parse_span<'b>(&self, cxt: ParseContext<'b, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        (**self).do_parse_span(cxt)
    }
}