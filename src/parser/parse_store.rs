use super::{ParsePos, ParseValue};


/// 
/// A struct that stores the data of a parse. Each position in a parse must
/// consult the store to get the value it represents and to advance to the next
/// position (if there is one).
/// 
pub trait ParseStore<Pos: ParsePos, V: ParseValue> {
    /// 
    /// Returns the value at the given position or `None` if the given position
    /// is over the end of the stored data.
    /// 
    /// In addition to returning the next value, it should advance the given
    /// position to the next position.
    /// 
    fn value_at(&self, pos: &mut Pos) -> Option<V>;

    // --- Given Methods ---

    /// 
    /// Peeks at and returns the value of the given position without actually
    /// advancing the position.
    /// 
    fn peek_at(&self, pos: &Pos) -> Option<V> {
        self.value_at(&mut pos.clone())
    }
}

impl <I: Iterator<Item = Item> + ParsePos, Item: ParseValue> ParseStore<I, Item> for () {
    fn value_at(&self, pos: &mut I) -> Option<Item> {
        pos.next()
    }
}

impl ParseStore<usize, char> for &str {
    fn value_at(&self, pos: &mut usize) -> Option<char> {
        let mut chars = self[*pos..].chars();
        let next = chars.next();

        if let Some(_) = next {
            *pos = *pos + (self.as_ptr() as usize).abs_diff(chars.as_str().as_ptr() as usize);
        }

        next
    }
}



