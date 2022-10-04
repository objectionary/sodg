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

use crate::sot::Sot;
use log::error;

impl Sot {
    /// Validate the Sot and return all found data
    /// inconsistencies. This is mostly used for testing.
    pub fn inconsistencies(&self) -> Vec<String> {
        let mut errors = Vec::new();
        for e in self.lost_edges() {
            errors.push(e);
        }
        for e in errors.to_vec() {
            error!("{}", e)
        }
        errors
    }

    /// Finds all edges that have lost ends.
    fn lost_edges(&self) -> Vec<String> {
        let mut errors = Vec::new();
        for (v, vtx) in self.vertices.iter() {
            for e in vtx.edges.iter() {
                if !self.vertices.contains_key(&e.to) {
                    errors.push(format!("Edge ν{}.{} arrives to lost ν{}", v, e.a, e.to));
                }
            }
        }
        errors
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn finds_lost_edge() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    sot.add(1)?;
    sot.bind(0, 1, "foo")?;
    sot.vertices.remove(&1);
    assert_eq!(1, sot.inconsistencies().len());
    Ok(())
}
