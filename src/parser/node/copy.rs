use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError, Span};

use ParseResult::*;

macro_rules! value {
    ($t: ident) => {
        impl <Err: From<UnexpectedValueError<Pos, $t>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, $t>, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, $t> for $t {
            fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
                let mut curr_pos = pos.clone();
                match store.value_at(&mut curr_pos) {
                    Some(value) => {
                        if value == *self {
                            Okay(Span::new(pos, curr_pos.clone()), curr_pos)
                        } else {
                            Error(UnexpectedValueError { pos, found: value, expected: *self }.into())
                        }
                    },
                    None => Error(UnexpectedEndError { pos }.into()),
                }
            }
        }
    };
}

value!(u8);
value!(u16);
value!(u32);
value!(u64);
value!(usize);
value!(u128);

value!(i8);
value!(i16);
value!(i32);
value!(i64);
value!(isize);
value!(i128);

value!(char);

value!(f32);
value!(f64);


