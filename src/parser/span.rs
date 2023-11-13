use std::{marker::PhantomData, fmt::{Display, Write}};

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
        SpanValueIter { span: self.clone(), store, phantom: PhantomData }
    }
}

impl <Pos: ParsePos> Display for Span<Pos> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.start, f)?;
        f.write_char('-')?;
        Display::fmt(&self.end, f)
    }
}

// --- Value Iterator ---

/// 
/// An iterator that returns every value of a span.
/// 
pub struct SpanValueIter<'a, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> {
    span: Span<Pos>,
    store: &'a Store,
    phantom: PhantomData<V>,
}

impl <'a, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Clone for SpanValueIter<'a, Store, Pos, V> {
    fn clone(&self) -> Self {
        Self { span: self.span.clone(), store: self.store.clone(), phantom: self.phantom.clone() }
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

impl <'a, Store: ParseStore<Pos, V>, Pos: ParsePos, V: ParseValue> Display for SpanValueIter<'a, Store, Pos, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.clone() {
            Display::fmt(&value, f)?;
        }

        Ok(())
    }
}


