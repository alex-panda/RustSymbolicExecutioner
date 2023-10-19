use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("the parser failed to advance when it expected to at position {pos}")]
pub struct NoAdvanceError<Pos: ParsePos> {
    pub pos: Pos
}

impl <Pos: ParsePos> From<NoAdvanceError<Pos>> for String {
    fn from(value: NoAdvanceError<Pos>) -> Self {
        format!("{}", value)
    }
}