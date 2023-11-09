use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};


impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for Box<Child> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        self.as_ref().parse(store, pos)
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<crate::parser::Span<Pos>, Err, Pos> {
        self.as_ref().parse_span(store, pos)
    }
}