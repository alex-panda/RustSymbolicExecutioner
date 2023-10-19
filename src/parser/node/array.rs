use std::mem::MaybeUninit;

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};



impl <Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<[Ok; N], Err, Store, Pos, V> for [Child; N] {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<[Ok; N], Err, Pos> {
        let mut accume: [MaybeUninit<Ok>; N] = unsafe { MaybeUninit::<[MaybeUninit<Ok>; N]>::uninit().assume_init() };
        let mut curr_pos = pos;

        use ParseResult::*;
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
}
