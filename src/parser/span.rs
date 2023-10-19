use std::marker::PhantomData;

use super::{ParsePos, ParseStore, ParseValue};

/// 
/// A span of values in the store.
/// 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span<Pos: ParsePos> {
    /// The start position of the span (inclusive).
    pub start: Pos,
    /// The end position of the span (exclusive).
    pub end: Pos,
}

impl <Pos: ParsePos> Span<Pos> {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }

    pub fn values<Store: ParseStore<Pos, V>, V: ParseValue>(self, store: &Store) -> SpanValueIter<Store, Pos, V> {
        SpanValueIter::new(store, self)
    }
}


// --- Value Iterator ---

/// 
/// An iterator that returns every value of a span.
/// 
#[derive(Clone)]
pub struct SpanValueIter<'a, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    pub span: Span<Pos>,
    pub store: &'a Store,
    phantom: PhantomData<V>,
}

impl <'a, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> SpanValueIter<'a, Store, Pos, V> {
    pub fn new(store: &'a Store, span: Span<Pos>) -> Self {
        Self { span, store, phantom: PhantomData }
    }
}

impl <'a, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Iterator for SpanValueIter<'a, Store, Pos, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.span.start.key() == self.span.end.key() {
            return None;
        }

        self.store.value_at(&mut self.span.start)
    }
}


