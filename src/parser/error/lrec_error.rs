use thiserror::Error;
use super::super::ParsePos;

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("`LRec` node error at position {pos}")]
pub struct LRecError<Pos: ParsePos> {
    pub pos: Pos
}

impl <Pos: ParsePos> From<LRecError<Pos>> for String {
    fn from(value: LRecError<Pos>) -> Self {
        format!("{}", value)
    }
}