use thiserror::Error;

use super::super::{ParsePos, ParseValue};


#[derive(Error, Debug, Clone)]
#[error("position {pos} had value {found} but that is outside the set of expected values: {set:?}")]
pub struct ValueOutsideSetError<Pos: ParsePos, V: ParseValue, const N: usize> {
    pub pos: Pos,
    pub found: V,
    pub set: [V; N],
}

impl <Pos: ParsePos, V: ParseValue, const N: usize> From<ValueOutsideSetError<Pos, V, N>> for String {
    fn from(value: ValueOutsideSetError<Pos, V, N>) -> Self {
        format!("{}", value)
    }
}