use std::ops::{RangeBounds, Bound, RangeTo, RangeFull, RangeFrom, Range, RangeToInclusive, RangeInclusive};

use super::super::{ParseNode, ParsePos, ParseStore, ParseResult, Span, UnexpectedEndError, ValueOutsideRangeError, ParseContext};

use ParseResult::*;

macro_rules! impl_range {
    ($rangeboundtype: ty, $t: ty) => {
        impl <Err: From<UnexpectedEndError<Pos>> + From<ValueOutsideRangeError<Pos, $t>>, Store: ParseStore<Pos, $t> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, $t> for $rangeboundtype {
            fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, $t>) -> ParseResult<Span<Pos>, Err, Pos> {
                let mut curr_pos = cxt.pos.clone();

                match cxt.store.value_at(&mut curr_pos) {
                    Some(actual_char) => {
                        if !self.contains(&actual_char) {
                            Error(ValueOutsideRangeError { pos: cxt.pos, found: actual_char, range_start: self.start_bound().cloned(), range_end: self.end_bound().cloned() }.into())
                        } else {
                            Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
                        }
                    },
                    None => Error(UnexpectedEndError { pos: curr_pos }.into()),
                }
            }
        }
    };
}

impl_range!(RangeFull, char);

impl_range!((Bound<char>, Bound<char>), char);
impl_range!(Range<char>, char);
impl_range!(RangeFrom<char>, char);
impl_range!(RangeInclusive<char>, char);
impl_range!(RangeTo<char>, char);
impl_range!(RangeToInclusive<char>, char);

impl_range!((Bound<u8>, Bound<u8>), u8);
impl_range!(Range<u8>, u8);
impl_range!(RangeFrom<u8>, u8);
impl_range!(RangeInclusive<u8>, u8);
impl_range!(RangeTo<u8>, u8);
impl_range!(RangeToInclusive<u8>, u8);

impl_range!((Bound<u16>, Bound<u16>), u16);
impl_range!(Range<u16>, u16);
impl_range!(RangeFrom<u16>, u16);
impl_range!(RangeInclusive<u16>, u16);
impl_range!(RangeTo<u16>, u16);
impl_range!(RangeToInclusive<u16>, u16);

impl_range!((Bound<u32>, Bound<u32>), u32);
impl_range!(Range<u32>, u32);
impl_range!(RangeFrom<u32>, u32);
impl_range!(RangeInclusive<u32>, u32);
impl_range!(RangeTo<u32>, u32);
impl_range!(RangeToInclusive<u32>, u32);

impl_range!((Bound<u64>, Bound<u64>), u64);
impl_range!(Range<u64>, u64);
impl_range!(RangeFrom<u64>, u64);
impl_range!(RangeInclusive<u64>, u64);
impl_range!(RangeTo<u64>, u64);
impl_range!(RangeToInclusive<u64>, u64);

impl_range!((Bound<usize>, Bound<usize>), usize);
impl_range!(Range<usize>, usize);
impl_range!(RangeFrom<usize>, usize);
impl_range!(RangeInclusive<usize>, usize);
impl_range!(RangeTo<usize>, usize);
impl_range!(RangeToInclusive<usize>, usize);

impl_range!((Bound<u128>, Bound<u128>), u128);
impl_range!(Range<u128>, u128);
impl_range!(RangeFrom<u128>, u128);
impl_range!(RangeInclusive<u128>, u128);
impl_range!(RangeTo<u128>, u128);
impl_range!(RangeToInclusive<u128>, u128);

impl_range!((Bound<i8>, Bound<i8>), i8);
impl_range!(Range<i8>, i8);
impl_range!(RangeFrom<i8>, i8);
impl_range!(RangeInclusive<i8>, i8);
impl_range!(RangeTo<i8>, i8);
impl_range!(RangeToInclusive<i8>, i8);

impl_range!((Bound<i16>, Bound<i16>), i16);
impl_range!(Range<i16>, i16);
impl_range!(RangeFrom<i16>, i16);
impl_range!(RangeInclusive<i16>, i16);
impl_range!(RangeTo<i16>, i16);
impl_range!(RangeToInclusive<i16>, i16);

impl_range!((Bound<i32>, Bound<i32>), i32);
impl_range!(Range<i32>, i32);
impl_range!(RangeFrom<i32>, i32);
impl_range!(RangeInclusive<i32>, i32);
impl_range!(RangeTo<i32>, i32);
impl_range!(RangeToInclusive<i32>, i32);

impl_range!((Bound<i64>, Bound<i64>), i64);
impl_range!(Range<i64>, i64);
impl_range!(RangeFrom<i64>, i64);
impl_range!(RangeInclusive<i64>, i64);
impl_range!(RangeTo<i64>, i64);
impl_range!(RangeToInclusive<i64>, i64);

impl_range!((Bound<isize>, Bound<isize>), isize);
impl_range!(Range<isize>, isize);
impl_range!(RangeFrom<isize>, isize);
impl_range!(RangeInclusive<isize>, isize);
impl_range!(RangeTo<isize>, isize);
impl_range!(RangeToInclusive<isize>, isize);

impl_range!((Bound<i128>, Bound<i128>), i128);
impl_range!(Range<i128>, i128);
impl_range!(RangeFrom<i128>, i128);
impl_range!(RangeInclusive<i128>, i128);
impl_range!(RangeTo<i128>, i128);
impl_range!(RangeToInclusive<i128>, i128);

impl_range!((Bound<f32>, Bound<f32>), f32);
impl_range!(Range<f32>, f32);
impl_range!(RangeFrom<f32>, f32);
impl_range!(RangeInclusive<f32>, f32);
impl_range!(RangeTo<f32>, f32);
impl_range!(RangeToInclusive<f32>, f32);

impl_range!((Bound<f64>, Bound<f64>), f64);
impl_range!(Range<f64>, f64);
impl_range!(RangeFrom<f64>, f64);
impl_range!(RangeInclusive<f64>, f64);
impl_range!(RangeTo<f64>, f64);
impl_range!(RangeToInclusive<f64>, f64);


macro_rules! impl_u32_range_for_char {
    ($rangeboundtype: ty) => {
        impl <Err: From<UnexpectedEndError<Pos>> + From<ValueOutsideRangeError<Pos, u32>>, Store: ParseStore<Pos, char> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, char> for $rangeboundtype {
            fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, char>) -> ParseResult<Span<Pos>, Err, Pos> {
                let mut curr_pos = cxt.pos.clone();

                match cxt.store.value_at(&mut curr_pos) {
                    Some(actual_char) => {
                        if !self.contains(&(actual_char as u32)) {
                            Error(ValueOutsideRangeError { pos: cxt.pos, found: actual_char as u32, range_start: self.start_bound().cloned(), range_end: self.end_bound().cloned() }.into())
                        } else {
                            Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
                        }
                    },
                    None => Error(UnexpectedEndError { pos: curr_pos }.into()),
                }
            }
        }
    };
}

impl_u32_range_for_char!((Bound<u32>, Bound<u32>));
impl_u32_range_for_char!(Range<u32>);
impl_u32_range_for_char!(RangeFrom<u32>);
impl_u32_range_for_char!(RangeInclusive<u32>);
impl_u32_range_for_char!(RangeTo<u32>);
impl_u32_range_for_char!(RangeToInclusive<u32>);
