use std::rc::Rc;

use crate::parser::{Span, ParseContext};

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult};


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ?Sized + ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for Rc<Child> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        self.as_ref().do_parse(cxt)
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        self.as_ref().do_parse_span(cxt)
    }
}




