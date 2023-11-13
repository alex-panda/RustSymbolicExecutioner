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
        let next = self[*pos..].chars().next();

        if let Some(c) = next {
            let mut buffer = [0; 4];
            // advance the position by however many bytes it takes to hold the character
            *pos += c.encode_utf8(&mut buffer).len();
        }

        next
    }
}



