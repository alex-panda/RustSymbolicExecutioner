use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, AllChildrenFailedError, ZSTNode, Span};

use ParseResult::*;
use zst::ZST;

use paste::paste;


/// 
/// Funnels its children (which must all must be of the same type) into one
/// output type.
/// 
/// If you would like to know which child parsed successfully use a `OneOf*`
/// node instead (where `*` is a number from 2 to 9 inclusive).
/// 
#[allow(non_snake_case)]
pub fn Funnel<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize>(children: [Child; N]) -> FunnelNode<Child, Ok, Err, Store, Pos, V, N> {
    FunnelNode {
        children,
        _zst: ZSTNode::default(),
        _ok1: ZST::default(),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunnelNode<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, const N: usize> {
    children: [Child; N],
    _zst: ZSTNode<Child, Err, Store, Pos, V>,
    _ok1: ZST<Ok>,
}


impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<Ok, Err, Store, Pos, V> for FunnelNode<Child, Ok, Err, Store, Pos, V, N> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.parse(store, pos.clone()) {
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`Funnel` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }

    fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.parse_span(store, pos.clone()) {
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`Funnel` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }
}

macro_rules! impl_one_of {
    ($num_children: tt, $name: tt, $($num: tt),+) => {
        paste! {
            /// 
            /// Returns a node that tries each of its child nodes in the order
            /// they are given and returns the result of the first one that
            /// parses successfully. If you would like to know which node parses
            /// successfully, or allow each node to return a different type, use
            /// a `OneOf*` node instead (where `*` is the number of children it
            /// has from 2 to 9 inclusive).
            /// 
            #[allow(non_snake_case)]
            pub fn [<Funnel $name>]<Child1 $(, [<Child $num>])*, Ok, Err: From<AllChildrenFailedError<Pos, Err, $num_children>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child1: Child1 $(,[<child $num>]: [<Child $num>])*) -> [<Funnel $name Node>]<Child1 $(, [<Child $num>])*, Ok, Err, Store, Pos, V> {
                [<Funnel $name Node>] {
                    _zst: ZSTNode::default(),
                    child1,
                    $([<child $num>],)*
                }
            }

            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct [<Funnel $name Node>]<Child1 $(, [<Child $num>])*, Ok, Err: From<AllChildrenFailedError<Pos, Err, $num_children>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
                _zst: ZSTNode<Ok, Err, Store, Pos, V>,
                pub child1: Child1,
                $(pub [<child $num>]: [<Child $num>],)*
            }

            impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, $num_children>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue, Child1: ParseNode<Ok, Err, Store, Pos, V> $(,[<Child $num>]: ParseNode<Ok, Err, Store, Pos, V>)*> ParseNode<Ok, Err, Store, Pos, V> for [<Funnel $name Node>]<Child1 $(,[<Child $num>])*, Ok, Err, Store, Pos, V> {
                fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
                    let error1 = match self.child1.parse(store, pos.clone()) {
                        Okay(value, advance) => return Okay(value, advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    };

                    $(
                        let [<error $num>] = match self.[<child $num>].parse(store, pos.clone()) {
                            Okay(value, advance) => return Okay(value, advance),
                            Error(error) => error,
                            Panic(error) => return Panic(error),
                        };
                    )*

                    Error(AllChildrenFailedError { pos, errors: [error1 $(,[<error $num>])*] }.into())
                }

                fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
                    let error1 = match self.child1.parse_span(store, pos.clone()) {
                        Okay(_, advance) => return Okay(Span::new(pos, advance.clone()), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    };

                    $(
                        let [<error $num>] = match self.[<child $num>].parse_span(store, pos.clone()) {
                            Okay(_, advance) => return Okay(Span::new(pos, advance.clone()), advance),
                            Error(error) => error,
                            Panic(error) => return Panic(error),
                        };
                    )*

                    Error(AllChildrenFailedError { pos, errors: [error1 $(,[<error $num>])*] }.into())
                }
            }
        }
    };
}

impl_one_of!(2, 2, 2);
impl_one_of!(3, 3, 2, 3);
impl_one_of!(4, 4, 2, 3, 4);
impl_one_of!(5, 5, 2, 3, 4, 5);
impl_one_of!(6, 6, 2, 3, 4, 5, 6);
impl_one_of!(7, 7, 2, 3, 4, 5, 6, 7);
impl_one_of!(8, 8, 2, 3, 4, 5, 6, 7, 8);
impl_one_of!(9, 9, 2, 3, 4, 5, 6, 7, 8, 9);