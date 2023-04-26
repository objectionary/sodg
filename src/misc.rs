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

impl<const N: usize> Sodg<N> {
    /// Get total number of vertices in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Get all IDs of vertices, in a vector.
    #[must_use]
    pub fn ids(&self) -> Vec<usize> {
        let mut ids = vec![];
        for (v, _) in self.vertices.iter() {
            ids.push(v);
        }
        ids
    }

    /// Is it empty?
    ///
    /// Emptiness means that not a single vertex is in the graph.
    ///
    /// For example:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sodg::{Label, Sodg};
    /// let mut sodg : Sodg<16> = Sodg::empty(256);
    /// sodg.add(0);
    /// sodg.add(42);
    /// sodg.bind(0, 42, Label::from_str("hello").unwrap()).unwrap();
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vertices.len() == 0
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn checks_for_emptiness() -> Result<()> {
    let g: Sodg<16> = Sodg::empty(256);
    assert!(g.is_empty());
    Ok(())
}

#[test]
fn counts_vertices() -> Result<()> {
    let g: Sodg<16> = Sodg::empty(256);
    assert_eq!(0, g.len());
    Ok(())
}

#[test]
fn collect_vertices() -> Result<()> {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    assert!(g.ids().contains(&1));
    Ok(())
}
