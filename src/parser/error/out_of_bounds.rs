use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("position {pos} was unexpectedly out of bounds")]
pub struct OutOfBounds<Pos: ParsePos> {
    pub pos: Pos
}


impl <Pos: ParsePos> From<OutOfBounds<Pos>> for String {
    fn from(value: OutOfBounds<Pos>) -> Self {
        format!("{}", value)
    }
}


