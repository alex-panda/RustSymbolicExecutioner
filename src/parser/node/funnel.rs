
            /// 
            /// Returns a node that tries each of its child nodes in the order
            /// they are given and returns the result of the first one that
            /// parses successfully. If you would like to know which node parses
            /// successfully, or allow each node to return a different type, use
            /// a `OneOf*` node instead (where `*` is the number of children it
            /// has from 2 to 9 inclusive).
            /// 
            /// Funnel#

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, AllChildrenFailedError, ZSTNode, Span};

use ParseResult::*;
use zst::ZST;


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
    ($fn_id: ident, $node_id: ident, $num: tt, $($lower_child_id: ident | $child_id: ident),*) => {
        #[allow(non_snake_case)]
        pub fn $fn_id<$($child_id: ParseNode<Ok, Err, Store, Pos, V>),*, Ok, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>($($lower_child_id: $child_id),*) -> $node_id<$($child_id),*, Ok, Err, Store, Pos, V> {
            $node_id {
                _zst: ZSTNode::default(),
                $($lower_child_id),*,
            }
        }

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $node_id<$($child_id),*, Ok, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
            _zst: ZSTNode<Ok, Err, Store, Pos, V>,
            $($lower_child_id:$child_id),*,
        }

        impl <$($child_id: ParseNode<Ok, Err, Store, Pos, V>),*, Ok, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for $node_id<$($child_id),*, Ok, Err, Store, Pos, V> {
            fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.parse(store, pos.clone()) {
                        Okay(value, advance) => return Okay(value, advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    },
                )*];

                Error(Err::from(AllChildrenFailedError { pos, errors }))
            }

            fn parse_span(&self, store: &Store, pos: Pos) -> ParseResult<Span<Pos>, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.parse_span(store, pos.clone()) {
                        Okay(_, advance) => return Okay(Span::new(pos, advance.clone()), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    },
                )*];

                Error(Err::from(AllChildrenFailedError { pos, errors }))
            }
        }
    };
}

impl_one_of!(Funnel2, Funnel2Node, 2, child1 | Child1, child2 | Child2);
impl_one_of!(Funnel3, Funnel3Node, 3, child1 | Child1, child2 | Child2, child3 | Child3);
impl_one_of!(Funnel4, Funnel4Node, 4, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4);
impl_one_of!(Funnel5, Funnel5Node, 5, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5);
impl_one_of!(Funnel6, Funnel6Node, 6, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6);
impl_one_of!(Funnel7, Funnel7Node, 7, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7);
impl_one_of!(Funnel8, Funnel8Node, 8, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8);
impl_one_of!(Funnel9, Funnel9Node, 9, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9);