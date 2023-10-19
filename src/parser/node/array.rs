use std::mem::MaybeUninit;

use crate::parser::Span;

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

use ParseResult::*;


impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<[Ok; N], Err, Store, Pos, V> for [Child; N] {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<[Ok; N], Err, Pos> {
        let mut accume: [MaybeUninit<Ok>; N] = unsafe { MaybeUninit::<[MaybeUninit<Ok>; N]>::uninit().assume_init() };
        let mut curr_pos = pos;

        for len in 0..N {
            match self[len].parse(store, curr_pos.clone()) {
                Okay(ok) => { accume[len].write(ok); },
                OkayAdvance(ok, advance) => {
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

        ParseResult::OkayAdvance(accume.map(|v| unsafe { v.assume_init() }), curr_pos)
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut curr_pos = pos.clone();

        // parse each child
        for child in self {
            match child.parse_span(store, curr_pos.clone()) {
                Okay(_) => {},
                OkayAdvance(_, advance) => { curr_pos = advance },
                Error(error) => return Error(error),
                Panic(error) => return Panic(error),
            }
        }

        // return the parsed span
        OkayAdvance(Span::new(pos, curr_pos.clone()), curr_pos)
    }
}
