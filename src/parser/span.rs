use std::{marker::PhantomData, fmt::{Display, Write}};

use super::{ParsePos, ParseStore, ParseValue};

/// 
/// A span of values in the store.
/// 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn values<'a, Store: ParseStore<Pos, V>, V: ParseValue>(&self, store: &'a Store) -> SpanValueIter<'a, Store, Pos, V> {
        SpanValueIter { span: self.clone(), store, phantom: PhantomData }
    }

    /// 
    /// Returns this span as a string made from its pointed-to characters, assuming that the given store yields `char`s.
    /// 
    pub fn to_string<Store: ParseStore<Pos, char>>(&self, store: &Store) -> String {
        self.values(store).collect()
    }

    /// 
    /// Returns this span as a vector of values.
    /// 
    pub fn to_vec<Store: ParseStore<Pos, V>, V: ParseValue>(&self, store: &Store) -> Vec<V> {
        self.values(store).collect()
    }
}

impl <Pos: ParsePos> Clone for Span<Pos> {
    fn clone(&self) -> Self {
        Self { start: self.start.clone(), end: self.end.clone() }
    }
}

impl <Pos: ParsePos> Display for Span<Pos> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        Display::fmt(&self.start, f)?;
        f.write_str("]-(")?;
        Display::fmt(&self.end, f)?;
        f.write_str(")")
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
        // do not have a value if we are at the end of the span
        if self.span.start.key() == self.span.end.key() {
            return None;
        }

        // only move forward if we currently have a value
        let mut curr_start = self.span.start.clone();
        if let Some(v) = self.store.value_at(&mut curr_start) {
            self.span.start = curr_start;
            Some(v)
        } else {
            None
        }
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


