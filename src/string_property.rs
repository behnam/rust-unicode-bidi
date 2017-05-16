// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::ops::{Index, IndexMut, Range};


/// Store a property `T` for each character in the `text`, indexed by the byte index of the
/// character in `text`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringProperty<'text, T> {
    text: &'text str,
    pub values: Vec<T>,
}

impl<'text, T: Copy> StringProperty<'text, T> {
    pub fn new(text: &'text str, default_value: T) -> Self {
        let mut values = Vec::<T>::with_capacity(text.len());
        values.resize(text.bytes().len(), default_value);
        StringProperty { text, values }
    }

    pub fn from_values(text: &'text str, values: Vec<T>) -> Self {
        // TODO debug_assert values match per char
        StringProperty { text, values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    // TODO either fetch ch or debug_assert ch == text[idx]
    pub fn set(&mut self, idx: usize, ch: char, value: T) {
        for v in self.values.iter_mut().take(idx + ch.len_utf8()).skip(idx) {
            *v = value;
        }
    }

    pub fn get(&self, idx: usize) -> T {
        // TODO debug_assert idx is start of utf8 byte seq
        // TODO debug_assert value is the same for the whole utf8 byte seq
        self.values[idx]
    }

    pub fn get_range(&self, range: Range<usize>) -> &[T] {
        &self.values[range]
    }

    pub fn get_all(&self) -> &[T] {
        &self.values[..]
    }
}


impl<'text, T: Copy> Index<usize> for StringProperty<'text, T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.values[idx]
    }
}

impl<'text, T: Copy> IndexMut<usize> for StringProperty<'text, T> {
    // TODO either debug_assert, replace with set(), or call set()
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut T {
        &mut self.values[idx]
    }
}
