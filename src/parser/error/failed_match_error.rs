use thiserror::Error;

use super::super::{ParsePos, ParseValue};


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("the parser expected {expected} to be found at position {pos} but found {found} instead")]
pub struct FailedMatchError<Pos: ParsePos, V: ParseValue> {
    pub pos: Pos,
    pub expected: V,
    pub found: V,
}

impl <Pos: ParsePos, Err: ParseValue> From<FailedMatchError<Pos, Err>> for String {
    fn from(value: FailedMatchError<Pos, Err>) -> Self {
        format!("{}", value)
    }
}