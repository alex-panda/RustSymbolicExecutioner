use std::mem::MaybeUninit;

use crate::parser::{Span, ParseContext};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

use ParseResult::*;


impl <Ok, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<[Ok; N], Err, Store, Pos, V> for [Child; N] {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<[Ok; N], Err, Pos> {
        let mut accume: [MaybeUninit<Ok>; N] = unsafe { MaybeUninit::<[MaybeUninit<Ok>; N]>::uninit().assume_init() };
        let mut curr_pos = cxt.pos.clone();

        for len in 0..N {
            match self[len].parse(cxt.with_pos(curr_pos.clone())) {
                Okay(ok, advance) => {
                    // successfull parse so write the value
                    accume[len].write(ok);
                    curr_pos = advance;
                },
                Error(e) => {
                    for i in 0..len { unsafe { accume[i].assume_init_drop() } }
                    return Error(e);
                },
                Panic(e) => {
                    for i in 0..len { unsafe { accume[i].assume_init_drop() } }
                    return Panic(e);
                },
            }
        }

        ParseResult::Okay(accume.map(|v| unsafe { v.assume_init() }), curr_pos)
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = cxt.pos.clone();

        // parse each child
        for child in self {
            match child.parse_span(cxt.with_pos(curr_pos.clone())) {
                Okay(_, advance) => { curr_pos = advance },
                Error(error) => return Error(error),
                Panic(error) => return Panic(error),
            }
        }

        // return the parsed span
        Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
    }
}
