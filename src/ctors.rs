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

use crate::Sodg;
use std::collections::HashMap;

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
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in vx.iter() {
                for e in g.vertices.get(v).unwrap().edges.iter() {
                    if e.to == *v {
                        errors.push(format!("Edge ν{}.{} arrives to ν{} (loop)", v, e.a, e.to));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in vx.iter() {
                for e in g.vertices.get(v).unwrap().edges.iter() {
                    if e.a.is_empty() {
                        errors.push(format!("Edge from ν{} to ν{} has empty label", v, e.to));
                    }
                }
            }
            errors
        });
        g
    }
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

#[test]
fn prohibits_loops() -> Result<()> {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0)?;
    g.bind(0, 0, "foo")?;
    assert!(g.alerts_on().is_err());
    Ok(())
}

#[test]
fn prohibits_empty_labels() -> Result<()> {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "")?;
    assert!(g.alerts_on().is_err());
    Ok(())
}
