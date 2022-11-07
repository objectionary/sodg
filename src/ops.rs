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

use crate::Edge;
use crate::Hex;
use crate::Sodg;
use crate::Vertex;
use anyhow::{anyhow, Context, Result};
use log::trace;
use rstest::rstest;
use std::collections::VecDeque;
use std::str::FromStr;

impl Sodg {
    /// Add a new vertex `v1` to the Sodg:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "hello").unwrap();
    /// ```
    ///
    /// If vertex `v1` already exists in the graph, `Ok` will be returned.
    pub fn add(&mut self, v1: u32) -> Result<()> {
        if self.vertices.contains_key(&v1) {
            return Ok(());
        }
        self.vertices.insert(v1, Vertex::empty());
        self.validate(vec![v1])?;
        trace!("#add(ν{}): new vertex added", v1);
        Ok(())
    }

    /// Makes an edge `e1` from vertex `v1` to vertex `v2` and puts `a` label on it. If the
    /// label is not equal to `"ρ"`, makes two backward edges from `v2` to `v1`
    /// and label them as `"ρ"` an `"𝜎"`.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "forward").unwrap();
    /// g.bind(42, 0, "backward").unwrap();
    /// ```
    ///
    /// If an edge with this label already exists, it will be replaced with a new edge.
    ///
    /// If either vertex `v1` or `v2` is absent, an `Err` will be returned.
    ///
    /// If `v1` equals to `v2`, an `Err` will be returned.
    ///
    /// The label `a` can't be empty. If it's empty, an `Err` will be returned.
    pub fn bind(&mut self, v1: u32, v2: u32, a: &str) -> Result<()> {
        let vtx1 = self
            .vertices
            .get_mut(&v1)
            .context(format!("Can't depart from ν{}, it's absent", v1))?;
        vtx1.edges
            .retain(|e| Self::split_a(e.a.clone()).0 != Self::split_a(a.to_string()).0);
        vtx1.edges.push(Edge::new(v2, a));
        self.validate(vec![v1, v2])?;
        trace!("#bind: edge added ν{}-{}->ν{}", v1, a, v2);
        Ok(())
    }

    /// Set vertex data.
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(42).unwrap();
    /// g.put(42, Hex::from_str("hello, world!")).unwrap();
    /// ```
    ///
    /// If vertex `v1` is absent, an `Err` will be returned.
    pub fn put(&mut self, v: u32, d: Hex) -> Result<()> {
        let vtx = self
            .vertices
            .get_mut(&v)
            .context(format!("Can't find ν{}", v))?;
        vtx.data = d.clone();
        self.validate(vec![v])?;
        trace!("#data: data of ν{} set to {}", v, d);
        Ok(())
    }

    /// Read vertex data.
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(42).unwrap();
    /// let data = Hex::from_str("hello, world!");
    /// g.put(42, data.clone()).unwrap();
    /// assert_eq!(data, g.data(42).unwrap());
    /// ```
    ///
    /// If vertex `v1` is absent, an `Err` will be returned.
    pub fn data(&self, v: u32) -> Result<Hex> {
        let vtx = self
            .vertices
            .get(&v)
            .context(format!("Can't find ν{}", v))?;
        Ok(vtx.data.clone())
    }

    /// Find all kids of a vertex.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "k").unwrap();
    /// let (a, tail, to) = g.kids(0).unwrap().first().unwrap().clone();
    /// assert_eq!("k", a);
    /// assert_eq!("", tail);
    /// assert_eq!(42, to);
    /// ```
    ///
    /// If vertex `v1` is absent, `None` will be returned.
    ///
    /// Just in case, if you need to put all names into a single line:
    ///
    /// ```
    /// use itertools::Itertools;
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "a").unwrap();
    /// g.bind(0, 42, "b/d.f.e").unwrap();
    /// g.bind(0, 42, "c/hello-world").unwrap();
    /// assert_eq!("a,b,c", g.kids(0).unwrap().into_iter().map(|(a, _, _)| a).collect::<Vec<String>>().join(","));
    /// ```
    pub fn kids(&self, v: u32) -> Result<Vec<(String, String, u32)>> {
        let vtx = self.vertices.get(&v).context(format!("Can't find ν{v}"))?;
        let kids = vtx
            .edges
            .iter()
            .map(|x| {
                let p = Self::split_a(x.a.clone());
                (p.0, p.1, x.to)
            })
            .collect();
        Ok(kids)
    }

    /// Find a kid of a vertex, by its edge name.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "k").unwrap();
    /// assert_eq!(42, g.kid(0, "k").unwrap());
    /// assert!(g.kid(0, "another").is_none());
    /// ```
    ///
    /// If vertex `v1` is absent, `None` will be returned.
    ///
    /// The name of the edge may be a composite of two parts, for example
    /// `π/Φ.test` or `foo/ν13.print.me`. The parts are separated by the
    /// forward slash. In this case, the search will only take into account
    /// the first part:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "π/Φ.test").unwrap();
    /// assert_eq!(Some(42), g.kid(0, "π"));
    /// ```
    ///
    pub fn kid(&self, v: u32, a: &str) -> Option<u32> {
        if let Some(vtx) = self.vertices.get(&v) {
            vtx.edges
                .iter()
                .find(|e| Self::split_a(e.a.clone()).0 == Self::split_a(a.to_string()).0)
                .map(|e| e.to)
        } else {
            None
        }
    }

    /// Get a locator of an edge, if it exists.
    /// The name of the edge may be a composite of two parts, for example
    /// `π/Φ.foo` or `foo/ν6.boom.x.y`. The parts are separated by the
    /// forward slash. This function returns the second part if it exists:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "π/Φ.test").unwrap();
    /// assert_eq!(Some("Φ.test".to_string()), g.loc(0, "π"));
    /// assert_eq!(None, g.loc(0, "foo"));
    /// ```
    ///
    /// If there is no second part, but the edge is present, an empty string
    /// will be returned:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "π").unwrap();
    /// assert_eq!(Some("".to_string()), g.loc(0, "π"));
    /// ```
    ///
    pub fn loc(&self, v: u32, a: &str) -> Option<String> {
        if let Some(vtx) = self.vertices.get(&v) {
            vtx.edges
                .iter()
                .map(|e| Self::split_a(e.a.clone()))
                .find(|(l, _)| l == a)
                .map(|(_, r)| r)
        } else {
            None
        }
    }

    /// Find a vertex in the Sodg by its locator.
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(1).unwrap();
    /// g.bind(0, 1, "a").unwrap();
    /// g.add(2).unwrap();
    /// g.bind(1, 2, "b").unwrap();
    /// assert_eq!(2, g.find(0, "a.b").unwrap());
    /// ```
    ///
    /// If target vertex is not found or `v1` is absent,
    /// an `Err` will be returned.
    pub fn find(&self, v1: u32, loc: &str) -> Result<u32> {
        let mut v = v1;
        let mut locator: VecDeque<String> = VecDeque::new();
        loc.split('.')
            .filter(|k| !k.is_empty())
            .for_each(|k| locator.push_back(k.to_string()));
        loop {
            let next = locator.pop_front();
            if next.is_none() {
                trace!("#find: end of locator, we are at ν{v}");
                break;
            }
            let k = next.unwrap().to_string();
            if k.is_empty() {
                return Err(anyhow!("System error, the locator is empty"));
            }
            if k.starts_with('ν') {
                let num: String = k.chars().skip(1).collect::<Vec<_>>().into_iter().collect();
                v = u32::from_str(num.as_str())?;
                trace!("#find: jumping directly to ν{v}");
                continue;
            }
            if let Some(to) = self.kid(v, k.as_str()) {
                trace!("#find: ν{v}.{k} -> ν{to}");
                v = to;
                continue;
            };
            let others: Vec<String> = self
                .vertices
                .get(&v)
                .context(format!("Can't find ν{v}"))
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
        trace!("#find: found ν{v1} by '{loc}'");
        Ok(v)
    }

    /// Split label into two parts.
    fn split_a(a: String) -> (String, String) {
        let s = a.splitn(2, '/').collect::<Vec<&str>>();
        (
            s.first().unwrap().to_string(),
            s.get(1).unwrap_or(&"").to_string(),
        )
    }
}

#[rstest]
#[case("hello", "hello", "")]
#[case("hello/", "hello", "")]
#[case("hello/a-1", "hello", "a-1")]
#[case("π/Φ.x.Δ", "π", "Φ.x.Δ")]
fn splits_label_correctly(#[case] a: &str, #[case] head: &str, #[case] tail: &str) {
    let s = Sodg::split_a(a.to_string());
    assert_eq!(head, s.0);
    assert_eq!(tail, s.1);
}

#[test]
fn adds_simple_vertex() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    assert_eq!(1, g.find(1, "")?);
    Ok(())
}

#[test]
fn binds_simple_vertices() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let k = "hello";
    g.bind(1, 2, k)?;
    assert_eq!(2, g.find(1, k)?);
    Ok(())
}

#[test]
fn pre_defined_ids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let k = "a-привет";
    g.bind(1, 2, k)?;
    assert_eq!(2, g.find(1, k)?);
    Ok(())
}

#[test]
fn binds_two_names() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "first")?;
    g.bind(1, 2, "second")?;
    assert_eq!(2, g.find(1, "first")?);
    Ok(())
}

#[rstest]
#[case("hello", "hello")]
#[case("hello/a.b.c", "hello")]
#[case("hello", "hello/f.f")]
#[case("hello", "hello/")]
#[case("hello/", "hello")]
fn overwrites_edges(#[case] before: &str, #[case] after: &str) {
    let mut g = Sodg::empty();
    g.add(1).unwrap();
    g.add(2).unwrap();
    g.bind(1, 2, before).unwrap();
    g.add(3).unwrap();
    g.bind(1, 3, after).unwrap();
    assert_eq!(3, g.kid(1, after).unwrap());
    assert_eq!(3, g.kid(1, before).unwrap());
}

#[test]
fn overwrites_edge() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "foo")?;
    g.add(3)?;
    g.bind(1, 3, "foo/ee")?;
    assert_eq!(3, g.kid(1, "foo").unwrap());
    assert_eq!(3, g.kid(1, "foo/ee").unwrap());
    Ok(())
}

#[test]
fn binds_to_root() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "x")?;
    assert!(g.kid(0, "ρ").is_none());
    assert!(g.kid(0, "σ").is_none());
    Ok(())
}

#[test]
fn sets_simple_data() -> Result<()> {
    let mut g = Sodg::empty();
    let data = Hex::from_str("hello");
    g.add(0)?;
    g.put(0, data.clone())?;
    assert_eq!(data, g.data(0)?);
    Ok(())
}

#[test]
fn finds_root() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert_eq!(0, g.find(0, "")?);
    Ok(())
}

#[test]
fn finds_all_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "one/f.d")?;
    g.bind(0, 1, "two")?;
    assert_eq!(2, g.kids(0)?.iter().count());
    let (a, tail, to) = g.kids(0)?.first().unwrap().clone();
    assert_eq!("one", a);
    assert_eq!("f.d", tail);
    assert_eq!(1, to);
    Ok(())
}

#[test]
fn builds_list_of_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "one")?;
    g.bind(0, 1, "two/d.f.hello-world")?;
    g.bind(0, 1, "three/")?;
    let names: Vec<String> = g.kids(0)?.into_iter().map(|(a, _, _)| a).collect();
    assert_eq!("one,two,three", names.join(","));
    Ok(())
}

#[test]
fn gets_data_from_empty_vertex() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert!(g.data(0)?.is_empty());
    Ok(())
}

#[test]
fn gets_absent_kid() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert!(g.kid(0, "hello").is_none());
    Ok(())
}

#[test]
fn gets_kid_from_absent_vertex() -> Result<()> {
    let g = Sodg::empty();
    assert!(g.kid(0, "hello").is_none());
    Ok(())
}

#[test]
fn finds_kid_by_prefix() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "π/Φ.test")?;
    assert_eq!(Some(1), g.kid(0, "π"));
    Ok(())
}

#[test]
fn finds_locator() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "π/Φ.test")?;
    assert_eq!(Some("Φ.test".to_string()), g.loc(0, "π"));
    Ok(())
}

#[test]
fn finds_empty_locator() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "π")?;
    assert_eq!(Some("".to_string()), g.loc(0, "π"));
    Ok(())
}

#[test]
fn adds_twice() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert!(g.add(0).is_ok());
    Ok(())
}

#[test]
fn replaces_ignoring_locator() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "π/Φ.one")?;
    g.bind(0, 1, "π/Φ.two")?;
    assert_eq!(1, g.kids(0)?.len());
    Ok(())
}
