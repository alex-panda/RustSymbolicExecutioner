use super::ParsePos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseResult<Ok, Err, Pos: ParsePos> {
    Okay(Ok, Pos),
    Error(Err),
    Panic(Err),
}

use ParseResult::*;
impl <Ok, Err, Pos: ParsePos> ParseResult<Ok, Err, Pos> {
    /// 
    /// Maps this result to a new one using the given function.
    /// 
    pub fn map<OOk, OErr, OPos: ParsePos, F: FnOnce(Self) -> ParseResult<OOk, OErr, OPos>>(self, f: F) -> ParseResult<OOk, OErr, OPos> {
        f(self)
    }

    /// 
    /// If this is an `Okay` or `OkayAdvance` result then this function maps the
    /// value of the result to a new value using the given function.
    /// 
    pub fn map_value<OOk, F: FnOnce(Ok) -> OOk>(self, f: F) -> ParseResult<OOk, Err, Pos> {
        match self {
            Okay(ok, pos) => Okay(f(ok), pos),
            Error(err) => Error(err),
            Panic(err) => Panic(err),
        }
    }

    /// 
    /// Maps the position of this result to a new one.
    /// 
    pub fn map_pos<OPos: ParsePos, F: FnOnce(Pos) -> OPos>(self, f: F) -> ParseResult<Ok, Err, OPos> {
        match self {
            Okay(ok, pos) => Okay(ok, f(pos)),
            Error(err) => Error(err),
            Panic(err) => Panic(err),
        }
    }

    /// 
    /// Maps the error of this result to a new one.
    /// 
    pub fn map_err<OErr, F: FnOnce(Err) -> OErr>(self, f: F) -> ParseResult<Ok, OErr, Pos> {
        match self {
            Okay(ok, pos) => Okay(ok, pos),
            Error(err) => Error(f(err)),
            Panic(err) => Panic(f(err)),
        }
    }

    /// 
    /// If this is an `Error`, then this method upgrades it to a `Panic`.
    /// 
    pub fn upgrade(self) -> Self {
        match self {
            Error(error) => Panic(error),
            p => p
        }
    }

    /// 
    /// If this is a `Panic`, then this method downgrades it to an `Error`.
    /// 
    pub fn downgrade(self) -> Self {
        match self {
            Panic(error) => Error(error),
            p => p
        }
    }
}


