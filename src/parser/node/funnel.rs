use crate::parser::ParseContext;

use super::super::{ParseStore, ParsePos, ParseValue, ParseNode, ParseResult, AllChildrenFailedError, ZSTNode, Span};

use ParseResult::*;
use zst::ZST;


/// 
/// Returns a node that that tries each of its children at the current parse
/// position in the order that they are given in and returns the result of the
/// first child that parses successfully. The node fails to parse if all
/// children fail to parse at the current position.
/// 
/// If you would like to know which child parsed successfully, use a `OneOf*`
/// node instead (where `*` is a number of children the node has).
/// 
#[allow(non_snake_case)]
pub fn Funnel<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize>(children: [Child; N]) -> FunnelNode<Child, Ok, Err, Store, Pos, V, N> {
    FunnelNode {
        children,
        _zst: ZSTNode::default(),
        _ok1: ZST::default(),
    }
}

pub struct FunnelNode<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize> {
    children: [Child; N],
    _zst: ZSTNode<Child, Err, Store, Pos, V>,
    _ok1: ZST<Ok>,
}


impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<Ok, Err, Store, Pos, V> for FunnelNode<Child, Ok, Err, Store, Pos, V, N> {
    fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.parse(cxt.clone()) {
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos: cxt.pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`Funnel` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }

    fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.parse_span(cxt.clone()) {
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos: cxt.pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`Funnel` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }
}

impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, const N: usize> Clone for FunnelNode<Child, Ok, Err, Store, Pos, V, N> {
    fn clone(&self) -> Self {
        Self { children: self.children.clone(), _zst: self._zst.clone(), _ok1: Default::default() }
    }
}

macro_rules! impl_one_of {
    ($fn_id: ident, $node_id: ident, $num: tt $(, $lower_child_id: ident | $child_id: ident)*) => {
        /// 
        /// Returns a node that tries each of its child nodes in the order
        /// they are given and returns the result of the first one that
        /// parses successfully. If you would like to know which node parses
        /// successfully, or allow each node to return a different type, use
        /// a `OneOf*` node instead (where `*` is the number of children the
        /// node has).
        /// 
        #[allow(non_snake_case)]
        pub fn $fn_id<$($child_id: ParseNode<Ok, Err, Store, Pos, V>,)* Ok, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>($($lower_child_id: $child_id,)*) -> $node_id<$($child_id,)* Ok, Err, Store, Pos, V> {
            $node_id {
                _zst: ZSTNode::default(),
                $($lower_child_id,)*
            }
        }

        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $node_id<$($child_id,)* Ok, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
            _zst: ZSTNode<Ok, Err, Store, Pos, V>,
            $($lower_child_id:$child_id,)*
        }

        impl <$($child_id: ParseNode<Ok, Err, Store, Pos, V>,)* Ok, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<Ok, Err, Store, Pos, V> for $node_id<$($child_id,)* Ok, Err, Store, Pos, V> {
            fn parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.parse(cxt.clone()) {
                        Okay(value, advance) => return Okay(value, advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    },
                )*];

                Error(Err::from(AllChildrenFailedError { pos: cxt.pos, errors }))
            }

            fn parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.parse_span(cxt.clone()) {
                        Okay(_, advance) => return Okay(Span::new(cxt.pos, advance.clone()), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    },
                )*];

                Error(Err::from(AllChildrenFailedError { pos: cxt.pos, errors }))
            }
        }
    };
}

impl_one_of!(Funnel0, Funnel0Node, 0);
impl_one_of!(Funnel1, Funnel1Node, 1, child1 | Child1);
impl_one_of!(Funnel2, Funnel2Node, 2, child1 | Child1, child2 | Child2);
impl_one_of!(Funnel3, Funnel3Node, 3, child1 | Child1, child2 | Child2, child3 | Child3);
impl_one_of!(Funnel4, Funnel4Node, 4, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4);
impl_one_of!(Funnel5, Funnel5Node, 5, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5);
impl_one_of!(Funnel6, Funnel6Node, 6, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6);
impl_one_of!(Funnel7, Funnel7Node, 7, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7);
impl_one_of!(Funnel8, Funnel8Node, 8, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8);
impl_one_of!(Funnel9, Funnel9Node, 9, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9);
impl_one_of!(Funnel10, Funnel10Node, 10, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10);
impl_one_of!(Funnel11, Funnel11Node, 11, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11);
impl_one_of!(Funnel12, Funnel12Node, 12, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12);
impl_one_of!(Funnel13, Funnel13Node, 13, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13);
impl_one_of!(Funnel14, Funnel14Node, 14, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14);
impl_one_of!(Funnel15, Funnel15Node, 15, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15);
impl_one_of!(Funnel16, Funnel16Node, 16, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16);
impl_one_of!(Funnel17, Funnel17Node, 17, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17);
impl_one_of!(Funnel18, Funnel18Node, 18, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18);
impl_one_of!(Funnel19, Funnel19Node, 19, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19);
impl_one_of!(Funnel20, Funnel20Node, 20, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20);
impl_one_of!(Funnel21, Funnel21Node, 21, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21);
impl_one_of!(Funnel22, Funnel22Node, 22, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22);
impl_one_of!(Funnel23, Funnel23Node, 23, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23);
impl_one_of!(Funnel24, Funnel24Node, 24, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24);
impl_one_of!(Funnel25, Funnel25Node, 25, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25);
impl_one_of!(Funnel26, Funnel26Node, 26, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26);
impl_one_of!(Funnel27, Funnel27Node, 27, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26, child27 | Child27);
impl_one_of!(Funnel28, Funnel28Node, 28, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26, child27 | Child27, child28 | Child28);
impl_one_of!(Funnel29, Funnel29Node, 29, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26, child27 | Child27, child28 | Child28, child29 | Child29);
impl_one_of!(Funnel30, Funnel30Node, 30, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26, child27 | Child27, child28 | Child28, child29 | Child29, child30 | Child30);
impl_one_of!(Funnel31, Funnel31Node, 31, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26, child27 | Child27, child28 | Child28, child29 | Child29, child30 | Child30, child31 | Child31);
impl_one_of!(Funnel32, Funnel32Node, 32, child1 | Child1, child2 | Child2, child3 | Child3, child4 | Child4, child5 | Child5, child6 | Child6, child7 | Child7, child8 | Child8, child9 | Child9, child10 | Child10, child11 | Child11, child12 | Child12, child13 | Child13, child14 | Child14, child15 | Child15, child16 | Child16, child17 | Child17, child18 | Child18, child19 | Child19, child20 | Child20, child21 | Child21, child22 | Child22, child23 | Child23, child24 | Child24, child25 | Child25, child26 | Child26, child27 | Child27, child28 | Child28, child29 | Child29, child30 | Child30, child31 | Child31, child32 | Child32);