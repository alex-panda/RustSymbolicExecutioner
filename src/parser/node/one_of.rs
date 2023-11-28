use crate::parser::ParseContext;

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
/// instead (where `*` is the written number of children the node has).
/// 
#[allow(non_snake_case)]
pub fn OneOf<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize>(children: [Child; N]) -> OneOfNode<Child, Ok, Err, Store, Pos, V, N> {
    OneOfNode {
        children,
        _zst: ZSTNode::default(),
        _ok1: ZST::default(),
    }
}

pub struct OneOfNode<Child, Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, const N: usize> {
    children: [Child; N],
    _zst: ZSTNode<Child, Err, Store, Pos, V>,
    _ok1: ZST<Ok>,
}


impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>, const N: usize> ParseNode<Ok, Err, Store, Pos, V> for OneOfNode<Child, Ok, Err, Store, Pos, V, N> {
    fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Ok, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.do_parse(cxt.clone()) {
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos: cxt.pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`OneOf` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }

    fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
        let mut out = core::array::from_fn(|_| None);
        for (i, child) in self.children.iter().enumerate() {
            match child.do_parse_span(cxt.clone()) {
                Okay(value, advance) => return Okay(value, advance),
                Error(error) => out[i] = Some(error),
                Panic(error) => return Panic(error),
            }
        }

        Error(AllChildrenFailedError { pos: cxt.pos, errors: out.map(|v| if let Some(v) = v { v } else { panic!("`OneOf` node expected either success or {} errors, but less errors than expected were given", N) }) }.into())
    }
}

impl <Ok, Err: From<AllChildrenFailedError<Pos, Err, N>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V> + Clone, const N: usize> Clone for OneOfNode<Child, Ok, Err, Store, Pos, V, N> {
    fn clone(&self) -> Self {
        Self { children: self.children.clone(), _zst: self._zst.clone(), _ok1: Default::default() }
    }
}

macro_rules! impl_one_of {
    ($fn_id: ident, $node_id: ident, $any_of_id: ident, $num: tt, $($lower_child_id: ident | $child_id: ident | $lower_ok_id: ident | $ok_id: ident),*) => {
        #[allow(non_snake_case)]
        pub fn $fn_id<$($child_id: ParseNode<$ok_id, Err, Store, Pos, V>),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue>($($lower_child_id: $child_id),*) -> $node_id<$($child_id),*, $($ok_id),*, Err, Store, Pos, V> {
            $node_id {
                _zst: ZSTNode::default(),
                $($lower_child_id),*,
                $($lower_ok_id: Default::default()),*,
            }
        }

        pub struct $node_id<$($child_id),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
            _zst: ZSTNode<(), Err, Store, Pos, V>,
            $($lower_child_id:$child_id),*,
            $($lower_ok_id: ZST<$ok_id>),*,
        }

        impl <$($child_id: ParseNode<$ok_id, Err, Store, Pos, V>),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseNode<$any_of_id<$($ok_id),*>, Err, Store, Pos, V> for $node_id<$($child_id),*, $($ok_id),*, Err, Store, Pos, V> {
            fn do_parse<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<$any_of_id<$($ok_id),*>, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.do_parse(cxt.clone()) {
                        Okay(value, advance) => return Okay($any_of_id::$child_id(value), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    },
                )*];

                Error(Err::from(AllChildrenFailedError { pos: cxt.pos, errors }))
            }

            fn do_parse_span<'a>(&self, cxt: ParseContext<'a, Store, Pos, V>) -> ParseResult<Span<Pos>, Err, Pos> {
                let errors = [$(
                    match self.$lower_child_id.do_parse_span(cxt.clone()) {
                        Okay(_, advance) => return Okay(Span::new(cxt.pos, advance.clone()), advance),
                        Error(error) => error,
                        Panic(error) => return Panic(error),
                    },
                )*];

                Error(Err::from(AllChildrenFailedError { pos: cxt.pos, errors }))
            }
        }

        impl <$($child_id: ParseNode<$ok_id, Err, Store, Pos, V> + Clone),*, $($ok_id),*, Err: From<AllChildrenFailedError<Pos, Err, $num>>, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for $node_id<$($child_id),*, $($ok_id),*, Err, Store, Pos, V> {
            fn clone(&self) -> Self {
                Self {
                    _zst: ZSTNode::default(),
                    $($lower_child_id: self.$lower_child_id.clone()),*,
                    $($lower_ok_id: Default::default()),*,
                }
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
impl_one_of!(OneOf10, OneOf10Node, AnyOf10, 10, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10);
impl_one_of!(OneOf11, OneOf11Node, AnyOf11, 11, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11);
impl_one_of!(OneOf12, OneOf12Node, AnyOf12, 12, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12);
impl_one_of!(OneOf13, OneOf13Node, AnyOf13, 13, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13);
impl_one_of!(OneOf14, OneOf14Node, AnyOf14, 14, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14);
impl_one_of!(OneOf15, OneOf15Node, AnyOf15, 15, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15);
impl_one_of!(OneOf16, OneOf16Node, AnyOf16, 16, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16);
impl_one_of!(OneOf17, OneOf17Node, AnyOf17, 17, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17);
impl_one_of!(OneOf18, OneOf18Node, AnyOf18, 18, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18);
impl_one_of!(OneOf19, OneOf19Node, AnyOf19, 19, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19);
impl_one_of!(OneOf20, OneOf20Node, AnyOf20, 20, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20);
impl_one_of!(OneOf21, OneOf21Node, AnyOf21, 21, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21);
impl_one_of!(OneOf22, OneOf22Node, AnyOf22, 22, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22);
impl_one_of!(OneOf23, OneOf23Node, AnyOf23, 23, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23);
impl_one_of!(OneOf24, OneOf24Node, AnyOf24, 24, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24);
impl_one_of!(OneOf25, OneOf25Node, AnyOf25, 25, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25);
impl_one_of!(OneOf26, OneOf26Node, AnyOf26, 26, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26);
impl_one_of!(OneOf27, OneOf27Node, AnyOf27, 27, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26, child27 | Child27 | ok27 | Ok27);
impl_one_of!(OneOf28, OneOf28Node, AnyOf28, 28, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26, child27 | Child27 | ok27 | Ok27, child28 | Child28 | ok28 | Ok28);
impl_one_of!(OneOf29, OneOf29Node, AnyOf29, 29, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26, child27 | Child27 | ok27 | Ok27, child28 | Child28 | ok28 | Ok28, child29 | Child29 | ok29 | Ok29);
impl_one_of!(OneOf30, OneOf30Node, AnyOf30, 30, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26, child27 | Child27 | ok27 | Ok27, child28 | Child28 | ok28 | Ok28, child29 | Child29 | ok29 | Ok29, child30 | Child30 | ok30 | Ok30);
impl_one_of!(OneOf31, OneOf31Node, AnyOf31, 31, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26, child27 | Child27 | ok27 | Ok27, child28 | Child28 | ok28 | Ok28, child29 | Child29 | ok29 | Ok29, child30 | Child30 | ok30 | Ok30, child31 | Child31 | ok31 | Ok31);
impl_one_of!(OneOf32, OneOf32Node, AnyOf32, 32, child1 | Child1 | ok1 | Ok1, child2 | Child2 | ok2 | Ok2, child3 | Child3 | ok3 | Ok3, child4 | Child4 | ok4 | Ok4, child5 | Child5 | ok5 | Ok5, child6 | Child6 | ok6 | Ok6, child7 | Child7 | ok7 | Ok7, child8 | Child8 | ok8 | Ok8, child9 | Child9 | ok9 | Ok9, child10 | Child10 | ok10 | Ok10, child11 | Child11 | ok11 | Ok11, child12 | Child12 | ok12 | Ok12, child13 | Child13 | ok13 | Ok13, child14 | Child14 | ok14 | Ok14, child15 | Child15 | ok15 | Ok15, child16 | Child16 | ok16 | Ok16, child17 | Child17 | ok17 | Ok17, child18 | Child18 | ok18 | Ok18, child19 | Child19 | ok19 | Ok19, child20 | Child20 | ok20 | Ok20, child21 | Child21 | ok21 | Ok21, child22 | Child22 | ok22 | Ok22, child23 | Child23 | ok23 | Ok23, child24 | Child24 | ok24 | Ok24, child25 | Child25 | ok25 | Ok25, child26 | Child26 | ok26 | Ok26, child27 | Child27 | ok27 | Ok27, child28 | Child28 | ok28 | Ok28, child29 | Child29 | ok29 | Ok29, child30 | Child30 | ok30 | Ok30, child31 | Child31 | ok31 | Ok31, child32 | Child32 | ok32 | Ok32);