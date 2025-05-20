// SPDX-FileCopyrightText: Copyright (c) 2022-2025 Objectionary.com
// SPDX-License-Identifier: MIT

use crate::{Hex, Label, Vertex};
use crate::{Persistence, Sodg, BRANCH_NONE, BRANCH_STATIC};
use anyhow::Context;
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
    /// If vertex `v1` already exists in the graph, nothing will happen.
    ///
    /// # Panics
    ///
    /// If alerts trigger any error, the error will be returned here.
    #[inline]
    pub fn add(&mut self, v1: usize) {
        let vertex = Vertex {
            branch: 1,
            data: Hex::empty(),
            persistence: Persistence::Empty,
            edges: micromap::Map::new(),
        };
        self.vertices.insert(v1, vertex);
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
        let mut ours = self.vertices.get(v1).unwrap().branch;
        let theirs = self.vertices.get(v2).unwrap().branch;
        let vtx1 = self.vertices.get_mut(v1).unwrap();
        vtx1.edges.insert(a, v2);
        if ours == BRANCH_STATIC {
            if theirs == BRANCH_STATIC {
                for b in self.branches.iter_mut() {
                    if b.1.is_empty() {
                        b.1.push(v1);
                        ours = b.0;
                        vtx1.branch = ours;
                        break;
                    }
                }
                self.vertices.get_mut(v2).unwrap().branch = ours;
                self.branches.get_mut(ours).unwrap().push(v2);
            } else {
                vtx1.branch = theirs;
                self.branches.get_mut(theirs).unwrap().push(v1);
            }
        } else {
            let vtx2 = self.vertices.get_mut(v2).unwrap();
            if vtx2.branch == BRANCH_STATIC {
                vtx2.branch = ours;
                self.branches.get_mut(ours).unwrap().push(v2);
            }
        }
        #[cfg(debug_assertions)]
        trace!(
            "#bind: edge added ν{}(b={}).{} → ν{}(b={})",
            v1,
            self.vertices.get(v1).unwrap().branch,
            a,
            v2,
            self.vertices.get(v2).unwrap().branch,
        );
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
        let vtx = self.vertices.get_mut(v).unwrap();
        vtx.persistence = Persistence::Stored;
        vtx.data = d.clone();
        *self.stores.get_mut(vtx.branch).unwrap() += 1;
        #[cfg(debug_assertions)]
        trace!("#put: data of ν{v} set to {d}");
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
    /// # Panics
    ///
    /// If vertex `v1` is absent, it will panic.
    #[inline]
    pub fn data(&mut self, v: usize) -> Option<Hex> {
        self.vertices.get(v)?;
        let vtx = self.vertices.get_mut(v).unwrap();
        match vtx.persistence {
            Persistence::Stored => {
                let d = vtx.data.clone();
                vtx.persistence = Persistence::Taken;
                let branch = vtx.branch;
                let s = self.stores.get_mut(branch).unwrap();
                *s -= 1;
                if *s == 0 {
                    let members = self.branches.get_mut(branch).unwrap();
                    for v in members.into_iter() {
                        if self.vertices.get(v).is_none() {
                            continue;
                        }
                        self.vertices.get_mut(v).unwrap().branch = BRANCH_NONE;
                    }
                    #[cfg(debug_assertions)]
                    trace!(
                        "#data: branch no.{} destroyed {} vertices as garbage: {}",
                        branch,
                        members.len(),
                        members
                            .into_iter()
                            .map(|v| format!("ν{v}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                    members.clear();
                }
                #[cfg(debug_assertions)]
                trace!("#data: data of ν{v} retrieved");
                Some(d)
            }
            Persistence::Taken => {
                #[cfg(debug_assertions)]
                trace!("#data: data of ν{v} retrieved again");
                Some(vtx.data.clone())
            }
            Persistence::Empty => None,
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
    /// let (a, to) = g.kids(0).next().unwrap().clone();
    /// assert_eq!("k", a.to_string());
    /// assert_eq!(42, *to);
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
    /// let mut names = g.kids(0).into_iter().map(|(a, _)| a.to_string()).collect::<Vec<String>>();
    /// names.sort();
    /// assert_eq!("a,b,c", names.join(","));
    /// ```
    ///
    /// # Panics
    ///
    /// If vertex `v1` is absent, `Err` will be returned.
    #[inline]
    pub fn kids(&self, v: usize) -> impl Iterator<Item = (&Label, &usize)> + '_ {
        self.vertices
            .get(v)
            .with_context(|| format!("Can't find ν{v} in kids()"))
            .unwrap()
            .edges
            .iter()
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
    /// # Panics
    ///
    /// If vertex `v1` is absent, it will panic.
    #[must_use]
    #[inline]
    pub fn kid(&self, v: usize, a: Label) -> Option<usize> {
        self.vertices.get(v)?;
        for e in &self.vertices.get(v).unwrap().edges {
            if *e.0 == a {
                return Some(*e.1);
            }
        }
        None
    }
}

#[cfg(test)]
use std::str::FromStr;

#[test]
fn adds_simple_vertex() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.bind(1, 2, Label::Alpha(0));
    assert_eq!(2, g.len());
}

#[test]
fn sets_branch_correctly() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.bind(1, 2, Label::Alpha(0));
    assert_eq!(1, g.branches.get(1).unwrap().len());
    assert_eq!(2, g.branches.get(2).unwrap().len());
    g.put(2, &Hex::from(42));
    assert_eq!(&1, g.stores.get(2).unwrap());
    g.add(3);
    g.bind(1, 3, Label::Alpha(1));
    assert_eq!(3, g.branches.get(2).unwrap().len());
    g.add(4);
    g.add(5);
    g.bind(4, 5, Label::Alpha(0));
    assert_eq!(2, g.branches.get(3).unwrap().len());
    g.data(2);
    assert_eq!(0, g.branches.get(2).unwrap().len());
}

#[test]
fn fetches_kid() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    let k = Label::from_str("hello").unwrap();
    g.bind(1, 2, k);
    assert_eq!(2, g.kid(1, k).unwrap());
}

#[test]
fn binds_two_names() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    let first = Label::from_str("first").unwrap();
    g.bind(1, 2, first);
    let second = Label::from_str("second").unwrap();
    g.bind(1, 2, second);
    assert_eq!(2, g.kid(1, first).unwrap());
    assert_eq!(2, g.kid(1, second).unwrap());
}

#[test]
fn overwrites_edge() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.bind(1, 2, Label::from_str("foo").unwrap());
    g.add(3);
    g.bind(1, 3, Label::from_str("foo").unwrap());
    assert_eq!(3, g.kid(1, Label::from_str("foo").unwrap()).unwrap());
}

#[test]
fn binds_to_root() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("x").unwrap());
    assert!(g.kid(0, Label::from_str("ρ").unwrap()).is_none());
    assert!(g.kid(0, Label::from_str("σ").unwrap()).is_none());
}

#[test]
fn sets_simple_data() {
    let mut g: Sodg<16> = Sodg::empty(256);
    let data = Hex::from_str_bytes("hello");
    g.add(0);
    g.put(0, &data);
    assert_eq!(data, g.data(0).unwrap());
}

#[test]
fn collects_garbage() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.bind(1, 2, Label::Alpha(0));
    g.put(2, &Hex::from_str_bytes("hello"));
    g.add(3);
    g.bind(1, 3, Label::Alpha(0));
    assert_eq!(3, g.len());
    assert_eq!(3, g.branches.get(2).unwrap().len());
    g.data(2);
    assert_eq!(0, g.len());
}

#[test]
fn finds_all_kids() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("one").unwrap());
    g.bind(0, 1, Label::from_str("two").unwrap());
    assert_eq!(2, g.kids(0).count());
    let mut names = vec![];
    for (a, to) in g.kids(0) {
        names.push(format!("{a}/{to}"));
    }
    names.sort();
    assert_eq!("one/1,two/1", names.join(","));
}

#[test]
fn builds_list_of_kids() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("one").unwrap());
    g.bind(0, 1, Label::from_str("two").unwrap());
    g.bind(0, 1, Label::from_str("three").unwrap());
    let mut names: Vec<String> = g.kids(0).map(|(a, _)| format!("{a}")).collect();
    names.sort();
    assert_eq!("one,three,two", names.join(","));
}

#[test]
fn gets_data_from_empty_vertex() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert!(g.data(0).is_none());
}

#[test]
fn gets_absent_kid() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert!(g.kid(0, Label::from_str("hello").unwrap()).is_none());
}

#[test]
fn gets_kid_from_absent_vertex() {
    let g: Sodg<16> = Sodg::empty(256);
    assert!(g.kid(0, Label::from_str("hello").unwrap()).is_none());
}

#[test]
fn adds_twice() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(0);
}
