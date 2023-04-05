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
use rstest::rstest;
use rustc_hash::FxHashMap;

#[cfg(feature = "sober")]
use std::collections::HashSet;

impl Sodg {
    /// Make an empty [`Sodg`], with no vertices and no edges.
    ///
    /// # Panics
    ///
    /// May panic if vertices provided to alerts are absent (should never happen, though).
    #[must_use]
    pub fn empty() -> Self {
        let mut g = Self {
            vertices: FxHashMap::default(),
            next_v: 0,
            alerts: vec![],
            alerts_active: true,
            #[cfg(feature = "sober")]
            finds: HashSet::new(),
        };
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(v).unwrap().edges {
                    if !g.vertices.contains_key(&e.to) {
                        errors.push(format!("Edge ν{v}.{} arrives to lost ν{}", e.a, e.to));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(v).unwrap().edges {
                    if e.to == *v {
                        errors.push(format!("Edge ν{v}.{} arrives to ν{} (loop)", e.a, e.to));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(v).unwrap().edges {
                    if e.a.is_empty() {
                        errors.push(format!("Edge from ν{v} to ν{} has empty label", e.to));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(v).unwrap().edges {
                    if !g.vertices.contains_key(&e.to) {
                        errors.push(format!(
                            "Edge ν{v}.{} points to ν{}, which doesn't exist",
                            e.a, e.to
                        ));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(v).unwrap().edges {
                    if e.a.is_empty() {
                        errors.push(format!(
                            "Edge label from ν{} to ν{} is an empty string",
                            v, e.to
                        ));
                    }
                    if e.a.contains(' ') {
                        errors.push(format!(
                            "Edge label from ν{} to ν{} has prohibited spaces: '{}'",
                            v, e.to, e.a
                        ));
                    }
                    let parts: Vec<&str> = e.a.split('/').collect();
                    if parts.len() > 2 {
                        errors.push(format!(
                            "Edge label from ν{} to ν{} has more than one slash: '{}'",
                            v, e.to, e.a
                        ));
                    }
                    if parts[0].contains('.') {
                        errors.push(format!(
                            "Edge label from ν{} to ν{} has a dot inside the head part: '{}'",
                            v, e.to, e.a
                        ));
                    }
                    if parts.len() == 2 && parts[1].is_empty() {
                        errors.push(format!(
                            "Edge label from ν{} to ν{} has an empty tail part: '{}'",
                            v, e.to, e.a
                        ));
                    }
                }
            }
            errors
        });
        g.alerts_off();
        #[cfg(feature = "sober")]
        g.alerts_on().unwrap();
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

#[test]
fn prohibits_labels_with_dot() -> Result<()> {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a.b")?;
    assert!(g.alerts_on().is_err());
    Ok(())
}

#[test]
fn prohibits_labels_with_empty_tail() -> Result<()> {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a/")?;
    assert!(g.alerts_on().is_err());
    Ok(())
}

#[test]
fn prohibits_orphan_edges() -> Result<()> {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0)?;
    assert!(g.bind(0, 1, "foo").is_err());
    Ok(())
}

#[rstest]
#[case("")]
#[case("with spaces")]
#[case("with/two/slashes")]
fn prohibits_labels_of_broken_format(#[case] a: &str) {
    let mut g = Sodg::empty();
    g.alerts_off();
    g.add(0).unwrap();
    g.add(1).unwrap();
    g.bind(0, 1, a).unwrap();
    assert!(g.alerts_on().is_err());
}
