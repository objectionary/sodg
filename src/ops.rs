// Copyright (c) 2022 Yegor Bugayenko
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

use crate::edge::Edge;
use crate::Sot;
use crate::Vertex;
use anyhow::{anyhow, Context, Result};
use log::trace;
use std::collections::VecDeque;
use std::str::FromStr;

impl Sot {
    /// Add a new vertex `v1` to the Sot:
    ///
    /// ```
    /// use sot::Sot;
    /// let mut sot = Sot::empty();
    /// sot.add(0).unwrap();
    /// sot.add(42).unwrap();
    /// sot.bind(0, 42, "hello").unwrap();
    /// ```
    pub fn add(&mut self, v1: u32) -> Result<()> {
        if self.vertices.contains_key(&v1) {
            return Err(anyhow!("Vertex ν{} already exists", v1));
        }
        self.vertices.insert(v1, Vertex::empty());
        trace!("#add(ν{}): new vertex added", v1);
        Ok(())
    }

    /// Makes an edge `e1` from vertex `v1` to vertex `v2` and puts `a` label on it. If the
    /// label is not equal to `"ρ"`, makes two backward edges from `v2` to `v1`
    /// and label them as `"ρ"` an `"𝜎"`.
    pub fn bind(&mut self, v1: u32, v2: u32, a: &str) -> Result<()> {
        if a.is_empty() {
            return Err(anyhow!(
                "Edge label can't be empty, from ν{} to ν{}",
                v1,
                v2
            ));
        }
        if !self.vertices.contains_key(&v1) {
            return Err(anyhow!("Can't find ν{}", v1));
        }
        if !self.vertices.contains_key(&v2) {
            return Err(anyhow!("Can't find ν{}", v2));
        }
        let vtx1 = self
            .vertices
            .get_mut(&v1)
            .context(format!("Can't find ν{}", v1))?;
        vtx1.edges.retain(|e| e.a != a);
        vtx1.edges.push(Edge::new(v2, a));
        trace!("#bind: edge added ν{}-{}->ν{}", v1, a, v2);
        Ok(())
    }

    /// Set vertex data.
    pub fn put(&mut self, v: u32, d: Vec<u8>) -> Result<()> {
        let vtx = self
            .vertices
            .get_mut(&v)
            .context(format!("Can't find ν{}", v))?;
        vtx.data = d.clone();
        trace!("#data: data of ν{} set to {}b", v, d.len());
        Ok(())
    }

    /// Read vertex data.
    pub fn data(&self, v: u32) -> Result<Vec<u8>> {
        let vtx = self
            .vertices
            .get(&v)
            .context(format!("Can't find ν{}", v))?;
        Ok(vtx.data.clone())
    }

    /// Find all kids of a vertex.
    pub fn kids(&self, _v: u32) -> Result<Vec<(String, u32)>> {
        Err(anyhow!("Not implemented yet"))
    }

    /// Find kid.
    pub fn kid(&self, v: u32, a: &str) -> Option<u32> {
        if let Some(e) = self
            .vertices
            .get(&v)
            .unwrap()
            .edges
            .iter()
            .find(|e| e.a == a)
        {
            Some(e.to)
        } else {
            None
        }
    }

    /// Find a vertex in the Sot by its locator.
    pub fn find(&self, v1: u32, loc: &str) -> Result<u32> {
        let mut v = v1;
        let mut locator: VecDeque<String> = VecDeque::new();
        loc.split('.')
            .filter(|k| !k.is_empty())
            .for_each(|k| locator.push_back(k.to_string()));
        loop {
            let next = locator.pop_front();
            if next.is_none() {
                trace!("#find: end of locator, we are at ν{}", v);
                break;
            }
            let k = next.unwrap().to_string();
            if k.is_empty() {
                return Err(anyhow!("System error, the locator is empty"));
            }
            if k.starts_with("ν") {
                let num: String = k.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
                v = u32::from_str(num.as_str())?;
                trace!("#find: jumping directly to ν{}", v);
                continue;
            }
            if let Some(to) = self.kid(v, k.as_str()) {
                trace!("#find: ν{}.{} -> ν{}", v, k, to);
                v = to;
                continue;
            };
            let others: Vec<String> = self
                .vertices
                .get(&v)
                .unwrap()
                .edges
                .iter()
                .map(|e| e.a.clone())
                .collect();
            return Err(anyhow!(
                "Can't find .{} in ν{} among other {} attribute{}: {}",
                k,
                v,
                others.len(),
                if others.len() == 1 { "" } else { "s" },
                others.join(", ")
            ));
        }
        trace!("#find: found ν{} by '{}'", v1, loc);
        Ok(v)
    }
}

#[test]
fn adds_simple_vertex() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(1)?;
    assert!(sot.inconsistencies().is_empty());
    assert_eq!(1, sot.find(1, "")?);
    Ok(())
}

#[test]
fn binds_simple_vertices() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(1)?;
    sot.add(2)?;
    let k = "hello";
    sot.bind(1, 2, k)?;
    assert!(sot.inconsistencies().is_empty());
    assert_eq!(2, sot.find(1, k)?);
    Ok(())
}

#[test]
fn pre_defined_ids() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(1)?;
    sot.add(2)?;
    let k = "a-привет";
    sot.bind(1, 2, k)?;
    assert!(sot.inconsistencies().is_empty());
    assert_eq!(2, sot.find(1, k)?);
    Ok(())
}

#[test]
fn binds_two_names() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(1)?;
    sot.add(2)?;
    sot.bind(1, 2, "first")?;
    sot.bind(1, 2, "second")?;
    assert!(sot.inconsistencies().is_empty());
    assert_eq!(2, sot.find(1, "first")?);
    Ok(())
}

#[test]
fn overwrites_edge() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(1)?;
    sot.add(2)?;
    let label = "hello";
    sot.bind(1, 2, label)?;
    sot.add(3)?;
    sot.bind(1, 3, label)?;
    assert!(sot.inconsistencies().is_empty());
    assert_eq!(3, sot.find(1, label)?);
    Ok(())
}

#[test]
fn binds_to_root() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    sot.add(1)?;
    sot.bind(0, 1, "x")?;
    assert!(sot.inconsistencies().is_empty());
    assert!(sot.kid(0, "ρ").is_none());
    assert!(sot.kid(0, "σ").is_none());
    Ok(())
}

#[test]
fn sets_simple_data() -> Result<()> {
    let mut sot = Sot::empty();
    let data = "hello".as_bytes().to_vec();
    sot.add(0)?;
    sot.put(0, data.clone())?;
    assert_eq!(data, sot.data(0)?);
    assert!(sot.inconsistencies().is_empty());
    Ok(())
}

#[test]
fn finds_root() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    assert_eq!(0, sot.find(0, "")?);
    Ok(())
}

/// @todo #1:30min Let's implement this method. It has to find
///  all edges departing from the given one and return a vector
///  of tuples, where first element is the label of the edge
///  and the second one is the vertex this edge points to.
#[test]
#[ignore]
fn finds_all_kids() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    sot.add(1)?;
    sot.bind(0, 1, "one")?;
    sot.bind(0, 1, "two")?;
    assert_eq!(2, sot.kids(0).iter().count());
    Ok(())
}
