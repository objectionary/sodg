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
use anyhow::{anyhow, Result};
use log::debug;
use std::collections::{HashMap, HashSet};

impl Sodg {
    /// Merge another graph into the current one.
    ///
    /// It is expected that both graphs are trees! If they are not, the result is unpredictable.
    ///
    /// The `right` vertex is mapped to the `left` vertex. The decisions about
    /// their kids are made recursively.
    ///
    /// The `left` vertex is expected
    /// to be the root of the current graph, while the `right` vertex is the root
    /// of the graph being merged into the current one.
    ///
    /// # Errors
    ///
    /// If it's impossible to merge, an error will be returned.
    pub fn merge(&mut self, g: &Self, left: u32, right: u32) -> Result<()> {
        let mut mapped = HashMap::new();
        let before = self.vertices.len();
        self.merge_rec(g, left, right, &mut mapped)?;
        let merged = mapped.len();
        let scope = g.vertices.len();
        if merged != scope {
            let must: Vec<u32> = g.vertices.keys().copied().collect();
            let seen: Vec<u32> = mapped.keys().copied().collect();
            let missed: HashSet<u32> = &HashSet::from_iter(must) - &HashSet::from_iter(seen);
            let mut ordered: Vec<u32> = missed.into_iter().collect();
            ordered.sort_unstable();
            return Err(anyhow!(
                "Just {merged} vertices merged, out of {scope}; maybe the right graph was not a tree? {} missed: {}",
                ordered.len(), ordered.iter().map(|v| format!("ν{v}")).collect::<Vec<String>>().join(", ")
            ));
        }
        debug!(
            "Merged all {merged} vertices into SODG of {}, making it have {} after the merge",
            before,
            self.vertices.len()
        );
        Ok(())
    }

    /// Merge two trees recursively, ignoring the nodes already `mapped`.
    ///
    /// The `right` vertex is mapped to the `left` vertex. The decisions about
    /// their kids are made recursively.
    ///
    /// The `mapped` is a key-value map, where the key is a vertex from the right
    /// graph, which is mapped to a vertex from the left graph.
    ///
    /// # Errors
    ///
    /// If it's impossible to merge, an error will be returned.
    fn merge_rec(
        &mut self,
        g: &Self,
        left: u32,
        right: u32,
        mapped: &mut HashMap<u32, u32>,
    ) -> Result<()> {
        if mapped.contains_key(&right) {
            return Ok(());
        }
        mapped.insert(right, left);
        let d = g
            .vertices
            .get(right)
            .ok_or_else(|| anyhow!("Can't find ν{right} in the right graph"))?
            .data
            .clone();
        if d.is_some() {
            self.put(left, &d.unwrap())?;
        }
        for (a, to) in g.kids(right)? {
            let matched = if let Some(t) = self.kid(left, &a) {
                t
            } else if let Some(t) = mapped.get(&to) {
                self.bind(left, *t, &a)?;
                *t
            } else {
                let id = self.next_id();
                self.add(id)?;
                self.bind(left, id, &a)?;
                id
            };
            self.merge_rec(g, matched, to, mapped)?;
        }
        for (a, to) in g.kids(right)? {
            if let Some(first) = self.kid(left, &a) {
                if let Some(second) = mapped.get(&to) {
                    if first != *second {
                        self.join(first, *second)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn join(&mut self, left: u32, right: u32) -> Result<()> {
        for vtx in self.vertices.iter_mut() {
            for e in &mut vtx.1.edges {
                if *e.1 == right {
                    *e.1 = left;
                }
            }
        }
        for e in self.kids(right)? {
            if self.kid(left, &e.0).is_some() {
                return Err(anyhow!(
                    "Can't merge ν{right} into ν{left}, due to conflict in '{}'",
                    e.0
                ));
            }
            self.bind(left, e.1, &e.0)?;
        }
        self.vertices.remove(right);
        Ok(())
    }
}

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
    g.merge(&extra, 0, 0)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(1, g.kid(0, "foo").unwrap());
    assert_eq!(2, g.kid(0, "bar").unwrap());
    Ok(())
}

#[test]
fn merges_two_non_trees() -> Result<()> {
    let mut g = Sodg::empty();
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(42)?;
    extra.add(2)?;
    extra.add(13)?;
    let r = g.merge(&extra, 0, 0);
    assert!(r.is_err());
    let msg = r.err().unwrap().to_string();
    assert!(msg.contains("ν2, ν13, ν42"), "{}", msg);
    Ok(())
}

#[test]
fn merges_a_loop() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.add(2)?;
    g.bind(1, 2, "b")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(4)?;
    extra.bind(0, 4, "c")?;
    extra.add(3)?;
    extra.bind(0, 3, "a")?;
    extra.bind(4, 3, "d")?;
    extra.add(5)?;
    extra.bind(3, 5, "e")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(5, g.vertices.len());
    assert_eq!(1, g.kid(0, "a").unwrap());
    assert_eq!(2, g.kid(1, "b").unwrap());
    // assert_eq!(3, g.kid(0, "c").unwrap());
    // assert_eq!(1, g.kid(3, "d").unwrap());
    // assert_eq!(5, g.kid(1, "e").unwrap());
    Ok(())
}

#[test]
fn avoids_simple_duplicates() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(5)?;
    g.bind(0, 5, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "foo")?;
    extra.add(2)?;
    extra.bind(1, 2, "bar")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(5, g.kid(0, "foo").unwrap());
    assert_eq!(6, g.kid(5, "bar").unwrap());
    Ok(())
}

#[test]
fn keeps_existing_vertices_intact() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    g.add(2)?;
    g.bind(1, 2, "bar")?;
    g.add(3)?;
    g.bind(2, 3, "zzz")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(5)?;
    extra.bind(0, 5, "foo")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(4, g.vertices.len());
    assert_eq!(1, g.kid(0, "foo").unwrap());
    assert_eq!(2, g.kid(1, "bar").unwrap());
    assert_eq!(3, g.kid(2, "zzz").unwrap());
    Ok(())
}

#[test]
fn merges_singletons() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(13)?;
    let mut extra = Sodg::empty();
    extra.add(13)?;
    g.merge(&extra, 13, 13)?;
    assert_eq!(1, g.vertices.len());
    Ok(())
}

#[test]
fn merges_simple_loop() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "foo")?;
    g.bind(2, 1, "bar")?;
    let extra = g.clone();
    g.merge(&extra, 1, 1)?;
    assert_eq!(extra.vertices.len(), g.vertices.len());
    Ok(())
}

#[test]
fn merges_large_loop() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.add(3)?;
    g.add(4)?;
    g.bind(1, 2, "a")?;
    g.bind(2, 3, "b")?;
    g.bind(3, 4, "c")?;
    g.bind(4, 1, "d")?;
    let extra = g.clone();
    g.merge(&extra, 1, 1)?;
    assert_eq!(extra.vertices.len(), g.vertices.len());
    Ok(())
}

#[cfg(test)]
use crate::Hex;

#[test]
fn merges_data() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.put(1, &Hex::from(42))?;
    g.merge(&extra, 1, 1)?;
    assert_eq!(42, g.data(1)?.to_i64()?);
    Ok(())
}

#[test]
fn understands_same_name_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.add(2)?;
    g.bind(1, 2, "x")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "b")?;
    extra.add(2)?;
    extra.bind(1, 2, "x")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(5, g.vertices.len());
    assert_eq!(1, g.kid(0, "a").unwrap());
    assert_eq!(2, g.kid(1, "x").unwrap());
    Ok(())
}

#[test]
fn merges_into_empty_graph() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.add(2)?;
    extra.add(3)?;
    extra.bind(1, 2, "a")?;
    extra.bind(2, 3, "b")?;
    extra.bind(3, 1, "c")?;
    g.merge(&extra, 1, 1)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(0, g.kid(1, "a").unwrap());
    Ok(())
}

#[test]
fn mixed_injection() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(4)?;
    let mut extra = Sodg::empty();
    extra.add(4)?;
    extra.put(4, &Hex::from(4))?;
    extra.add(5)?;
    extra.put(5, &Hex::from(5))?;
    extra.bind(4, 5, "b")?;
    g.merge(&extra, 4, 4)?;
    assert_eq!(2, g.vertices.len());
    Ok(())
}

#[test]
fn zero_to_zero() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.bind(1, 0, "back")?;
    g.add(2)?;
    g.bind(0, 2, "b")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "c")?;
    extra.bind(1, 0, "back")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}

#[test]
fn finds_siblings() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.add(2)?;
    g.bind(0, 2, "b")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "b")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(3, g.vertices.len());
    Ok(())
}

#[cfg(test)]
use crate::Script;

#[test]
fn two_big_graphs() -> Result<()> {
    let mut g = Sodg::empty();
    Script::from_str(
        "ADD(0); ADD(1); BIND(0, 1, foo);
        ADD(2); BIND(0, 1, alpha);
        BIND(1, 0, back);",
    )
    .deploy_to(&mut g)?;
    let mut extra = Sodg::empty();
    Script::from_str("ADD(0); ADD(1); BIND(0, 1, bar); BIND(1, 0, back);").deploy_to(&mut extra)?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}
