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

use crate::{Roll, Vertex, Vertices, VerticesIter};
use std::fmt;
use std::fmt::{Debug, Formatter};

impl<const M : usize, const N : usize> Debug for Vertices<M, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (i, v) in &self.map {
            let mut attrs = v
                .edges
                .into_iter()
                .map(|e| format!("\n\t{} ➞ ν{}", e.0, e.1))
                .collect::<Vec<String>>();
            if let Some(d) = &v.data {
                attrs.push(format!("{d}"));
            }
            lines.push(format!("ν{i} -> ⟦{}⟧", attrs.join(", ")));
        }
        f.write_str(lines.join("\n").as_str())
    }
}

impl<'a, const M : usize, const N : usize> Iterator for VerticesIter<'a, M, N> {
    type Item = (&'a u32, &'a Vertex<N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<const M : usize, const N : usize> Vertices<M, N> {
    pub fn new() -> Self {
        Self {
            map: Roll::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn insert(&mut self, v: u32) {
        self.map.insert(v, Vertex::empty());
    }

    pub fn get(&self, v: u32) -> Option<&Vertex<N>> {
        self.map.get(v)
    }

    pub fn get_mut(&mut self, v: u32) -> Option<&mut Vertex<N>> {
        self.map.get_mut(v)
    }

    pub fn try_id(&self, id: u32) -> u32 {
        if !self.contains(id) {
            return id;
        }
        let mut next = None;
        for (v, _) in self.iter() {
            if let Some(before) = next {
                if *v > before {
                    next = Some(*v);
                }
            }
            if next.is_none() {
                next = Some(*v);
            }
        }
        next.map_or(0, |x| x + 1)
    }

    pub fn remove(&mut self, v: u32) {
        self.map.remove(v);
    }

    pub fn contains(&self, v: u32) -> bool {
        self.map.contains_key(v)
    }

    pub fn iter(&self) -> VerticesIter<M, N> {
        VerticesIter {
            iter: self.map.iter(),
        }
    }
}

#[test]
fn iterates_empty() {
    let vcs : Vertices<4, 4> = Vertices::new();
    assert!(vcs.iter().next().is_none());
}

#[test]
fn inserts_and_lists() {
    let mut vcs : Vertices<4, 4> = Vertices::new();
    vcs.insert(1);
    assert_eq!(1, *vcs.iter().next().unwrap().0);
}

#[test]
fn inserts_and_iterates() {
    let mut vcs : Vertices<4, 4> = Vertices::new();
    vcs.insert(42);
    vcs.insert(16);
    let mut keys = vec![];
    for (v, _) in vcs.iter() {
        keys.push(*v);
    }
    assert_eq!(2, keys.len());
}

#[test]
fn inserts_and_gets() {
    let mut vcs : Vertices<4, 4> = Vertices::new();
    vcs.insert(42);
    assert!(vcs.get(42).unwrap().edges.into_iter().next().is_none());
}

#[test]
fn inserts_and_deletes() {
    let mut vcs : Vertices<4, 4> = Vertices::new();
    vcs.insert(42);
    vcs.remove(42);
    assert_eq!(0, vcs.len());
}
