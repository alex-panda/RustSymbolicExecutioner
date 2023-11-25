use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult, Span};

use ParseResult::*;

// impl parse node for empty tuple
impl <Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<(), Err, Store, Pos, V> for () {
    fn parse(&self, _store: &Store, pos: Pos) -> ParseResult<(), Err, Pos> {
        Okay((), pos)
    }
}

// implement ParseNode for tuples with up to (and including) 32 items in them

macro_rules! impl_tuple {
    ($($child_id: ident | $ok_id: ident | $num: tt),*) => {
        impl <$($ok_id),*, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, $($child_id: ParseNode<$ok_id, Err, Store, Pos, V>),*> ParseNode<($($ok_id),*,), Err, Store, Pos, V> for ($($child_id),*,) {
            fn parse(&self, store: &Store, mut pos: Pos) -> ParseResult<($($ok_id),*,), Err, Pos> {
                Okay((
                    $(match self.$num.parse(store, pos.clone()) {
                        ParseResult::Okay(v1, new_pos) => { pos = new_pos; v1 },
                        ParseResult::Error(err) => { return Error(err) },
                        ParseResult::Panic(err) => { return Panic(err) },
                    }),*,
                ), pos)
            }

            fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
                let mut curr_pos = pos.clone();

                $(match self.$num.parse(store, curr_pos.clone()) {
                    ParseResult::Okay(_, new_pos) => { curr_pos = new_pos; },
                    ParseResult::Error(err) => { return Error(err) },
                    ParseResult::Panic(err) => { return Panic(err) },
                });*

                Okay(Span::new(pos, curr_pos.clone()), curr_pos)
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



