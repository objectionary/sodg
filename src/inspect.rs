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

use crate::DeadRelay;
use crate::Sodg;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashSet;

impl Sodg {
    /// Find an object by the provided locator and print its tree
    /// of sub-objects and edges.
    ///
    /// The function is mostly used for testing.
    ///
    /// # Errors
    ///
    /// If it's impossible to inspect, an error will be returned.
    pub fn inspect(&self, loc: &str) -> Result<String> {
        let v = self
            .find(0, loc, &DeadRelay::default())
            .context(format!("Can't locate '{loc}'"))?;
        let mut seen = HashSet::new();
        Ok(format!(
            "{}/ν{}\n{}",
            loc,
            v,
            self.inspect_v(v, &mut seen)?.join("\n")
        ))
    }

    fn inspect_v(&self, v: u32, seen: &mut HashSet<u32>) -> Result<Vec<String>> {
        seen.insert(v);
        let mut lines = vec![];
        self.vertices
            .get(&v)
            .context(format!("Can't find ν{v}"))?
            .edges
            .iter()
            .sorted()
            .for_each(|e| {
                let skip = seen.contains(&e.to);
                let line = format!(
                    "  .{} ➞ ν{}{}",
                    e.a,
                    e.to,
                    if skip {
                        "…".to_string()
                    } else {
                        String::new()
                    }
                );
                lines.push(line);
                if !skip {
                    seen.insert(e.to);
                    self.inspect_v(e.to, seen)
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

#[test]
fn inspects_simple_object() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.put(0, &Hex::from_str_bytes("hello"))?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let txt = g.inspect("")?;
    println!("{}", txt);
    assert_ne!("".to_string(), txt);
    Ok(())
}
