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

use crate::{Deserialize, Edge, Hex, Serialize};
use std::collections::HashSet;

/// A vertex in the [`Sodg`].
#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Vertex {
    /// This is a list of edges departing from this vertex.
    pub edges: Vec<Edge>,
    /// This is the data in the vertex (possibly empty).
    pub data: Hex,
    /// This is a supplementary list of parent nodes, staying here for caching.
    pub parents: HashSet<u32>,
    /// This is `TRUE` if the data has been already taken by the use of [`Sodg::data`].
    pub taken: bool,
}

impl Vertex {
    /// Make an empty one.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg = Sodg::empty();
    /// sodg.add(0).unwrap();
    /// ```
    pub fn empty() -> Self {
        Vertex {
            edges: vec![],
            data: Hex::empty(),
            parents: HashSet::new(),
            taken: false,
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn makes_an_empty_vertex() -> Result<()> {
    let vtx = Vertex::empty();
    assert_eq!(0, vtx.edges.len());
    Ok(())
}
