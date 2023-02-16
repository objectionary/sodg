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

use crate::Sodg;
use log::debug;
use std::collections::HashMap;

impl Sodg {
    /// Merge another graph into itself.
    pub fn merge(&mut self, g: &Sodg) {
        let mut ups: HashMap<u32, Vec<(u32, String)>> = HashMap::new();
        for (v, vtx) in g.vertices.iter() {
            if !ups.contains_key(v) {
                ups.insert(*v, vec![]);
            }
            for e in vtx.edges.iter() {
                if !ups.contains_key(&e.to) {
                    ups.insert(e.to, vec![]);
                }
                ups.get_mut(&e.to).unwrap().push((*v, e.a.clone()));
            }
        }
        let mut rename: HashMap<u32, u32> = HashMap::new();
        rename.insert(0, 0);
        loop {
            for (v, _vtx) in g.vertices.iter() {
                if rename.contains_key(v) {
                    continue;
                }
                let mut found = None;
                for up in ups.get(v).unwrap() {
                    let left = rename.get(&up.0);
                    if left.is_none() {
                        continue;
                    }
                    let has = match self.vertices.get(left.unwrap()) {
                        None => Vertex::empty(),
                        Some(vt) => vt.clone(),
                    };
                    for h in has.edges {
                        if h.a == up.1 {
                            if found.is_some() && found.unwrap() != h.to {
                                panic!("Double parents");
                            }
                            found = Some(h.to);
                        }
                    }
                }
                if found.is_none() {
                    found = Some(self.next_id());
                }
                rename.insert(*v, found.unwrap());
            }
            if rename.len() >= g.vertices.len() {
                break;
            }
        }
        for (v, vtx) in g.vertices.iter() {
            let new = rename.get(v).unwrap();
            let mut before = match self.vertices.get(new) {
                None => Vertex::empty(),
                Some(b) => b.clone(),
            };
            for e in vtx.edges.iter() {
                let v1 = *rename.get(&e.to).unwrap();
                before.edges.retain(|e1| &e1.a != e.a.as_str());
                before.edges.push(Edge::new(v1, e.a.as_str()));
            }
            self.vertices.insert(*new, before);
        }
        debug!(
            "Merged {} vertices into the existing Sodg",
            g.vertices.len()
        );
    }
}

use crate::edge::Edge;
use crate::vertex::Vertex;
#[cfg(test)]
use anyhow::Result;

#[test]
fn merges_two_graphs() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "bar")?;
    g.merge(&extra);
    assert_eq!(3, g.vertices.len());
    Ok(())
}

#[test]
fn avoids_simple_duplicates() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "foo")?;
    extra.add(2)?;
    extra.bind(1, 2, "bar")?;
    g.merge(&extra);
    debug!("{g:?}");
    assert_eq!(3, g.vertices.len());
    assert_eq!(1, g.kid(0, "foo").unwrap().0);
    Ok(())
}
