use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("position {pos} was passed the end of the parse values when it was expected to not be")]
pub struct UnexpectedEndError<Pos: ParsePos> {
    pub pos: Pos
}

impl <Pos: ParsePos> From<UnexpectedEndError<Pos>> for String {
    fn from(value: UnexpectedEndError<Pos>) -> Self {
        format!("{}", value)
    }
}