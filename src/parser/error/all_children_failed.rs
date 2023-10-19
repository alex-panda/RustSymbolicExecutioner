use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("all children of a parse node failed when the parse node tried to parse its children at position {pos}")]
pub struct AllChildrenFailedError<Pos: ParsePos, Err, const NUM_CHILDREN: usize> {
    pub pos: Pos,
    pub errors: [Err; NUM_CHILDREN],
}

impl <Pos: ParsePos, Err, const NUM_CHILDREN: usize> From<AllChildrenFailedError<Pos, Err, NUM_CHILDREN>> for String {
    fn from(value: AllChildrenFailedError<Pos, Err, NUM_CHILDREN>) -> Self {
        format!("{}", value)
    }
}
