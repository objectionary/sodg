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
use crate::Vertices;
#[cfg(feature = "sober")]
use std::collections::HashSet;

impl<const N: usize> Sodg<N> {
    /// Make an empty [`Sodg`], with no vertices and no edges.
    ///
    /// # Panics
    ///
    /// May panic if vertices provided to alerts are absent (should never happen, though).
    #[must_use]
    pub fn empty(cap: usize) -> Self {
        let mut g = Self {
            vertices: Vertices::with_capacity(cap),
            next_v: 0,
            alerts: vec![],
            alerts_active: true,
            #[cfg(feature = "sober")]
            finds: HashSet::new(),
        };
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(*v).unwrap().edges {
                    if !g.vertices.contains(e.1) {
                        errors.push(format!("Edge ν{v}.{} arrives to lost ν{}", e.0, e.1));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(*v).unwrap().edges {
                    if e.1 == *v {
                        errors.push(format!("Edge ν{v}.{} arrives to ν{} (loop)", e.0, e.1));
                    }
                }
            }
            errors
        });
        g.alert_on(|g, vx| {
            let mut errors = Vec::new();
            for v in &vx {
                for e in &g.vertices.get(*v).unwrap().edges {
                    if !g.vertices.contains(e.1) {
                        errors.push(format!(
                            "Edge ν{v}.{} points to ν{}, which doesn't exist",
                            e.0, e.1
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

#[cfg(test)]
use crate::Label;

#[test]
fn makes_an_empty_sodg() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert_eq!(1, g.vertices.len());
    Ok(())
}

#[test]
fn prohibits_loops() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.alerts_off();
    g.add(0);
    g.bind(0, 0, Label::Alpha(0))?;
    assert!(g.alerts_on().is_err());
    Ok(())
}
