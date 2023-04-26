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
use std::fmt;
use std::fmt::{Debug, Formatter};

impl<const N: usize> Debug for Vertices<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (i, v) in self.iter() {
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

impl<const N: usize> Vertices<N> {
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            emap: emap::Map::with_capacity_init(cap),
        }
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.emap.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.emap.len()
    }

    #[inline]
    pub fn insert(&mut self, v: usize) {
        self.emap.insert(v, Vertex::empty());
    }

    #[inline]
    pub fn get(&self, v: usize) -> Option<&Vertex<N>> {
        self.emap.get(v)
    }

    #[inline]
    pub fn get_mut(&mut self, v: usize) -> Option<&mut Vertex<N>> {
        self.emap.get_mut(v)
    }

    #[inline]
    pub fn try_id(&self, id: usize) -> usize {
        if !self.contains(id) {
            return id;
        }
        self.emap.next_key_gte(id)
    }

    pub fn remove(&mut self, v: usize) {
        self.emap.remove(v);
        let all: Vec<usize> = self.iter().map(|(k, _v)| k).collect();
        for v1 in all {
            if let Some(vtx) = self.emap.get_mut(v1) {
                if let Some(a) = vtx
                    .edges
                    .into_iter()
                    .filter(|(_, t)| *t == v)
                    .map(|(a, _)| a)
                    .next()
                {
                    vtx.edges.remove(a);
                }
            }
        }
    }

    #[inline]
    pub fn contains(&self, v: usize) -> bool {
        self.emap.contains_key(v)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Vertex<N>)> {
        self.emap.iter()
    }
}

#[test]
fn iterates_empty() {
    let vcs: Vertices<16> = Vertices::with_capacity(256);
    assert!(vcs.iter().next().is_none());
}

#[test]
fn inserts_and_lists() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(0);
    assert_eq!(0, vcs.iter().next().unwrap().0);
}

#[test]
fn inserts_and_checks() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(0);
    assert!(vcs.contains(0));
    vcs.insert(8);
    assert!(vcs.contains(8));
    assert!(!vcs.contains(5));
}

#[test]
fn evaluates_length() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(1);
    vcs.insert(5);
    assert_eq!(2, vcs.len());
}

#[test]
fn inserts_and_iterates() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(0);
    vcs.insert(1);
    let mut keys = vec![];
    for (v, _) in vcs.iter() {
        keys.push(v);
    }
    assert_eq!(2, keys.len());
}

#[test]
fn inserts_and_gets() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(0);
    assert!(vcs.get(0).unwrap().edges.into_iter().next().is_none());
}

#[test]
fn inserts_and_deletes() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(0);
    vcs.remove(0);
    assert_eq!(0, vcs.len());
}

#[test]
fn try_id_many_times() {
    let vcs: Vertices<16> = Vertices::with_capacity(256);
    assert_eq!(0, vcs.try_id(0));
    assert_eq!(1, vcs.try_id(1));
}

#[test]
fn can_be_cloned() {
    let mut vcs: Vertices<16> = Vertices::with_capacity(256);
    vcs.insert(7);
    assert_eq!(1, vcs.clone().len());
}
