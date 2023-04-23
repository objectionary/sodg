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
    pub fn new() -> Self {
        Self { bar: Vec::with_capacity(100) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[inline]
    pub fn insert(&mut self, v: u32) {
        self.bar.insert(v as usize, Some(Vertex::empty()));
    }

    #[inline]
    pub fn get(&self, v: u32) -> Option<&Vertex<N>> {
        if let Some(v) = self.bar.get(v as usize) {
            Some(v.as_ref().unwrap())
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, v: u32) -> Option<&mut Vertex<N>> {
        if let Some(v) = self.bar.get_mut(v as usize) {
            Some(v.as_mut().unwrap())
        } else {
            None
        }
    }

    pub fn try_id(&self, id: u32) -> u32 {
        if !self.contains(id) {
            return id;
        }
        let mut next = None;
        for (v, _) in self.iter() {
            if let Some(before) = next {
                if v > before {
                    next = Some(v);
                }
            }
            if next.is_none() {
                next = Some(v);
            }
        }
        next.map_or(0, |x| x + 1)
    }

    pub fn remove(&mut self, v: u32) {
        self.bar.remove(v as usize);
        let all: Vec<u32> = self.iter().map(|(k, _v)| k as u32).collect();
        for v1 in all {
            if let Some(vtx) = self.bar.get_mut(v1 as usize).unwrap() {
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
    pub fn contains(&self, v: u32) -> bool {
        self.bar.get(v as usize).is_some()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=(u32, &Vertex<N>)> {
        self.bar.iter().enumerate().filter(|(_i, v)| v.is_some()).map(|(i, v)| (i as u32, v.as_ref().unwrap()))
    }
}

#[test]
fn iterates_empty() {
    let vcs: Vertices<4> = Vertices::new();
    assert!(vcs.iter().next().is_none());
}

#[test]
fn inserts_and_lists() {
    let mut vcs: Vertices<4> = Vertices::new();
    vcs.insert(0);
    assert_eq!(0, vcs.iter().next().unwrap().0);
}

#[test]
fn inserts_and_iterates() {
    let mut vcs: Vertices<4> = Vertices::new();
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
    let mut vcs: Vertices<4> = Vertices::new();
    vcs.insert(0);
    assert!(vcs.get(0).unwrap().edges.into_iter().next().is_none());
}

#[test]
fn inserts_and_deletes() {
    let mut vcs: Vertices<4> = Vertices::new();
    vcs.insert(0);
    vcs.remove(0);
    assert_eq!(0, vcs.len());
}
