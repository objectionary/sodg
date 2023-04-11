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

use crate::Hex;
use crate::Sodg;
use anyhow::{Context, Result};
#[cfg(debug_assertions)]
use log::trace;

impl Sodg {
    /// Add a new vertex `v1` to itself.
    ///
    /// For example:
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
    ///
    /// # Errors
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn add(&mut self, v1: u32) -> Result<()> {
        if self.vertices.contains(v1) {
            return Ok(());
        }
        self.vertices.insert(v1);
        #[cfg(not(debug_assertions))]
        {
            self.next_v = v1 + 1;
        }
        #[cfg(debug_assertions)]
        self.validate(vec![v1])?;
        #[cfg(debug_assertions)]
        trace!("#add: vertex ν{v1} added");
        Ok(())
    }

    /// Make an edge `e1` from vertex `v1` to vertex `v2` and put `a` label on it.
    ///
    /// For example:
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
    /// # Errors
    ///
    /// If either vertex `v1` or `v2` is absent, an `Err` will be returned.
    ///
    /// If `v1` equals to `v2`, an `Err` will be returned.
    ///
    /// The label `a` can't be empty. If it is empty, an `Err` will be returned.
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn bind(&mut self, v1: u32, v2: u32, a: &str) -> Result<()> {
        let vtx1 = self
            .vertices
            .get_mut(v1)
            .with_context(|| format!("Can't depart from ν{v1}, it's absent"))?;
        vtx1.edges.insert(a.to_string(), v2);
        #[cfg(feature = "gc")]
        let vtx2 = self
            .vertices
            .get_mut(v2)
            .with_context(|| format!("Can't arrive at ν{v2}, it's absent"))?;
        #[cfg(feature = "gc")]
        vtx2.parents.insert(v1);
        #[cfg(debug_assertions)]
        self.validate(vec![v1, v2])?;
        #[cfg(debug_assertions)]
        trace!("#bind: edge added ν{}.{} → ν{}", v1, a, v2);
        Ok(())
    }

    /// Set vertex data.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(42).unwrap();
    /// g.put(42, &Hex::from_str_bytes("hello, world!")).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If vertex `v1` is absent, an `Err` will be returned.
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn put(&mut self, v: u32, d: &Hex) -> Result<()> {
        let vtx = self
            .vertices
            .get_mut(v)
            .with_context(|| format!("Can't find ν{v} in put()"))?;
        vtx.data = Some(d.clone());
        #[cfg(debug_assertions)]
        self.validate(vec![v])?;
        #[cfg(debug_assertions)]
        trace!("#data: data of ν{v} set to {d}");
        Ok(())
    }

    /// Read vertex data, and then submit the vertex to garbage collection.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(42).unwrap();
    /// let data = Hex::from_str_bytes("hello, world!");
    /// g.put(42, &data).unwrap();
    /// assert_eq!(data, g.data(42).unwrap());
    /// #[cfg(feature = "gc")]
    /// assert!(g.is_empty());
    /// ```
    ///
    /// If there is no data, an empty `Hex` will be returned, for example:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(42).unwrap();
    /// assert!(g.data(42).unwrap().is_empty());
    /// ```
    ///
    /// # Errors
    ///
    /// If vertex `v1` is absent, an `Err` will be returned.
    ///
    /// If garbage collection triggers any error, the error will be returned here.
    #[inline]
    pub fn data(&mut self, v: u32) -> Result<Hex> {
        let vtx = self
            .vertices
            .get_mut(v)
            .with_context(|| format!("Can't find ν{v} in data()"))?;
        if let Some(d) = vtx.data.clone() {
            vtx.taken = true;
            #[cfg(feature = "gc")]
            self.collect(v)?;
            #[cfg(debug_assertions)]
            trace!("#data: data of ν{v} retrieved");
            Ok(d)
        } else {
            Ok(Hex::empty())
        }
    }

    /// Find all kids of a vertex.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "k").unwrap();
    /// let (a, to) = g.kids(0).unwrap().first().unwrap().clone();
    /// assert_eq!("k", a);
    /// assert_eq!(42, to);
    /// ```
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
    /// g.bind(0, 42, "b").unwrap();
    /// g.bind(0, 42, "c").unwrap();
    /// let mut names = g.kids(0).unwrap().into_iter().map(|(a, _)| a).collect::<Vec<String>>();
    /// names.sort();
    /// assert_eq!("a,b,c", names.join(","));
    /// ```
    ///
    /// # Errors
    ///
    /// If vertex `v1` is absent, `Err` will be returned.
    #[inline]
    pub fn kids(&self, v: u32) -> Result<Vec<(String, u32)>> {
        let vtx = self
            .vertices
            .get(v)
            .with_context(|| format!("Can't find ν{v} in kids()"))?;
        let kids = vtx.edges.iter().map(|(a, to)| (a.clone(), *to)).collect();
        Ok(kids)
    }

    /// Find a kid of a vertex, by its edge name, and return the ID of the vertex found.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.add(42).unwrap();
    /// g.bind(0, 42, "k").unwrap();
    /// assert_eq!(42, g.kid(0, "k").unwrap());
    /// ```
    ///
    /// If vertex `v1` is absent, `None` will be returned.
    #[must_use]
    #[inline]
    pub fn kid(&self, v: u32, a: &str) -> Option<u32> {
        self.vertices
            .get(v)
            .and_then(|vtx| vtx.edges.iter().find(|e| e.0 == a).map(|e| *e.1))
    }
}

#[cfg(test)]
use crate::DeadRelay;

#[test]
fn adds_simple_vertex() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    assert_eq!(1, g.find(1, "", &DeadRelay::default())?);
    Ok(())
}

#[test]
fn binds_simple_vertices() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let k = "hello";
    g.bind(1, 2, k)?;
    assert_eq!(2, g.find(1, k, &DeadRelay::default())?);
    Ok(())
}

#[test]
fn pre_defined_ids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    let k = "a-привет";
    g.bind(1, 2, k)?;
    assert_eq!(2, g.find(1, k, &DeadRelay::default())?);
    Ok(())
}

#[test]
fn binds_two_names() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "first")?;
    g.bind(1, 2, "second")?;
    assert_eq!(2, g.find(1, "first", &DeadRelay::default())?);
    assert_eq!(2, g.find(1, "second", &DeadRelay::default())?);
    Ok(())
}

#[test]
fn overwrites_edge() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "foo")?;
    g.add(3)?;
    g.bind(1, 3, "foo")?;
    assert_eq!(3, g.kid(1, "foo").unwrap());
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
    let data = Hex::from_str_bytes("hello");
    g.add(0)?;
    g.put(0, &data)?;
    assert_eq!(data, g.data(0)?);
    Ok(())
}

#[test]
#[cfg(feature = "gc")]
fn simple_data_gc() -> Result<()> {
    let mut g = Sodg::empty();
    let data = Hex::from_str_bytes("hello");
    g.add(0)?;
    g.put(0, data.clone())?;
    assert_eq!(data, g.data(0)?);
    assert!(g.is_empty());
    Ok(())
}

#[test]
fn finds_all_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "one")?;
    g.bind(0, 1, "two")?;
    assert_eq!(2, g.kids(0)?.len());
    let mut names = vec![];
    for (a, to) in g.kids(0)? {
        names.push(format!("{a}/{to}"));
    }
    names.sort();
    assert_eq!("one/1,two/1", names.join(","));
    Ok(())
}

#[test]
fn builds_list_of_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "one")?;
    g.bind(0, 1, "two")?;
    g.bind(0, 1, "three")?;
    let mut names: Vec<String> = g.kids(0)?.into_iter().map(|(a, _)| a).collect();
    names.sort();
    assert_eq!("one,three,two", names.join(","));
    Ok(())
}

#[test]
fn gets_data_from_empty_vertex() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert!(g.data(0).is_ok());
    assert!(g.data(0).unwrap().is_empty());
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
fn adds_twice() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert!(g.add(0).is_ok());
    Ok(())
}
