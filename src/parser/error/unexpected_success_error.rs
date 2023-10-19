use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("a parse node expected its child to fail its parse at position {pos} but the child succeeded instead")]
pub struct UnexpectedSuccessError<Pos: ParsePos, Ok> {
    pub pos: Pos,
    pub child_result: Option<Ok>,
}

impl <Pos: ParsePos, Ok> From<UnexpectedSuccessError<Pos, Ok>> for String {
    fn from(value: UnexpectedSuccessError<Pos, Ok>) -> Self {
        format!("{}", value)
    }
}