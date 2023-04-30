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

use crate::{Label, Sodg};
use itertools::Itertools;

impl<const N: usize> Sodg<N> {
    /// Print SODG as a DOT graph.
    ///
    /// For example, for this code:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Hex, Label};
    /// use sodg::Sodg;
    /// let mut g : Sodg<16> = Sodg::empty(256);
    /// g.add(0);
    /// g.put(0, &Hex::from_str_bytes("hello"));
    /// g.add(1);
    /// g.bind(0, 1, Label::from_str("foo").unwrap());
    /// g.bind(0, 1, Label::from_str("bar").unwrap());
    /// let dot = g.to_dot();
    /// println!("{}", dot);
    /// ```
    ///
    /// The printout will look approximately like this:
    ///
    /// ```text
    /// digraph {
    ///   v0[shape=circle,label="ν0"];
    ///   v0 -> v1 [label="bar"];
    ///   v0 -> v1 [label="foo"];
    ///   v1[shape=circle,label="ν1"];
    /// }
    /// ```
    #[must_use]
    pub fn to_dot(&self) -> String {
        let mut lines: Vec<String> = vec![];
        lines.push(
            "/* Render it at https://dreampuf.github.io/GraphvizOnline/ */
digraph {
  node [fixedsize=true,width=1,fontname=\"Arial\"];
  edge [fontname=\"Arial\"];"
                .to_string(),
        );
        for (v, edges) in self
            .edges
            .iter()
            .sorted_by_key(|(v, _)| <usize>::clone(v))
        {
            lines.push(format!(
                "  v{v}[shape=circle,label=\"ν{v}\"{}]; {}",
                if self.data.contains_key(v) {
                    ""
                } else {
                    ",color=\"#f96900\""
                },
                self.data.get(v)
                    .as_ref()
                    .map_or_else(String::new, |d| format!("/* {d} */"))
            ));
            for e in edges.into_iter().sorted_by_key(|e| e.0) {
                lines.push(format!(
                    "  v{v} -> v{} [label=\"{}\"{}{}];",
                    e.1,
                    e.0,
                    match e.0 {
                        Label::Greek(g) => {
                            if g == 'ρ' || g == 'σ' {
                                ",color=gray,fontcolor=gray"
                            } else {
                                ""
                            }
                        }
                        _ => {
                            ""
                        }
                    },
                    match e.0 {
                        Label::Greek(g) => {
                            if g == 'π' {
                                ",style=dashed"
                            } else {
                                ""
                            }
                        }
                        _ => {
                            ""
                        }
                    }
                ));
            }
        }
        lines.push("}\n".to_string());
        lines.join("\n")
    }
}

#[cfg(test)]
use crate::Hex;

#[cfg(test)]
use anyhow::Result;

#[test]
fn simple_graph_to_dot() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.put(0, &Hex::from_str_bytes("hello"));
    g.add(1);
    g.bind(0, 1, Label::Alpha(0));
    let dot = g.to_dot();
    assert!(dot.contains("shape=circle,label=\"ν0\""));
    Ok(())
}
