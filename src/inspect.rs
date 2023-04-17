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
use itertools::Itertools;
use std::collections::HashSet;

impl<const N: usize> Sodg<N> {
    /// Find an object by the provided locator and print its tree
    /// of sub-objects and edges.
    ///
    /// The function is mostly used for testing.
    ///
    /// # Errors
    ///
    /// If it's impossible to inspect, an error will be returned.
    pub fn inspect(&self, v: u32) -> Result<String> {
        let mut seen = HashSet::new();
        Ok(format!(
            "ν{}\n{}",
            v,
            self.inspect_v(v, &mut seen)?.join("\n")
        ))
    }

    fn inspect_v(&self, v: u32, seen: &mut HashSet<u32>) -> Result<Vec<String>> {
        seen.insert(v);
        let mut lines = vec![];
        self.vertices
            .get(v)
            .with_context(|| format!("Can't find ν{v}"))?
            .edges
            .into_iter()
            .sorted()
            .for_each(|e| {
                let skip = seen.contains(&e.1);
                let line = format!(
                    "  .{} ➞ ν{}{}",
                    e.0,
                    e.1,
                    if skip {
                        "…".to_string()
                    } else {
                        String::new()
                    }
                );
                lines.push(line);
                if !skip {
                    seen.insert(e.1);
                    self.inspect_v(e.1, seen)
                        .unwrap()
                        .iter()
                        .for_each(|t| lines.push(format!("  {t}")));
                }
            });
        Ok(lines)
    }
}

#[cfg(test)]
use crate::Hex;

#[cfg(test)]
use crate::Label;

#[test]
fn inspects_simple_object() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty();
    g.add(0)?;
    g.put(0, &Hex::from_str_bytes("hello"))?;
    g.add(1)?;
    let txt = g.inspect(0)?;
    g.bind(0, 1, Label::Alpha(0))?;
    println!("{txt}");
    assert_ne!(String::new(), txt);
    Ok(())
}
