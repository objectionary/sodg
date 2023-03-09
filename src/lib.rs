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
//! use sodg::Sodg;
//! let mut sodg = Sodg::empty();
//! sodg.add(0).unwrap();
//! sodg.add(1).unwrap();
//! sodg.bind(0, 1, "foo").unwrap();
//! ```

#![doc(html_root_url = "https://docs.rs/sodg/0.0.26")]
#![deny(warnings)]

mod alerts;
mod clone;
mod ctors;
mod debug;
mod dot;
mod edge;
mod find;
mod gc;
mod hex;
mod inspect;
mod merge;
mod misc;
mod next;
mod ops;
mod script;
mod serialization;
mod slice;
mod vertex;
mod xml;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::alerts::Alert;
pub(crate) use crate::edge::Edge;
pub use crate::hex::Hex;
pub use crate::script::Script;
pub(crate) use crate::vertex::Vertex;

#[cfg(feature = "sober")]
use std::collections::HashSet;

/// A struct that represents a Surging Object DiGraph (SODG).
///
/// You add vertices to it, bind them one to one with edges,
/// put data into some of them, and read data back, for example:
///
/// ```
/// use sodg::Sodg;
/// use sodg::DeadRelay;
/// let mut sodg = Sodg::empty();
/// sodg.add(0).unwrap();
/// sodg.add(1).unwrap();
/// sodg.bind(0, 1, "a").unwrap();
/// sodg.add(2).unwrap();
/// sodg.bind(1, 2, "b").unwrap();
/// assert_eq!(2, sodg.find(0, "a.b", &mut DeadRelay::default()).unwrap());
/// ```
///
/// This package is used in [reo](https://github.com/objectionary/reo)
/// project, as a memory model for objects and dependencies between them.
#[derive(Serialize, Deserialize)]
pub struct Sodg {
    /// This is a map of vertices with their unique numbers/IDs.
    vertices: HashMap<u32, Vertex>,
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

/// A relay that is used by [`Sodg::find()`] when it can't find an attribute.
///
/// The finding algorithm asks the relay for the name of the attribute to use instead
/// of the not found one, which is provided as the `a` argument to the relay. The
/// `v` argument provided to the relay is the ID of the vertex
/// where the attribute `a` is not found.
///
/// A relay may return a new vertex ID as a string `"ν42"`, for example.
/// Pretty much anything that the relay returns will be used
/// as a new search string, starting from the `v` vertex.
pub trait Relay {
    /// A method to be called when the searching algorithm
    /// fails to find the required attribute.
    ///
    /// The method must accept two arguments:
    /// 1) the ID of the vertex where the search algorithm found a problem,
    /// 2) the name of the edge it is trying to find.
    ///
    /// The method must return a new locator, which the algorithm will use.
    /// If it is just a string, it will be treated as a name of the attribute to
    /// try instead. If it starts from `"ν"`, it is treated as an absolute
    /// locator on the entire graph.
    fn re(&self, v: u32, a: &str) -> Result<String>;
}

/// A [`Relay`] that doesn't even try to find anything, but returns an error.
///
/// If you don't know what [`Relay`] to use, use [`DeadRelay::new()`].
pub struct DeadRelay {}

/// A [`Relay`] that is made of a lambda function.
///
/// The function must accept two arguments:
/// 1) `v` is the ID of the vertex where an attribute is not found,
/// and 2) `a` is the name of the attribute.
/// The function must return a new locator where the
/// search algorithm must continue. It can be just a name of a new attribute,
/// or an absolute locator (starting from `"ν"`) with dots inside.
pub struct LambdaRelay {
    lambda: fn(u32, &str) -> Result<String>,
}

/// A [`Relay`] that always returns the same `String`.
pub struct ConstRelay {
    s: String,
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
