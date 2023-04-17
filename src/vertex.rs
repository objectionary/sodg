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

use crate::{Edges, Vertex};
#[cfg(feature = "gc")]
use std::collections::HashSet;

impl<const N: usize> Vertex<N> {
    /// Make an empty one.
    ///
    /// For example:
    ///
    /// ```
    /// use sodg::Sodg;
    /// let mut sodg : Sodg<16> = Sodg::empty();
    /// sodg.add(0).unwrap();
    /// ```
    pub fn empty() -> Self {
        Self {
            edges: Edges::new(),
            data: None,
            taken: false,
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
use crate::Label;

#[test]
fn makes_an_empty_vertex() -> Result<()> {
    let mut v: Vertex<4> = Vertex::empty();
    v.edges.insert(Label::Alpha(0), 1);
    assert_eq!(1, v.edges.into_iter().next().unwrap().1);
    Ok(())
}
