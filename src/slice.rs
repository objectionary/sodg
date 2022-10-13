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
use anyhow::Result;
use std::collections::{HashMap, HashSet};

impl Sodg {
    /// Take a slice of the Sodg, keeping only the vertex specified
    /// by the locator.
    pub fn slice(&mut self, loc: &str) -> Result<Sodg> {
        let mut todo = HashSet::new();
        let mut done = HashSet::new();
        todo.insert(self.find(0, loc)?);
        loop {
            if todo.is_empty() {
                break;
            }
            let before: Vec<u32> = todo.drain().collect();
            for v in before {
                done.insert(v);
                let vtx = self.vertices.get(&v).unwrap();
                for to in vtx.edges.iter().map(|e| e.to) {
                    if done.contains(&to) {
                        continue;
                    }
                    done.insert(to);
                    todo.insert(to);
                }
            }
        }
        let mut new_vertices = HashMap::new();
        for (v, vtx) in self.vertices.iter().filter(|(v, _)| done.contains(v)) {
            new_vertices.insert(*v, vtx.clone());
        }
        Ok(Sodg {
            vertices: new_vertices,
            alerts: self.alerts.clone(),
            alerts_active: self.alerts_active,
        })
    }
}

#[test]
fn makes_a_slice() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    g.add(2)?;
    g.bind(0, 2, "bar")?;
    let slice = g.slice("bar")?;
    assert_eq!(1, slice.vertices.len());
    Ok(())
}
