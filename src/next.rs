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

impl Sodg {
    /// Get next unique ID of a vertex.
    ///
    /// This ID will never be returned by [`Sodg::next_id`] again. Also, this ID will not
    /// be equal to any of the existing IDs of vertices.
    pub fn next_id(&mut self) -> u32 {
        let mut id = self.next_v;
        id = self.vertices.try_id(id);
        self.next_v = id + 1;
        id
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn simple_next_id() -> Result<()> {
    let mut g = Sodg::empty();
    assert_eq!(0, g.next_id());
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
    Ok(())
}

#[test]
fn calculates_next_id() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(42)?;
    assert_eq!(43, g.next_id());
    assert_eq!(44, g.next_id());
    Ok(())
}

#[test]
fn next_id_after_inject() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    assert_eq!(0, g.next_id());
    assert_eq!(2, g.next_id());
    Ok(())
}

#[test]
fn next_id_after_sequence() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    assert_eq!(2, g.next_id());
    assert_eq!(3, g.next_id());
    Ok(())
}

#[test]
fn next_id_after_zero() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
    Ok(())
}
