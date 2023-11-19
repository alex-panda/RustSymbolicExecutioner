use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, Span};
use paste::paste;

use ParseResult::*;

macro_rules! handle_result {
    ($res: expr, $pos: expr) => {
        {
            match $res {
                ParseResult::Okay(v1) => { v1 },
                ParseResult::OkayAdvance(v1, new_pos) => { $pos = new_pos; v1 },
                ParseResult::Error(err) => { return Error(err) },
                ParseResult::Panic(err) => { return Panic(err) },
            }
        }
    };
}

// impl parse node for empty tuple
impl <Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for () {
    fn parse(&self, _store: &Store, _pos: Pos) -> ParseResult<(), Err, Pos> {
        Okay(())
    }
}

// impl parse node for tuple with one element
impl <Ok0, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child0: ParseNode<Ok0, Err, Store, Pos, V>> ParseNode<(Ok0,), Err, Store, Pos, V> for (Child0,) {
    fn parse(&self, store: &Store, mut pos: Pos) -> ParseResult<(Ok0,), Err, Pos> {
        OkayAdvance((
            handle_result!(self.0.parse(store, pos.clone()), pos),
        ), pos)
    }
}

// implement ParseNode for tuples with up to (and including) 32 items in them

macro_rules! impl_tuple {
    ($($num: tt),*) => {
        paste! {
            impl <Ok0 $(,[<Ok $num>])*, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child0: ParseNode<Ok0, Err, Store, Pos, V> $(, [<Child $num>]: ParseNode<[<Ok $num>], Err, Store, Pos, V>)*> ParseNode<(Ok0 $(,[<Ok $num>])*), Err, Store, Pos, V> for (Child0 $(,[<Child $num>])*) {
                fn parse(&self, store: &Store, mut pos: Pos) -> ParseResult<(Ok0, $([<Ok $num>],)*), Err, Pos> {
                    OkayAdvance((
                        handle_result!(self.0.parse(store, pos.clone()), pos),
                        $(handle_result!(self.$num.parse(store, pos.clone()), pos),)*
                    ), pos)
                }

                fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
                    let mut curr_pos = pos.clone();

                    handle_result!(self.0.parse(store, pos.clone()), curr_pos);
                    $(handle_result!(self.$num.parse(store, pos.clone()), curr_pos);)*

                    OkayAdvance(Span::new(pos, curr_pos.clone()), curr_pos)
                }
            }
        }
    };
}

impl_tuple!(1);
impl_tuple!(1, 2);
impl_tuple!(1, 2, 3);
impl_tuple!(1, 2, 3, 4);
impl_tuple!(1, 2, 3, 4, 5);
impl_tuple!(1, 2, 3, 4, 5, 6);
impl_tuple!(1, 2, 3, 4, 5, 6, 7);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30);
//impl_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);