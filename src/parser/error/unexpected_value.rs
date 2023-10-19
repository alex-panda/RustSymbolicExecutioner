use thiserror::Error;

use super::super::{ParsePos, ParseValue};


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("position {pos} had value {found} when expected value {expected}")]
pub struct UnexpectedValueError<Pos: ParsePos, V: ParseValue> {
    pub pos: Pos,
    pub found: V,
    pub expected: V,
}

impl <Pos: ParsePos, V: ParseValue> From<UnexpectedValueError<Pos, V>> for String {
    fn from(value: UnexpectedValueError<Pos, V>) -> Self {
        format!("{}", value)
    }
}