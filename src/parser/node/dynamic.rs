use crate::parser::{ParseNode, ParseStore, ParsePos, ParseValue};


/// 
/// Casts the given reference to a `ParseNode` to a reference to a `dyn ParseNode` node. 
/// 
#[allow(non_snake_case)]
#[inline]
pub fn Dyn<'a, Child: ParseNode<Ok, Err, Store, Pos, V>, Ok, Err, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue>(child: &'a Child) -> &'a dyn ParseNode<Ok, Err, Store, Pos, V> {
    child
}