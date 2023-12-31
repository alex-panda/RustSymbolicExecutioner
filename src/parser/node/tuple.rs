use crate::parser::ParseContext;

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, Span};

use ParseResult::*;

// impl parse node for empty tuple
impl <Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for () {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<(), Err, Pos> {
        Okay((), cxt.pos)
    }
}

// implement ParseNode for tuples with up to (and including) 32 items in them

macro_rules! impl_tuple {
    ($($child_id: ident | $ok_id: ident | $num: tt),*) => {
        impl <$($ok_id),*, Err, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, $($child_id: ParseNode<$ok_id, Err, Store, Pos, V>),*> ParseNode<($($ok_id),*,), Err, Store, Pos, V> for ($($child_id),*,) {
            fn parse<'a>(&self, mut cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<($($ok_id),*,), Err, Pos> {
                Okay((
                    $(match self.$num.parse(cxt.clone()) {
                        ParseResult::Okay(v1, new_pos) => { cxt.pos = new_pos; v1 },
                        ParseResult::Error(err) => { return Error(err) },
                        ParseResult::Panic(err) => { return Panic(err) },
                    }),*,
                ), cxt.pos)
            }

            fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
                let mut _curr_pos = cxt.pos.clone();

                $(match self.$num.parse_span(cxt.clone()) {
                    ParseResult::Okay(_, new_pos) => { _curr_pos = new_pos; },
                    ParseResult::Error(err) => { return Error(err) },
                    ParseResult::Panic(err) => { return Panic(err) },
                });*

                Okay(Span::new(cxt.pos, _curr_pos.clone()), _curr_pos)
            }
        }
    };
}

impl_tuple!(Child0 | Ok0 | 0);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26, Child27 | Ok27 | 27);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26, Child27 | Ok27 | 27, Child28 | Ok28 | 28);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26, Child27 | Ok27 | 27, Child28 | Ok28 | 28, Child29 | Ok29 | 29);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26, Child27 | Ok27 | 27, Child28 | Ok28 | 28, Child29 | Ok29 | 29, Child30 | Ok30 | 30);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26, Child27 | Ok27 | 27, Child28 | Ok28 | 28, Child29 | Ok29 | 29, Child30 | Ok30 | 30, Child31 | Ok31 | 31);
impl_tuple!(Child0 | Ok0 | 0, Child1 | Ok1 | 1, Child2 | Ok2 | 2, Child3 | Ok3 | 3, Child4 | Ok4 | 4, Child5 | Ok5 | 5, Child6 | Ok6 | 6, Child7 | Ok7 | 7, Child8 | Ok8 | 8, Child9 | Ok9 | 9, Child10 | Ok10 | 10, Child11 | Ok11 | 11, Child12 | Ok12 | 12, Child13 | Ok13 | 13, Child14 | Ok14 | 14, Child15 | Ok15 | 15, Child16 | Ok16 | 16, Child17 | Ok17 | 17, Child18 | Ok18 | 18, Child19 | Ok19 | 19, Child20 | Ok20 | 20, Child21 | Ok21 | 21, Child22 | Ok22 | 22, Child23 | Ok23 | 23, Child24 | Ok24 | 24, Child25 | Ok25 | 25, Child26 | Ok26 | 26, Child27 | Ok27 | 27, Child28 | Ok28 | 28, Child29 | Ok29 | 29, Child30 | Ok30 | 30, Child31 | Ok31 | 31, Child32 | Ok32 | 32);



