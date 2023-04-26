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

use crate::{Edges, Vertices};
use crate::{Label, Sodg};
use anyhow::{anyhow, Context, Result};
use log::trace;
use std::collections::HashSet;

impl<const N: usize> Sodg<N> {
    /// Take a slice of the graph, keeping only the vertex specified
    /// by the locator and its kids, recursively found in the entire graph.
    ///
    /// # Errors
    ///
    /// If impossible to slice, an error will be returned.
    pub fn slice(&self, v: u32) -> Result<Self> {
        let g : Sodg<N> = self.slice_some(v, |_, _, _| true)?;
        trace!(
            "#slice: taken {} vertices out of {} at ν{v}",
            g.vertices.len(),
            self.vertices.len()
        );
        Ok(g)
    }

    /// Take a slice of the graph, keeping only the vertex specified
    /// by the locator and its kids, recursively found in the entire graph,
    /// but only if the provided predicate agrees with the selection of
    /// the kids.
    ///
    /// # Errors
    ///
    /// If impossible to slice, an error will be returned.
    pub fn slice_some(&self, v: u32, p: impl Fn(u32, u32, Label) -> bool) -> Result<Self> {
        let mut todo = HashSet::new();
        let mut done = HashSet::new();
        todo.insert(v);
        loop {
            if todo.is_empty() {
                break;
            }
            let before: Vec<u32> = todo.drain().collect();
            for v in before {
                done.insert(v);
                let vtx = self
                    .vertices
                    .get(v)
                    .ok_or_else(|| anyhow!("Can't find ν{v}"))?;
                for e in &vtx.edges {
                    if done.contains(&e.1) {
                        continue;
                    }
                    if !p(v, e.1, e.0) {
                        continue;
                    }
                    done.insert(e.1);
                    todo.insert(e.1);
                }
            }
        }
        let mut new_vertices : Vertices<N> = Vertices::with_capacity(self.vertices.capacity());
        for (v, vtx) in self.vertices.iter().filter(|(v, _)| done.contains(v)) {
            let mut nv = vtx.clone();
            let mut ne = Edges::new();
            for (k, v) in &nv.edges {
                if done.contains(&v) {
                    ne.insert(k, v);
                }
            }
            nv.edges = ne;
            new_vertices.insert(v);
            let vtx = new_vertices.get_mut(v).with_context(|| "Can't find?")?;
            for e in &nv.edges {
                vtx.edges.insert(e.0, e.1);
            }
        }
        let g = Self {
            vertices: new_vertices,
            next_v: self.next_v,
            alerts: self.alerts.clone(),
            alerts_active: self.alerts_active,
            #[cfg(feature = "sober")]
            finds: HashSet::new(),
        };
        trace!(
            "#slice_some: taken {} vertices out of {} at ν{v}",
            g.vertices.len(),
            self.vertices.len()
        );
        Ok(g)
    }
}

#[cfg(test)]
use std::str::FromStr;

#[test]
fn makes_a_slice() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, Label::from_str("foo")?)?;
    g.add(2)?;
    g.bind(0, 2, Label::from_str("bar")?)?;
    assert_eq!(1, g.slice(1)?.vertices.len());
    assert_eq!(1, g.slice(2)?.vertices.len());
    Ok(())
}

#[test]
fn makes_a_partial_slice() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, Label::from_str("foo")?)?;
    g.add(2)?;
    g.bind(1, 2, Label::from_str("bar")?)?;
    let slice = g.slice_some(1, |_v, _to, _a| false)?;
    assert_eq!(1, slice.vertices.len());
    Ok(())
}

#[test]
fn skips_some_vertices() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, Label::from_str("foo")?)?;
    g.add(2)?;
    g.bind(0, 2, Label::from_str("+bar")?)?;
    let slice = g.slice_some(0, |_, _, a| !a.to_string().starts_with('+'))?;
    assert_eq!(2, slice.vertices.len());
    assert_eq!(1, slice.kids(0)?.len());
    Ok(())
}
