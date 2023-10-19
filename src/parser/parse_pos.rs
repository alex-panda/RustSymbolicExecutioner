use std::{fmt::{Debug, Display}, hash::Hash};


///
/// A position in a parse.
/// 
/// A position can be used along with a `ParseStore` to essentially become an
/// iterator as the `value_at` method of a `ParseStore` both returns the value
/// at a position and advances the position.
/// 
/// Positions must be determinative. This means that two clones of the same
/// position must both iterate over the same values and only the same values.
/// One, for instance, cannot return `None` earlier than the other.
/// 
pub trait ParsePos: Debug + Display + Clone {
    type Key: Clone + PartialEq + Hash;

    fn key(&self) -> Self::Key;
}

impl ParsePos for usize {
    type Key = usize;
    fn key(&self) -> Self::Key { *self }
}
