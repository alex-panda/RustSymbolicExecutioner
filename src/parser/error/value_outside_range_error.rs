use std::ops::Bound;

use thiserror::Error;

use super::super::{ParsePos, ParseValue};


#[derive(Error, Debug, Clone)]
#[error("position {pos} had value {found} which was outside its expected range of values ({range_start:?} to {range_end:?})")]
pub struct ValueOutsideRangeError<Pos: ParsePos, V: ParseValue> {
    pub pos: Pos,
    pub found: V,
    pub range_start: Bound<V>,
    pub range_end: Bound<V>,
}

impl <Pos: ParsePos, V: ParseValue> From<ValueOutsideRangeError<Pos, V>> for String {
    fn from(value: ValueOutsideRangeError<Pos, V>) -> Self {
        format!("{}", value)
    }
}


