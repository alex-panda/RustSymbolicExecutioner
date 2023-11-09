use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, AllChildrenFailedError, ZSTNode, Span};

use ParseResult::*;
use zst::ZST;

use paste::paste;


/// 
/// The constructor for the `OneOf` parse node.
/// 
/// A `OneOf` parse node tries to parse each of its children one by one and
/// returns the result of the firsth child that parses successfully.
/// 
/// If you would like to know what child parsed successfully or have the
/// children be of different types, you should use one of the `OneOf*` nodes
/// instead (where `*` is the written out number of children from `Two` to
/// `Nine` (i.e. `Two`, `Three`, ..., `Nine`).
/// 
/// If you need to choose between more than nine children, you should make one
/// of the `OneOf*` node's children itself a `OneOf*` node.
/// 
#[allow(non_snake_case)]
pub fn OneOf<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize>(children: [Child; N]) -> OneOfNode<Child, Ok, Err, Store, Pos, V, N> {
    OneOfNode {
        children,
        _zst: ZSTNode::default(),
        _ok1: ZST::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneOfNode<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize> {
    children: [Child; N],
    _zst: ZSTNode<Child, Err, Store, Pos, V>,
    _ok1: ZST<Ok>,
}


impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<Ok, Err, Store, Pos, V> for OneOfNode<Child, Ok, Err, Store, Pos, V, N> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.parse(store, pos.clone()) {
                Okay(value) => return Okay(value),
                OkayAdvance(value, advance) => return OkayAdvance(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`OneOf` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }
}

macro_rules! impl_one_of {
    ($num_children: tt, $name: ident $(, $num: tt)+) => {
        paste! {
            #[allow(non_snake_case)]
            pub fn [<OneOf $name>]<Child1 $(, [<Child $num>])*, Ok1 $(,[<Ok $num>])*, Err: From<AllChildrenFailedError<Pos, Err, $num_children>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child1: Child1 $(,[<child $num>]: [<Child $num>])*) -> [<OneOf $name Node>]<Child1 $(, [<Child $num>])*, Ok1 $(, [<Ok $num>])*, Err, Store, Pos, V> {
                [<OneOf $name Node>] {
                    _zst: ZSTNode::default(),
                    child1,
                    $([<child $num>],)*
                    _ok1: ZST::default(),
                    $([<_ok $num>]: ZST::default(),)*
                }
            }

            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct [<OneOf $name Node>]<Child1 $(, [<Child $num>])*, Ok1 $(, [<Ok $num>])*, Err: From<AllChildrenFailedError<Pos, Err, $num_children>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
                _zst: ZSTNode<(), Err, Store, Pos, V>,
                pub child1: Child1,
                $(pub [<child $num>]: [<Child $num>],)*
                _ok1: ZST<Ok1>,
                $([<_ok $num>]: ZST<[<Ok $num>]>,)*
            }

            #[derive(Default)]
            pub struct [<OneOf $name NodeData>]<T1 $(,[<T $num>])*> {
                pub t1: T1,
                $(pub [<t $num>]: [<T $num>],)*
            }

            impl <Ok1 $(,[<Ok $num>])*, Err: From<AllChildrenFailedError<Pos, Err, $num_children>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child1: ParseNode<Ok1, Err, Store, Pos, V> $(,[<Child $num>]: ParseNode<[<Ok $num>], Err, Store, Pos, V>)*> ParseNode<[<AnyOf $name>]<Ok1 $(,[<Ok $num>])*>, Err, Store, Pos, V> for [<OneOf $name Node>]<Child1 $(,[<Child $num>])*, Ok1 $(,[<Ok $num>])*, Err, Store, Pos, V> {
                fn parse(&self, store: &Store, pos: Pos) -> ParseResult<[<AnyOf $name>]<Ok1 $(,[<Ok $num>])*>, Err, Pos> {
                    let error1 = match self.child1.parse(store, pos.clone()) {
                        Okay(value) => return Okay([<AnyOf $name>]::Child1(value)),
                        OkayAdvance(value, advance) => return OkayAdvance([<AnyOf $name>]::Child1(value), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    };

                    $(
                        let [<error $num>] = match self.[<child $num>].parse(store, pos.clone()) {
                            Okay(value) => return Okay([<AnyOf $name>]::[<Child $num>](value)),
                            OkayAdvance(value, advance) => return OkayAdvance([<AnyOf $name>]::[<Child $num>](value), advance),
                            Error(error) => error,
                            Panic(error) => return Panic(error),
                        };
                    )*

                    Error(AllChildrenFailedError { pos, errors: [error1 $(,[<error $num>])*] }.into())
                }

                fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
                    let error1 = match self.child1.parse_span(store, pos.clone()) {
                        Okay(_) => return Okay(Span::new(pos.clone(), pos)),
                        OkayAdvance(_, advance) => return OkayAdvance(Span::new(pos, advance.clone()), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    };

                    $(
                        let [<error $num>] = match self.[<child $num>].parse_span(store, pos.clone()) {
                            Okay(_) => return Okay(Span::new(pos.clone(), pos)),
                            OkayAdvance(_, advance) => return OkayAdvance(Span::new(pos, advance.clone()), advance),
                            Error(error) => error,
                            Panic(error) => return Panic(error),
                        };
                    )*

                    Error(AllChildrenFailedError { pos, errors: [error1 $(,[<error $num>])*] }.into())
                }
            }

            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum [<AnyOf $name>]<O1 $(,[<O $num>])*> {
                Child1(O1),
                $([<Child $num>]([<O $num>]),)*
            }
        }
    };
}

impl_one_of!(2, Two  , 2);
impl_one_of!(3, Three, 2, 3);
impl_one_of!(4, Four , 2, 3, 4);
impl_one_of!(5, Five , 2, 3, 4, 5);
impl_one_of!(6, Six  , 2, 3, 4, 5, 6);
impl_one_of!(7, Seven, 2, 3, 4, 5, 6, 7);
impl_one_of!(8, Eight, 2, 3, 4, 5, 6, 7, 8);
impl_one_of!(9, Nine , 2, 3, 4, 5, 6, 7, 8, 9);