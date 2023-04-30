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
use crate::{Hex, Label};
use anyhow::{Context, Result};
#[cfg(debug_assertions)]
use log::trace;

impl<const N: usize> Sodg<N> {
    /// Add a new vertex `v1` to itself.
    ///
    /// For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Sodg};
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(0);
    /// g.add(42);
    /// g.bind(0, 42, Label::from_str("hello").unwrap());
    /// ```
    ///
    /// If vertex `v1` already exists in the graph, `Ok` will be returned.
    ///
    /// # Panics
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn add(&mut self, v1: usize) {
        self.alive.insert(v1, true);
        self.edges.get_mut(v1).unwrap().clear();
        self.data.remove(v1);
        #[cfg(debug_assertions)]
        trace!("#add: vertex ν{v1} added");
    }

    /// Make an edge `e1` from vertex `v1` to vertex `v2` and put `a` label on it.
    ///
    /// For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Sodg};
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(0);
    /// g.add(42);
    /// g.bind(0, 42, Label::from_str("forward").unwrap());
    /// g.bind(42, 0, Label::from_str("backward").unwrap());
    /// ```
    ///
    /// If an edge with this label already exists, it will be replaced with a new edge.
    ///
    /// # Panics
    ///
    /// If either vertex `v1` or `v2` is absent, an `Err` will be returned.
    ///
    /// If `v1` equals to `v2`, an `Err` will be returned.
    ///
    /// The label `a` can't be empty. If it is empty, an `Err` will be returned.
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn bind(&mut self, v1: usize, v2: usize, a: Label) {
        self
            .edges
            .get_mut(v1)
            .unwrap()
            .insert(a, v2);
        #[cfg(debug_assertions)]
        trace!("#bind: edge added ν{}.{} → ν{}", v1, a, v2);
    }

    /// Set vertex data.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(42);
    /// g.put(42, &Hex::from_str_bytes("hello, world!"));
    /// ```
    ///
    /// # Panics
    ///
    /// If vertex `v1` is absent, an `Err` will be returned.
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn put(&mut self, v: usize, d: &Hex) {
        self.data.insert(v, d.clone());
        #[cfg(debug_assertions)]
        trace!("#data: data of ν{v} set to {d}");
    }

    /// Read vertex data, and then submit the vertex to garbage collection.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(42);
    /// let data = Hex::from_str_bytes("hello, world!");
    /// g.put(42, &data);
    /// assert_eq!(data, g.data(42).unwrap());
    /// ```
    ///
    /// If there is no data, `None` will be returned, for example:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(42);
    /// assert!(g.data(42).is_none());
    /// ```
    ///
    /// # Errors
    ///
    /// If vertex `v1` is absent, an `Err` will be returned.
    ///
    /// If garbage collection triggers any error, the error will be returned here.
    #[inline]
    pub fn data(&mut self, v: usize) -> Option<Hex> {
        match self.data.get(v) {
            Some(d) => {
                self.taken.insert(v, true);
                #[cfg(debug_assertions)]
                trace!("#data: data of ν{v} retrieved");
                Some(d.clone())
            },
            None => None
        }
    }

    /// Find all kids of a vertex.
    ///
    /// For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Sodg};
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(0);
    /// g.add(42);
    /// g.bind(0, 42, Label::from_str("k").unwrap());
    /// let (a, to) = g.kids(0).unwrap().first().unwrap().clone();
    /// assert_eq!("k", a.to_string());
    /// assert_eq!(42, to);
    /// ```
    ///
    /// Just in case, if you need to put all names into a single line:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use itertools::Itertools;
    /// use sodg::{Label, Sodg};
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(0);
    /// g.add(42);
    /// g.bind(0, 42, Label::from_str("a").unwrap());
    /// g.bind(0, 42, Label::from_str("b").unwrap());
    /// g.bind(0, 42, Label::from_str("c").unwrap());
    /// let mut names = g.kids(0).unwrap().into_iter().map(|(a, _)| a.to_string()).collect::<Vec<String>>();
    /// names.sort();
    /// assert_eq!("a,b,c", names.join(","));
    /// ```
    ///
    /// # Errors
    ///
    /// If vertex `v1` is absent, `Err` will be returned.
    #[inline]
    pub fn kids(&self, v: usize) -> Vec<(Label, usize)> {
        self
            .edges
            .get(v)
            .with_context(|| format!("Can't find ν{v} in kids()"))
            .unwrap()
            .into_iter().map(|(a, to)| (a, to))
            .collect()
    }

    /// Find a kid of a vertex, by its edge name, and return the ID of the vertex found.
    ///
    /// For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Sodg};
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(0);
    /// g.add(42);
    /// let k = Label::from_str("k").unwrap();
    /// g.bind(0, 42, k);
    /// assert_eq!(42, g.kid(0, k).unwrap());
    /// ```
    ///
    /// If vertex `v1` is absent, `None` will be returned.
    #[must_use]
    #[inline]
    pub fn kid(&self, v: usize, a: Label) -> Option<usize> {
        self.edges
            .get(v)
            .and_then(|edges| edges.into_iter().find(|e| e.0 == a).map(|e| e.1))
    }

    /// Remove a vertex from the graph.
    ///
    /// All vertices that pointed to this one will lose the pointing edges.
    #[inline]
    pub fn remove(&mut self, v: usize) {
        self.alive.remove(v);
        self.edges.get_mut(v).unwrap().clear();
        for (_, edges) in self.edges.iter_mut() {
            edges.retain(|_, v1| *v1 != v)
        }
    }
}

#[cfg(test)]
use std::str::FromStr;

#[test]
fn adds_simple_vertex() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.bind(1, 2, Label::Alpha(0));
    assert_eq!(2, g.len());
    Ok(())
}

#[test]
fn fetches_kid() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    let k = Label::from_str("hello")?;
    g.bind(1, 2, k);
    assert_eq!(2, g.kid(1, k).unwrap());
    Ok(())
}

#[test]
fn binds_two_names() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    let first = Label::from_str("first")?;
    g.bind(1, 2, first);
    let second = Label::from_str("second")?;
    g.bind(1, 2, second);
    assert_eq!(2, g.kid(1, first).unwrap());
    assert_eq!(2, g.kid(1, second).unwrap());
    Ok(())
}

#[test]
fn overwrites_edge() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.bind(1, 2, Label::from_str("foo")?);
    g.add(3);
    g.bind(1, 3, Label::from_str("foo")?);
    assert_eq!(3, g.kid(1, Label::from_str("foo")?).unwrap());
    Ok(())
}

#[test]
fn binds_to_root() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("x")?);
    assert!(g.kid(0, Label::from_str("ρ")?).is_none());
    assert!(g.kid(0, Label::from_str("σ")?).is_none());
    Ok(())
}

#[test]
fn sets_simple_data() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    let data = Hex::from_str_bytes("hello");
    g.add(0);
    g.put(0, &data);
    assert_eq!(data, g.data(0).unwrap());
    Ok(())
}

#[test]
fn finds_all_kids() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("one")?);
    g.bind(0, 1, Label::from_str("two")?);
    assert_eq!(2, g.kids(0).len());
    let mut names = vec![];
    for (a, to) in g.kids(0) {
        names.push(format!("{a}/{to}"));
    }
    names.sort();
    assert_eq!("one/1,two/1", names.join(","));
    Ok(())
}

#[test]
fn builds_list_of_kids() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("one")?);
    g.bind(0, 1, Label::from_str("two")?);
    g.bind(0, 1, Label::from_str("three")?);
    let mut names: Vec<String> = g
        .kids(0)
        .into_iter()
        .map(|(a, _)| format!("{a}"))
        .collect();
    names.sort();
    assert_eq!("one,three,two", names.join(","));
    Ok(())
}

#[test]
fn gets_data_from_empty_vertex() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert!(g.data(0).is_none());
    Ok(())
}

#[test]
fn gets_absent_kid() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert!(g.kid(0, Label::from_str("hello")?).is_none());
    Ok(())
}

#[test]
fn gets_kid_from_absent_vertex() -> Result<()> {
    let g: Sodg<16> = Sodg::empty(256);
    assert!(g.kid(0, Label::from_str("hello")?).is_none());
    Ok(())
}

#[test]
fn adds_twice() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(0);
    Ok(())
}
