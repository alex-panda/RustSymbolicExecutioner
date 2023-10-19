use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("the parse has failed to produce any value at position {pos}")]
pub struct NoReturnValueError<Pos: ParsePos> {
    pub pos: Pos
}


impl <Pos: ParsePos> From<NoReturnValueError<Pos>> for String {
    fn from(value: NoReturnValueError<Pos>) -> Self {
        format!("{}", value)
    }
}