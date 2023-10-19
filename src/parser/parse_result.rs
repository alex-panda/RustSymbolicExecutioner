use super::ParsePos;

pub enum ParseResult<Ok, Err, Pos: ParsePos> {
    Okay(Ok),
    OkayAdvance(Ok, Pos),
    Error(Err),
    Panic(Err),
}

use ParseResult::*;
impl <Ok, Err, Pos: ParsePos> ParseResult<Ok, Err, Pos> {
    pub fn map<OOk, OErr, OPos: ParsePos, F: FnOnce(Self) -> ParseResult<OOk, OErr, OPos>>(self, f: F) -> ParseResult<OOk, OErr, OPos> {
        f(self)
    }

    pub fn map_value<OOk, F: FnOnce(Ok) -> OOk>(self, f: F) -> ParseResult<OOk, Err, Pos> {
        match self {
            Okay(ok) => Okay(f(ok)),
            OkayAdvance(ok, pos) => OkayAdvance(f(ok), pos),
            Error(err) => Error(err),
            Panic(err) => Panic(err),
        }
    }

    pub fn map_pos<OPos: ParsePos, F: FnOnce(Pos) -> OPos>(self, f: F) -> ParseResult<Ok, Err, OPos> {
        match self {
            Okay(ok) => Okay(ok),
            OkayAdvance(ok, pos) => OkayAdvance(ok, f(pos)),
            Error(err) => Error(err),
            Panic(err) => Panic(err),
        }
    }

    pub fn map_err<OErr, F: FnOnce(Err) -> OErr>(self, f: F) -> ParseResult<Ok, OErr, Pos> {
        match self {
            Okay(ok) => Okay(ok),
            OkayAdvance(ok, pos) => OkayAdvance(ok, pos),
            Error(err) => Error(f(err)),
            Panic(err) => Panic(f(err)),
        }
    }
}


