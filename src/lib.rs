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

#![deny(warnings)]

mod debug;
mod edge;
mod hex;
mod inspect;
mod merge;
mod next;
mod ops;
mod script;
mod serialization;
mod slice;
mod vertex;
mod xml;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Alert = fn(g: &Sodg, vx: Vec<u32>) -> Vec<String>;

/// This struct represents a Simple Object DiGraph (SODG). You add vertices
/// to it, bind them one to one with edges
///
/// ```
/// use sodg::Sodg;
/// let mut sodg = Sodg::empty();
/// sodg.add(0).unwrap();
/// sodg.add(1).unwrap();
/// sodg.bind(0, 1, "a").unwrap();
/// sodg.add(2).unwrap();
/// sodg.bind(1, 2, "b").unwrap();
/// assert_eq!(2, sodg.find(0, "a.b").unwrap());
/// ```
#[derive(Serialize, Deserialize)]
pub struct Sodg {
    vertices: HashMap<u32, Vertex>,
    #[serde(skip_serializing, skip_deserializing)]
    next_v: u32,
    #[serde(skip_serializing, skip_deserializing)]
    alerts: Vec<Alert>,
    #[serde(skip_serializing, skip_deserializing)]
    alerts_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Hex {
    bytes: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord)]
struct Edge {
    pub to: u32,
    pub a: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Vertex {
    pub edges: Vec<Edge>,
    pub data: Hex,
}

pub struct Script {
    txt: String,
    vars: HashMap<String, u32>,
    root: u32,
}

impl Sodg {
    /// Makes an empty Sodg, with no vertices and no edges.
    pub fn empty() -> Self {
        let mut g = Sodg {
            vertices: HashMap::new(),
            next_v: 0,
            alerts: vec![],
            alerts_active: true,
        };
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in vx.iter() {
                for e in g.vertices.get(v).unwrap().edges.iter() {
                    if !g.vertices.contains_key(&e.to) {
                        errors.push(format!("Edge ν{}.{} arrives to lost ν{}", v, e.a, e.to));
                    }
                }
            }
            errors
        });
        g
    }

    /// Attach a new alert to this SODG.
    pub fn alert_on(&mut self, a: Alert) {
        self.alerts.push(a);
    }

    /// Disable all alerts.
    pub fn alerts_off(&mut self) {
        self.alerts_active = false;
    }

    /// Enable all alerts.
    pub fn alerts_on(&mut self) {
        self.alerts_active = true;
    }
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

#[cfg(test)]
use anyhow::Result;

#[test]
fn makes_an_empty_sodg() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert_eq!(1, g.vertices.len());
    Ok(())
}
