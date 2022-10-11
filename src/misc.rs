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

use crate::Sot;
use std::collections::HashMap;
use std::fmt;

impl fmt::Debug for Sot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (i, v) in self.vertices.iter() {
            let mut attrs = v
                .edges
                .iter()
                .map(|e| format!("\n\t{} ➞ ν{}", e.a, e.to))
                .collect::<Vec<String>>();
            if !&v.data.is_empty() {
                attrs.push(format!("{}b", v.data.len()));
            }
            lines.push(format!("ν{} -> ⟦{}⟧", i, attrs.join(", ")));
        }
        f.write_str(lines.join("\n").as_str())
    }
}

impl Sot {
    /// Makes an empty Sot, with no vertices and no edges.
    pub fn empty() -> Self {
        Sot {
            vertices: HashMap::new(),
        }
    }

    /// Get max ID of a vertex.
    pub fn max(&self) -> u32 {
        let mut id = 0;
        for v in self.vertices.keys() {
            if *v > id {
                id = *v;
            }
        }
        id
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn makes_an_empty_sot() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    assert_eq!(1, sot.vertices.len());
    Ok(())
}

#[test]
fn calculates_max() -> Result<()> {
    let mut sot = Sot::empty();
    sot.add(0)?;
    sot.add(1)?;
    assert_eq!(1, sot.max());
    Ok(())
}
