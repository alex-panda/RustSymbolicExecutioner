use thiserror::Error;

use super::super::ParsePos;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("expected the parse to have consumed all of what it was parsing by position {pos} but there was still more")]
pub struct ExpectedEndError<Pos: ParsePos> {
    pub pos: Pos
}


impl <Pos: ParsePos> From<ExpectedEndError<Pos>> for String {
    fn from(value: ExpectedEndError<Pos>) -> Self {
        format!("{}", value)
    }
}

