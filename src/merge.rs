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

use crate::Sodg;
use log::debug;
use std::collections::HashMap;

impl Sodg {
    /// Merge another graph into itself.
    pub fn merge(&mut self, g: &Sodg) {
        let mut matcher: HashMap<u32, u32> = HashMap::new();
        let mut next = self.next_id();
        for (v, vtx) in g.vertices.iter() {
            let mut id = 0;
            if *v != 0 {
                id = next;
                next += 1;
            }
            matcher.insert(*v, id);
            self.vertices.insert(id, vtx.clone());
        }
        for v in matcher.values() {
            let vtx = self.vertices.get_mut(v).unwrap();
            for e in vtx.edges.iter_mut() {
                e.to = *matcher.get(v).unwrap();
            }
        }
        debug!(
            "Merged {} vertices into the existing Sodg",
            g.vertices.len()
        );
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn merges_two_graphs() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "bar")?;
    g.merge(&extra);
    assert_eq!(3, g.vertices.len());
    Ok(())
}
