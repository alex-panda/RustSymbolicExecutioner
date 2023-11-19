use crate::parser::{ZSTNode, ParseNode, ParseResult, ParseValue, ParsePos, ParseStore, UnexpectedEndError, Span};


/// 
/// Returns a node that will map the value at the current position of the parse
/// to a `ParseResult` using the given function.
/// 
#[allow(non_snake_case)]
pub fn MapPValue<F: Fn(Span<Pos>, V) -> ParseResult<Ok, Err, Pos>, Ok, Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(f: F) -> MapPValueNode<F, Ok, Err, Store, Pos, V> {
    MapPValueNode {
        func: f,
        _zst: ZSTNode::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapPValueNode<F: Fn(Span<Pos>, V) -> ParseResult<Ok, Err, Pos>, Ok, Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub func: F,
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <F: Fn(Span<Pos>, V) -> ParseResult<Ok, Err, Pos>, Ok, Err: From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for MapPValueNode<F, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        let mut curr_pos = pos.clone();
        let v = store.value_at(&mut curr_pos);
        match v {
            Some(v) => (self.func)(Span::new(pos, curr_pos), v),
            None => ParseResult::Error(UnexpectedEndError { pos }.into())
        }
    }
}

