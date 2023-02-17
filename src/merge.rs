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

use crate::vertex::Vertex;
use crate::Sodg;
use anyhow::{anyhow, Context, Result};
use log::debug;
use std::collections::{HashMap, HashSet};

impl Sodg {
    /// Merge another graph into the current one.
    ///
    /// During the merge, vertices that are present in both graphs and have
    /// identical coordinates are considered as duplicates and are not re-created
    /// in the current graph. For example, the current graph is "1-a->2", while
    /// the other graph is "1-a->3". The vertex "3" will not be created in the
    /// current graph - it will be considered a duplicate. The resulting graph
    /// will look like "1-a->2".
    pub fn merge(&mut self, g: &Sodg) -> Result<()> {
        let mut ups: HashMap<u32, Vec<(u32, String)>> = HashMap::new();
        for (v, vtx) in g.vertices.iter() {
            if !ups.contains_key(v) {
                ups.insert(*v, vec![]);
            }
            for e in vtx.edges.iter() {
                ups.entry(e.to).or_insert_with(std::vec::Vec::new);
                ups.get_mut(&e.to).unwrap().push((*v, e.a.clone()));
            }
        }
        let mut toxic = HashSet::new();
        loop {
            let mut again = false;
            for (v, vtx) in self.vertices.iter() {
                if let Some(right) = g.vertices.get(v) {
                    for e1 in vtx.edges.iter() {
                        for e2 in right.edges.iter() {
                            if e2.to == e1.to && e2.a != e1.a && !toxic.contains(&e2.to) {
                                toxic.insert(e2.to);
                                again = true;
                            }
                        }
                    }
                    if toxic.contains(v) {
                        for e2 in right.edges.iter() {
                            toxic.insert(e2.to);
                        }
                    }
                }
            }
            if !again {
                break;
            }
        }
        let mut rename: HashMap<u32, u32> = HashMap::new();
        if g.vertices.contains_key(&0) && self.vertices.contains_key(&0) {
            rename.insert(0, 0);
        }
        for (_, vtx) in self.vertices.iter() {
            for e in vtx.edges.iter() {
                if toxic.contains(&e.to) {
                    continue;
                }
                if g.vertices.contains_key(&e.to)
                    && ups.get(&e.to).unwrap().iter().any(|(_, a)| *a == e.a)
                {
                    rename.insert(e.to, e.to);
                }
            }
        }
        for (v, _vtx) in g.vertices.iter() {
            if self.next_v <= *v {
                self.next_v = *v + 1;
            }
        }
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
                            if found.is_none() {
                                found = Some(h.to);
                            } else {
                                found = None;
                            }
                        }
                    }
                }
                if found.is_none() {
                    if self.vertices.contains_key(v) && ups.get(v).unwrap().is_empty() {
                        found = Some(*v);
                    } else {
                        let mut id = *v;
                        if self.vertices.contains_key(&id) {
                            id = self.next_id();
                        }
                        found = Some(id);
                    }
                }
                rename.insert(*v, found.unwrap());
            }
            if rename.len() >= g.vertices.len() {
                break;
            }
        }
        let mut merged = HashSet::new();
        loop {
            let mut again = false;
            for (v, vtx) in g.vertices.iter() {
                let new = rename.get(v).unwrap();
                if !merged.contains(v) {
                    self.add(*new)?;
                    self.put(*new, vtx.data.clone()).context(anyhow!(
                        "Data conflict, ups=[{}], toxic={toxic:?}",
                        ups.get(v)
                            .unwrap()
                            .iter()
                            .map(|(k, v)| format!("{k}->{v}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))?;
                    merged.insert(v);
                }
                for e in vtx.edges.iter() {
                    let v1 = *rename.get(&e.to).unwrap();
                    if !self.vertices.contains_key(&v1) {
                        again = true;
                        continue;
                    }
                    self.bind(*new, v1, e.a.as_str())?;
                }
            }
            if !again {
                break;
            }
        }
        debug!(
            "Merged {} vertices into the existing Sodg",
            g.vertices.len()
        );
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
    g.merge(&extra)?;
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
    g.merge(&extra)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(1, g.kid(0, "foo").unwrap().0);
    Ok(())
}

#[test]
fn merges_singletons() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(13)?;
    let mut extra = Sodg::empty();
    extra.add(13)?;
    g.merge(&extra)?;
    assert_eq!(1, g.vertices.len());
    Ok(())
}

#[test]
fn merges_connected_singletons() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "foo")?;
    g.add(3)?;
    g.bind(3, 2, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.add(2)?;
    extra.bind(1, 2, "foo")?;
    extra.add(3)?;
    extra.bind(3, 2, "foo")?;
    g.merge(&extra)?;
    assert_eq!(3, g.vertices.len());
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
    g.merge(&extra)?;
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
    g.merge(&extra)?;
    assert_eq!(extra.vertices.len(), g.vertices.len());
    Ok(())
}

#[test]
fn merges_large_identical_graphs() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "foo")?;
    g.bind(2, 1, "bar")?;
    g.add(4)?;
    g.bind(4, 1, "x")?;
    g.bind(4, 2, "y")?;
    g.add(5)?;
    g.bind(1, 5, "z")?;
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.add(2)?;
    extra.bind(1, 2, "foo")?;
    extra.bind(2, 1, "bar")?;
    extra.add(4)?;
    extra.bind(4, 1, "x")?;
    extra.bind(4, 2, "y")?;
    extra.add(5)?;
    extra.bind(1, 5, "z")?;
    g.merge(&extra)?;
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
    extra.put(1, Hex::from(42))?;
    g.merge(&extra)?;
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
    g.merge(&extra)?;
    assert_eq!(5, g.vertices.len());
    assert_eq!(1, g.kid(0, "a").unwrap().0);
    assert_eq!(2, g.kid(1, "x").unwrap().0);
    Ok(())
}

#[test]
fn ignores_double_parents() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.add(3)?;
    g.bind(1, 3, "a")?;
    g.bind(2, 3, "a")?;
    let mut extra = Sodg::empty();
    extra.add(42)?;
    extra.add(3)?;
    extra.bind(42, 3, "a")?;
    g.merge(&extra)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}

#[test]
fn merges_into_empty_graph() -> Result<()> {
    let mut g = Sodg::empty();
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.add(2)?;
    extra.add(3)?;
    extra.bind(1, 2, "a")?;
    extra.bind(2, 3, "b")?;
    extra.bind(3, 1, "c")?;
    g.merge(&extra)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(2, g.kid(1, "a").unwrap().0);
    Ok(())
}

#[test]
fn two_roots() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "a")?;
    let mut extra = Sodg::empty();
    extra.add(42)?;
    extra.add(43)?;
    extra.bind(42, 43, "a")?;
    g.merge(&extra)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}

#[test]
fn mixed_injection() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(4)?;
    let mut extra = Sodg::empty();
    extra.add(4)?;
    extra.put(4, Hex::from(4))?;
    extra.add(5)?;
    extra.put(5, Hex::from(5))?;
    extra.bind(5, 4, "b")?;
    g.merge(&extra)?;
    assert_eq!(3, g.vertices.len());
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
    g.merge(&extra)?;
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
    g.merge(&extra)?;
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
    g.merge(&extra)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}
