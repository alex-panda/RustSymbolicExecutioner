use zst::ZST;

use super::{ParseStore, ParsePos, ParseValue, ParseResult, UnexpectedEndError, Span};


/// 
/// A struct to encapsulate the current position of a parse.
/// 
pub struct ParseContext<'a, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> {
    /// A reference to the data being parsed.
    pub store: &'a Store,
    /// The current position of the store the parse is at.
    pub pos: Pos,
    /// The trace level.
    pub trace: i32,
    zst: ZST<V>,
}

impl <'a, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> ParseContext<'a, Store, Pos, V> {
    #[inline]
    pub(crate) fn new(store: &'a Store, pos: Pos, trace: i32) -> Self {
        Self { store, pos, zst: ZST::default(), trace }
    }

    #[inline]
    pub fn with_pos(&self, pos: Pos) -> Self {
        Self::new(self.store, pos, self.trace)
    }

    #[inline]
    pub fn with_store(&self, store: &'a Store) -> Self {
        Self::new(store, self.pos.clone(), self.trace)
    }

    #[inline]
    pub fn with_trace(&self, trace: i32) -> Self {
        Self::new(self.store, self.pos.clone(), self.trace)
    }

    /// 
    /// Returns the next value of the parse, advancing the parse to the next
    /// position.
    /// 
    #[inline]
    pub fn next(&mut self) -> Option<V> {
        self.store.value_at(&mut self.pos)
    }

    /// 
    /// Peeks at the next value of the parse WITHOUT advancing the parse to the
    /// next position.
    /// 
    #[inline]
    pub fn peek(&self) -> Option<V> {
        self.store.peek_at(&self.pos)
    }

    pub fn map_value<Ok, Err: From<UnexpectedEndError<Pos>>, F: FnOnce(Span<Pos>, V) -> ParseResult<Ok, Err, Pos>>(&self, f: F) -> ParseResult<Ok, Err, Pos> {
        let mut curr_pos = self.pos.clone();
        let v = self.store.value_at(&mut curr_pos);
        match v {
            Some(v) => f(Span::new(self.pos.clone(), curr_pos), v),
            None => ParseResult::Error(UnexpectedEndError { pos: self.pos.clone() }.into())
        }
    }
}

impl <'a, Store: ParseStore<Pos, V> + ?Sized, Pos: ParsePos, V: ParseValue> Clone for ParseContext<'a, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { store: self.store, pos: self.pos.clone(), zst: self.zst.clone(), trace: self.trace.clone() }
    }
}
