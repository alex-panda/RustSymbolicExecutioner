use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("a parse node expected its child to fail its parse at position {pos} but the child succeeded instead")]
pub struct UnexpectedSuccessError<Pos: ParsePos> {
    pub pos: Pos,
}

impl <Pos: ParsePos> From<UnexpectedSuccessError<Pos>> for String {
    fn from(value: UnexpectedSuccessError<Pos>) -> Self {
        format!("{}", value)
    }
}