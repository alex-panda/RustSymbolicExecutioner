use core::fmt::Debug;
use std::{collections::{HashMap, BTreeMap}, cell::RefCell};

use crate::parser::{ZSTNode, LRecError, NoAdvanceError};

use super::super::{ParseNode, ParsePos, ParseStore, ParseValue, ParseResult};

#[allow(non_snake_case)]
pub fn LRec<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Debug + Clone, Err: Debug + Clone + From<LRecError> + From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: Child) -> LRecNode<Child, Ok, Err, Store, Pos, V> {
    LRecNode { mem_table: RefCell::new(HashMap::new()), child, _zst: ZSTNode::default() }
}

pub struct LRecNode<Child: ParseNode<Ok, Err, Store, Pos, V>, Ok: Debug + Clone, Err: Debug + Clone + From<LRecError> + From<NoAdvanceError<Pos>>, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub mem_table: RefCell<HashMap<(usize, usize), ParseResult<Ok, Err, Pos>>>,
    pub child: Child,
    pub(super) _zst: ZSTNode<Ok, Err, Store, Pos, V>
}

use ParseResult::*;
impl <Ok: Debug + Clone + 'static, Err: Debug + Clone + From<LRecError> + From<NoAdvanceError<Pos>> + 'static, Store: ParseStore<Pos, V>, Pos: ParsePos + 'static, V: ParseValue, Child: ParseNode<Ok, Err, Store, Pos, V>> ParseNode<Ok, Err, Store, Pos, V> for LRecNode<Child, Ok, Err, Store, Pos, V> {
    fn parse(&self, store: &Store, pos: Pos) -> ParseResult<Ok, Err, Pos> {
        let key = ((store as *const _) as usize, pos.key());

        // check if answer already in mem table
        {
            let table = self.mem_table.borrow();
            let store = table.get(&key);

            if let Some(res) = store {
                return res.clone();
            }
        }

        // Seed the parse by only parsing rules that do not involve this node
        // (i.e. parse such that every recursive call to this node's `parse`
        // function fails). Do this by inserting an error into the memoize
        // table so that this node will return it whenever it is parsed at this
        // position.
        {
            self.mem_table.borrow_mut()
                .insert(key, Error(Err::from(LRecError)));
        }

        let res = self.child.parse(store, pos.clone());

        println!("LRec Seed: {:?}", res);

        // update the memoize table result
        {
            self.mem_table.borrow_mut()
                .insert(key, res.clone());
        }

        // If we still get an error after parsing all child
        // nodes without the recursion of this one, then that means that we only
        // recurse (there are no descendants that don't recurse without making
        // progess to a new position) so we can just return the error.
        let (mut curr_ok, mut curr_pos) = match res {
            Okay(ok) => (ok, pos.clone()),
            OkayAdvance(ok, advance) => (ok, advance),
            Error(error) => return Error(error),
            Panic(error) => return Panic(error),
        };

        loop {
            match self.child.parse(store, pos.clone()) {
                Okay(_) => {
                    // no advance
                    self.mem_table.borrow_mut()
                        .insert(key, Okay(curr_ok.clone()));
                    return Okay(curr_ok);
                },
                OkayAdvance(ok, adv) => {
                    if adv.key() <= curr_pos.key() {
                        // no advance
                        self.mem_table.borrow_mut()
                            .insert(key, OkayAdvance(curr_ok.clone(), curr_pos.clone()));
                        return OkayAdvance(curr_ok, curr_pos);
                    }

                    curr_ok = ok;
                    curr_pos = adv;
                    self.mem_table.borrow_mut().insert(key, OkayAdvance(curr_ok.clone(), curr_pos.clone()));
                },
                Error(_) => {
                    // return last successful result
                    self.mem_table.borrow_mut().insert(key, OkayAdvance(curr_ok.clone(), curr_pos.clone()));
                    return OkayAdvance(curr_ok, curr_pos);
                },
                Panic(err) => {
                    // memoize last successful result
                    self.mem_table.borrow_mut().insert(key, OkayAdvance(curr_ok.clone(), curr_pos.clone()));
                    // return Panic
                    return Panic(err);
                },
            }
        }
    }
}
