use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, AllChildrenFailedError, ZSTNode, Span};

use std::fmt::Debug;
use ParseResult::*;
use zst::ZST;

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
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`OneOf` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
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

        Error(AllChildrenFailedError { pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`OneOf` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }
}

macro_rules! impl_one_of {
    ($fn_id: ident, $node_id: ident, $any_of_id: ident, $num: tt, $($lower_child_id: ident | $child_id: ident | $lower_ok_id: ident | $ok_id: ident),*) => {
        #[allow(non_snake_case)]
        pub fn $fn_id<$($child_id: ParseNode<$ok_id, Err, Store, Pos, V>),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>($($lower_child_id: $child_id),*) -> $node_id<$($child_id),*, $($ok_id),*, Err, Store, Pos, V> {
            $node_id {
                _zst: ZSTNode::default(),
                $($lower_child_id),*,
                $($lower_ok_id: Default::default()),*,
            }
        }

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $node_id<$($child_id),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
            _zst: ZSTNode<(), Err, Store, Pos, V>,
            $($lower_child_id:$child_id),*,
            $($lower_ok_id: ZST<$ok_id>),*,
        }

        impl <$($child_id: ParseNode<$ok_id, Err, Store, Pos, V>),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> ParseNode<$any_of_id<$($ok_id),*>, Err, Store, Pos, V> for $node_id<$($child_id),*, $($ok_id),*, Err, Store, Pos, V> {
            fn parse(&self, store: &Store, pos: Pos) -> ParseResult<$any_of_id<$($ok_id),*>, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.parse(store, pos.clone()) {
                        Okay(value, advance) => return Okay($any_of_id::$child_id(value), advance),
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

        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $any_of_id<$($ok_id),*> {
            $($child_id($ok_id)),*,
        }
    };
}

impl_one_of!(OneOf2, OneOf2Node, AnyOf2, 2, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2);
impl_one_of!(OneOf3, OneOf3Node, AnyOf3, 3, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3);
impl_one_of!(OneOf4, OneOf4Node, AnyOf4, 4, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4);
impl_one_of!(OneOf5, OneOf5Node, AnyOf5, 5, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5);
impl_one_of!(OneOf6, OneOf6Node, AnyOf6, 6, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6);
impl_one_of!(OneOf7, OneOf7Node, AnyOf7, 7, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7);
impl_one_of!(OneOf8, OneOf8Node, AnyOf8, 8, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8);
impl_one_of!(OneOf9, OneOf9Node, AnyOf9, 9, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9);