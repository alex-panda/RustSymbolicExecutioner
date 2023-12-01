use super::super::{ParseStore, ParsePos, ParseNode, ParseResult, UnexpectedValueError, UnexpectedEndError, Span, ParseContext};

use ParseResult::*;

macro_rules! value {
    ($t: ident) => {
        impl <Err: From<UnexpectedValueError<Pos, $t>> + From<UnexpectedEndError<Pos>>, Store: ParseStore<Pos, $t> + ?Sized, Pos: ParsePos> ParseNode<Span<Pos>, Err, Store, Pos, $t> for $t {
            fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, $t>) -> ParseResult<Span<Pos>, Err, Pos> {
                let mut curr_pos = cxt.pos.clone();
                match cxt.store.value_at(&mut curr_pos) {
                    Some(value) => {
                        if value == *self {
                            Okay(Span::new(cxt.pos, curr_pos.clone()), curr_pos)
                        } else {
                            Error(UnexpectedValueError { pos: cxt.pos, found: value, expected: *self }.into())
                        }
                    },
                    None => Error(UnexpectedEndError { pos: cxt.pos }.into()),
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


