// Copyright (c) 2022-2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{Edges, EdgesIter, Label};
use rustc_hash::FxHashMap;
use std::collections::hash_map::IterMut;

impl<'a> Iterator for EdgesIter<'a> {
    type Item = (&'a Label, &'a u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Edges {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }

    pub fn iter(&self) -> EdgesIter {
        EdgesIter {
            iter: self.map.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<Label, u32> {
        self.map.iter_mut()
    }

    pub fn insert(&mut self, a: Label, v: u32) {
        self.map.insert(a, v);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Label, &mut u32) -> bool,
    {
        self.map.retain(f);
    }
}

#[test]
fn panic_on_complex_alert() {
    // assert!(g.add(42).is_err());
}
