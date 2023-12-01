use crate::parser::{ZSTNode, ParseStore, ParsePos, ParseValue, ValueOutsideSetError, ParseNode, ParseResult, UnexpectedEndError, ParseContext};


use ParseResult::*;


#[allow(non_snake_case)]
pub fn ValueSet<Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize>(values: [V; N]) -> ValueSetNode<Ok, Err, Store, Pos, V, N> {
    ValueSetNode { values, _zst: ZSTNode::default() }
}

pub struct ValueSetNode<Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize> {
    pub values: [V; N],
    _zst: ZSTNode<Ok, Err, Store, Pos, V>,
}

impl <Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize> ParseNode<V, Err, Store, Pos, V> for ValueSetNode<Ok, Err, Store, Pos, V, N> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<V, Err, Pos> {
        let mut new_pos = cxt.pos.clone();

        // get the value at the current position
        let value = match cxt.store.value_at(&mut new_pos) {
            Some(value) => value,
            None => return Error(UnexpectedEndError { pos: cxt.pos }.into()),
        };

        // check if the value is in the set
        for v in &self.values {
            if &value == v {
                // value was in the set
                return Okay(value, new_pos);
            }
        }

        // value not in the set, return error saying so
        Error(ValueOutsideSetError { pos: cxt.pos, found: value, set: self.values.clone() }.into())
    }
}
impl <Ok, Err: From<ValueOutsideSetError<Pos, V, N>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize> Clone for ValueSetNode<Ok, Err, Store, Pos, V, N> {
    fn clone(&self) -> Self {
        Self { values: self.values.clone(), _zst: self._zst.clone() }
    }
}