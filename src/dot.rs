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
use itertools::Itertools;

impl Sodg {
    /// Print SODG as a DOT graph.
    ///
    /// For example, for this code:
    ///
    /// ```
    /// use sodg::Hex;
    /// use sodg::Sodg;
    /// let mut g = Sodg::empty();
    /// g.add(0).unwrap();
    /// g.put(0, Hex::from_str_bytes("hello")).unwrap();
    /// g.add(1).unwrap();
    /// g.bind(0, 1, "foo").unwrap();
    /// g.bind(0, 1, "bar").unwrap();
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
    pub fn to_dot(&self) -> String {
        let mut lines: Vec<String> = vec![];
        lines.push(
            "/* Render it at https://dreampuf.github.io/GraphvizOnline/ */
digraph {
  node [fixedsize=true,width=1,fontname=\"Arial\"];
  edge [fontname=\"Arial\"];"
                .to_string(),
        );
        for (v, vtx) in self
            .vertices
            .iter()
            .sorted_by_key(|(v, _)| <&u32>::clone(v))
        {
            lines.push(format!(
                "  v{v}[shape=circle,label=\"ν{v}\"{}]; {}",
                if vtx.data.is_empty() {
                    ""
                } else {
                    ",color=\"#f96900\""
                },
                if vtx.data.is_empty() {
                    "".to_string()
                } else {
                    format!("/* {} */", vtx.data)
                }
            ));
            for e in vtx.edges.iter().sorted_by_key(|e| e.a.clone()) {
                lines.push(format!(
                    "  v{v} -> v{} [label=\"{}\"{}{}];",
                    e.to,
                    e.a,
                    if e.a.starts_with('ρ') || e.a.starts_with('σ') {
                        ",color=gray,fontcolor=gray"
                    } else {
                        ""
                    },
                    if e.a.starts_with('π') {
                        ",style=dashed"
                    } else {
                        ""
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
    let mut g = Sodg::empty();
    g.add(0)?;
    g.put(0, &Hex::from_str_bytes("hello"))?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let dot = g.to_dot();
    assert!(dot.contains("shape=circle,label=\"ν0\""));
    Ok(())
}
