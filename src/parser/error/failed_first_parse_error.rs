use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("a parse node expected its child to parse at least once at (starting position {pos}) but it failed to")]
pub struct FailedFirstParseError<Pos: ParsePos, Err> {
    pub pos: Pos,
    pub cause: Err,
}

impl <Pos: ParsePos, Err> From<FailedFirstParseError<Pos, Err>> for String {
    fn from(value: FailedFirstParseError<Pos, Err>) -> Self {
        format!("{}", value)
    }
}