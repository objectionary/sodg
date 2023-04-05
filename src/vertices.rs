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

use crate::{Vertex, Vertices};
use rustc_hash::FxHashMap;
use std::collections::hash_map::{Iter, IterMut, Keys};
use std::fmt;
use std::fmt::{Debug, Formatter};

impl Debug for Vertices {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (i, v) in &self.map {
            let mut attrs = v
                .edges
                .iter()
                .map(|e| format!("\n\t{} ➞ ν{}", e.0, e.1))
                .collect::<Vec<String>>();
            if !&v.data.is_empty() {
                attrs.push(format!("{}", v.data));
            }
            lines.push(format!("ν{i} -> ⟦{}⟧", attrs.join(", ")));
        }
        f.write_str(lines.join("\n").as_str())
    }
}

impl Vertices {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }

    pub fn keys(&self) -> Keys<u32, Vertex> {
        self.map.keys()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn insert(&mut self, v: u32) {
        self.map.insert(v, Vertex::empty());
    }

    pub fn get(&self, v: u32) -> Option<&Vertex> {
        self.map.get(&v)
    }

    pub fn get_mut(&mut self, v: u32) -> Option<&mut Vertex> {
        self.map.get_mut(&v)
    }

    pub fn remove(&mut self, v: u32) {
        self.map.remove(&v);
    }

    pub fn contains(&self, v: u32) -> bool {
        self.map.contains_key(&v)
    }

    pub fn iter(&self) -> Iter<u32, Vertex> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<u32, Vertex> {
        self.map.iter_mut()
    }
}

#[test]
fn panic_on_complex_alert() {
    // assert!(g.add(42).is_err());
}
