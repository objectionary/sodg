// Copyright (c) 2022-2025 Objectionary.com
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

use crate::{Persistence, Sodg};
use anyhow::{Context, Result};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

impl<const N: usize> Display for Sodg<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Debug>::fmt(&self, f)
    }
}

impl<const N: usize> Debug for Sodg<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut lines = vec![];
        for (v, vtx) in self.vertices.iter() {
            if vtx.branch == 0 {
                continue;
            }
            let mut attrs = vtx
                .edges
                .iter()
                .map(|e| format!("\n\t{} ➞ ν{}", e.0, e.1))
                .collect::<Vec<String>>();
            if vtx.persistence != Persistence::Empty {
                attrs.push(format!("{}", vtx.data));
            }
            lines.push(format!("ν{v} -> ⟦{}⟧", attrs.join(", ")));
        }
        for (b, members) in self.branches.iter() {
            if members.is_empty() {
                continue;
            }
            lines.push(format!(
                "b{b}: {{{}}}",
                members
                    .into_iter()
                    .map(|v| format!("ν{v}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
        f.write_str(lines.join("\n").as_str())
    }
}

impl<const N: usize> Sodg<N> {
    /// Print a single vertex to a string, which can be used for
    /// logging and debugging.
    ///
    /// # Errors
    ///
    /// If the vertex is absent, an error may be returned.
    pub fn v_print(&self, v: usize) -> Result<String> {
        let vtx = &self
            .vertices
            .get(v)
            .with_context(|| format!("Can't find ν{v}"))?;
        let list: Vec<String> = vtx
            .edges
            .iter()
            .map(|e| format!("{}", e.0.clone()))
            .collect();
        Ok(format!(
            "ν{v}⟦{}{}⟧",
            if vtx.persistence == Persistence::Empty {
                ""
            } else {
                "Δ, "
            },
            list.join(", ")
        ))
    }
}

#[test]
fn prints_itself() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    assert_ne!("", format!("{g:?}"));
}

#[test]
fn displays_itself() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    assert_ne!("", format!("{g}"));
}
