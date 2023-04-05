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
use anyhow::{Context, Result};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

impl Display for Sodg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Debug>::fmt(&self, f)
    }
}

impl Debug for Sodg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (i, v) in self.vertices.iter() {
            let mut attrs = v
                .edges
                .iter()
                .map(|e| format!("\n\t{} ➞ ν{}", e.0, e.1))
                .collect::<Vec<String>>();
            if !&v.data.is_empty() {
                attrs.push(format!("{}", v.data));
            }
            lines.push(format!("ν{i} -> ⟦{}⟧", attrs.join(", ")));
        }
        f.write_str(lines.join("\n").as_str())
    }
}

impl Sodg {
    /// Print a single vertex to a string, which can be used for
    /// logging and debugging.
    ///
    /// # Errors
    ///
    /// If the vertex is absent, an error may be returned.
    pub fn v_print(&self, v: u32) -> Result<String> {
        let vtx = self
            .vertices
            .get(v)
            .with_context(|| format!("Can't find ν{v}"))?;
        let list: Vec<String> = vtx.edges.iter().map(|e| e.0.clone()).collect();
        Ok(format!(
            "ν{v}⟦{}{}⟧",
            if vtx.data.is_empty() { "" } else { "Δ, " },
            list.join(", ")
        ))
    }
}

#[test]
fn prints_itself() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    assert_ne!("", format!("{:?}", g));
    Ok(())
}

#[test]
fn displays_itself() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    assert_ne!("", format!("{g}"));
    Ok(())
}
