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

//! This is a memory structure with vertices and edges between them,
//! which we call Surging Object DiGraph (SODG), because it expects
//! modifications comping from a user (through [`Sodg::add`],
//! [`Sodg::bind`], and [`Sodg::put`]) and then decides itself when
//! it's time to delete some vertices (something similar to
//! "garbage collection").
//!
//! For example, here is how you create a simple
//! di-graph with two vertices and an edge between them:
//!
//! ```
//! use std::str::FromStr;
//! use sodg::{Label, Sodg};
//! let mut sodg = Sodg::empty();
//! sodg.add(0).unwrap();
//! sodg.add(1).unwrap();
//! sodg.bind(0, 1, Label::from_str("foo").unwrap()).unwrap();
//! ```

#![doc(html_root_url = "https://docs.rs/sodg/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]

mod alerts;
mod clone;
mod ctors;
mod debug;
mod dot;
mod edges;
mod gc;
mod hex;
mod inspect;
mod label;
mod merge;
mod misc;
mod next;
mod ops;
mod roll;
mod script;
mod serialization;
mod slice;
mod vertex;
mod vertices;
mod xml;

use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
#[cfg(feature = "gc")]
use std::collections::HashSet;

/// A function that is called when a problem is found in [`Sodg`].
///
/// Instances of this type can be used in [`Sodg::alert_on`] method,
/// in order to ensure runtime consistency of data inside the graph.
pub type Alert = fn(g: &Sodg, vx: Vec<u32>) -> Vec<String>;

/// An object-oriented representation of binary data
/// in hexadecimal format, which can be put into vertices of the graph.
///
/// You can create it from Rust primitives:
///
/// ```
/// use sodg::Hex;
/// let d = Hex::from(65534);
/// assert_eq!("00-00-00-00-00-00-FF-FE", d.print());
/// ```
///
/// Then, you can turn it back to Rust primitives:
///
/// ```
/// use sodg::Hex;
/// let d = Hex::from(65534);
/// assert_eq!(65534, d.to_i64().unwrap());
/// ```
#[derive(Serialize, Deserialize, Clone)]
pub enum Hex {
    Vector(Vec<u8>),
    Bytes([u8; 24], usize),
}

/// A label on an edge.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Label {
    Greek(char),
    Alpha(usize),
    Str([char; 8]),
}

/// A vertex in the [`Sodg`].
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Vertex {
    /// This is a list of edges departing from this vertex.
    pub edges: Edges,
    /// This is the data in the vertex (possibly empty).
    pub data: Option<Hex>,
    /// This is a supplementary list of parent nodes, staying here for caching.
    #[cfg(feature = "gc")]
    pub parents: HashSet<u32>,
    /// This is `TRUE` if the data has been already taken by the use of [`Sodg::data`].
    pub taken: bool,
}

/// Internal structure, map of all vertices.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Vertices {
    map: HashMap<u32, Vertex>,
}

/// Iterator over vertices.
pub(crate) struct VerticesIter<'a> {
    iter: Iter<'a, u32, Vertex>,
}

/// Internal structure, map of all edges.
#[derive(Clone)]
pub(crate) struct Edges {
    map: Roll<Label, u32, MAX_EDGES>,
}

/// Iterator over edges.
pub(crate) struct EdgesIntoIter<'a> {
    iter: RollIntoIter<'a, Label, u32, MAX_EDGES>,
}

const MAX_EDGES: usize = 10;

/// Memory structure for edges.
#[derive(Clone)]
pub struct Roll<K: Copy + PartialEq, V: Copy, const N: usize> {
    items: [Option<(K, V)>; N],
}

/// Iterator over roll.
pub struct RollIntoIter<'a, K, V, const N: usize> {
    pos: usize,
    items: &'a [Option<(K, V)>; N],
}

/// A wrapper of a plain text with graph-modifying instructions.
///
/// For example, you can pass the following instructions to it:
///
/// ```text
/// ADD(0);
/// ADD($ν1); # adding new vertex
/// BIND(0, $ν1, foo);
/// PUT($ν1, d0-bf-D1-80-d0-B8-d0-b2-d0-b5-d1-82);
/// ```
///
/// In the script you can use "variables", similar to `$ν1` used
/// in the text above. They will be replaced by autogenerated numbers
/// during the deployment of this script to a [`Sodg`].
pub struct Script {
    /// The text of it.
    txt: String,
    /// The vars dynamically discovered.
    vars: HashMap<String, u32>,
}

/// A struct that represents a Surging Object Di-Graph (SODG).
///
/// You add vertices to it, bind them one to one with edges,
/// put data into some of them, and read data back, for example:
///
/// ```
/// use sodg::{Label, Sodg};
/// let mut sodg = Sodg::empty();
/// sodg.add(0).unwrap();
/// sodg.add(1).unwrap();
/// sodg.bind(0, 1, Label::Alpha(0)).unwrap();
/// sodg.add(2).unwrap();
/// sodg.bind(1, 2, Label::Alpha(1)).unwrap();
/// assert_eq!(1, sodg.kids(0).unwrap().len());
/// assert_eq!(1, sodg.kids(1).unwrap().len());
/// ```
///
/// This package is used in [reo](https://github.com/objectionary/reo)
/// project, as a memory model for objects and dependencies between them.
#[derive(Serialize, Deserialize)]
pub struct Sodg {
    /// This is a map of vertices with their unique numbers/IDs.
    vertices: Vertices,
    /// This is the next ID of a vertex to be returned by the [`Sodg::next_v`] function.
    #[serde(skip_serializing, skip_deserializing)]
    next_v: u32,
    /// This is the list of alerts, which is managed by the [`Sodg::alert_on`] function.
    #[serde(skip_serializing, skip_deserializing)]
    alerts: Vec<Alert>,
    /// This is the flag that either enables or disables alerts, through [`Sodg::alerts_on`]
    /// and [`Sodg::alerts_off`].
    #[serde(skip_serializing, skip_deserializing)]
    alerts_active: bool,
    #[cfg(feature = "sober")]
    finds: HashSet<String>,
}

#[cfg(test)]
use simple_logger::SimpleLogger;

#[cfg(test)]
use log::LevelFilter;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();
}
