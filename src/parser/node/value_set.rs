use crate::parser::{ZSTNode, ParseStore, ParsePos, ParseValue, ValueOutsideSetError, ParseNode, ParseResult, UnexpectedEndError};


use ParseResult::*;


#[allow(non_snake_case)]
pub fn ValueSet<Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize>(values: [V; N]) -> ValueSetNode<Ok, Err, Store, Pos, V, N> {
    ValueSetNode { values, _zst: ZSTNode::default() }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueSetNode<Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize> {
    pub values: [V; N],
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize> ParseNode<V, Err, Store, Pos, V> for ValueSetNode<Ok, Err, Store, Pos, V, N> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<V, Err, Pos> {
        let mut new_pos = pos.clone();

        // get the value at the current position
        let value = match store.value_at(&mut new_pos) {
            Some(value) => value,
            None => return Error(UnexpectedEndError { pos }.into()),
        };

        // check if the value is in the set
        for v in &self.values {
            if &value == v {
                // value was in the set
                return OkayAdvance(value, new_pos);
            }
        }

        // value not in the set, return error saying so
        Error(ValueOutsideSetError { pos, found: value, set: self.values.clone() }.into())
    }
}