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
#[cfg(debug_assertions)]
use log::trace;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Abandoned,
    Connected,
    Busy,
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Abandoned => "abandoned",
            Self::Connected => "connected",
            Self::Busy => "busy",
        })
    }
}

impl<const N: usize> Sodg<N> {
    /// Attempt to collect the vertex (delete it from the graph).
    ///
    /// If there are no edges leading to it, then it is deleted and
    /// all its children are collected. Otherwise, nothing happens. For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Hex, Label, Sodg};
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(1);
    /// g.put(1, &Hex::from(0));
    /// g.add(2);
    /// g.put(2, &Hex::from(0));
    /// g.bind(1, 2, Label::from_str("x").unwrap());
    /// g.data(2).unwrap();
    /// g.collect().unwrap(); // Both vertices are removed
    /// assert!(g.data(2).is_err());
    /// ```
    ///
    /// # Algorithm
    ///
    /// At the moment, the algorithm is naive. There are three steps.
    ///
    /// First, it scrolls multiple times through all available vertices
    /// in order to detect which of them are connected to the root. All
    /// vertices that are not in the detected group are called "Abandoned."
    /// These vertices are the candidates for garbage collecting. The vertices
    /// that are connected to the root are called "Connected.".
    ///
    /// Second, it scrolls multiple times through all Abandoned vertices
    /// in order to detect those that are not connected anyhow to data
    /// (not yet taken). The vertices that are connected to the not-yet-taken
    /// data are called "Busy."
    ///
    /// Third, it scrolls multiple times through all Abandoned vertices
    /// (not Busy and not Connected) and
    /// removes those that have no parents (only kids).
    ///
    /// # Errors
    ///
    /// If something goes wrong, an error may be returned.
    ///
    /// # Panics
    ///
    /// May panic!
    pub fn collect(&mut self) {
        let mut all = HashMap::new();
        for (v, _) in self.edges.iter() {
            all.insert(v, Status::Abandoned);
        }
        if all.contains_key(&0) {
            all.insert(0, Status::Connected);
        }
        loop {
            let mut modified = false;
            let vec: Vec<(usize, Status)> = all
                .clone()
                .into_iter()
                .filter(|(_, s)| *s == Status::Connected)
                .collect();
            for (v, _) in &vec {
                for (_, to) in self.edges.get(*v).unwrap() {
                    if *all.get(&to).unwrap() == Status::Abandoned {
                        all.insert(to, Status::Connected);
                        modified = true;
                    }
                }
            }
            if !modified {
                break;
            }
        }
        loop {
            let mut modified = false;
            let vec: Vec<(usize, Status)> = all
                .clone()
                .into_iter()
                .filter(|(_, s)| *s != Status::Busy)
                .collect();
            for (v, _) in vec {
                let edges = self.edges.get(v).unwrap();
                if self.data.contains_key(v) && !self.taken.contains_key(v) {
                    all.insert(v, Status::Busy);
                    modified = true;
                }
                for (_, to) in edges {
                    if *all.get(&to).unwrap() == Status::Busy {
                        all.insert(v, Status::Busy);
                        modified = true;
                    }
                }
            }
            if !modified {
                break;
            }
        }
        #[cfg(debug_assertions)]
        let mut total = 0;
        loop {
            let mut modified = false;
            let vec: Vec<(usize, Status)> = all
                .clone()
                .into_iter()
                .filter(|(_, s)| *s == Status::Abandoned)
                .collect();
            for (v, _) in vec {
                let edges = self.edges.get(v).unwrap();
                if edges.into_iter().next().is_none() {
                    // self.remove(v); DO SOMETHING ABOUT THIS!
                    all.remove(&v);
                    modified = true;
                    #[cfg(debug_assertions)]
                    {
                        trace!("#collect: ν{v} removed");
                        total += 1;
                    }
                }
            }
            if !modified {
                break;
            }
        }
        #[cfg(debug_assertions)]
        trace!("#collect: collected {total} vertices, status: {:?}", all);
        }
}

#[cfg(test)]
use crate::Label;

#[cfg(test)]
use std::str::FromStr;

#[test]
fn does_not_collect_owned() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    g.bind(0, 1, Label::from_str("x").unwrap());
    g.collect();
    assert!(g.edges.get(1).is_some());
}

#[test]
fn collects_simple_graph() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(2);
    g.add(3);
    g.add(4);
    g.bind(1, 2, Label::from_str("x").unwrap());
    g.bind(1, 3, Label::from_str("y").unwrap());
    g.bind(2, 4, Label::from_str("z").unwrap());
    g.data(4).unwrap();
    g.data(2).unwrap();
    g.data(1).unwrap();
    g.data(3).unwrap();
    g.collect();
    assert_eq!(0, g.len());
}

#[test]
fn collects_complicated_graph() {
    let mut g: Sodg<16> = Sodg::empty(256);
    for i in 1..=5 {
        g.add(i);
    }
    g.bind(1, 2, Label::from_str("x").unwrap());
    g.bind(1, 3, Label::from_str("y").unwrap());
    g.bind(2, 4, Label::from_str("z").unwrap());
    g.bind(3, 5, Label::from_str("a").unwrap());
    g.bind(4, 3, Label::from_str("b").unwrap());
    for i in 1..=5 {
        g.data(i).unwrap();
    }
    g.collect();
    assert_eq!(0, g.len());
}
