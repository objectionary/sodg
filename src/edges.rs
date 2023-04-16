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

use crate::{Edges, EdgesIntoIter, Label, Roll};

impl<'a, const N: usize> Iterator for EdgesIntoIter<'a, N> {
    type Item = (Label, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Edges<N> {
    type Item = (Label, u32);
    type IntoIter = EdgesIntoIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        EdgesIntoIter {
            iter: self.map.into_iter(),
        }
    }
}

impl<const N: usize> Edges<N> {
    #[inline]
    pub fn new() -> Self {
        Self { map: Roll::new() }
    }

    #[inline]
    pub fn insert(&mut self, a: Label, v: u32) {
        self.map.insert(a, v);
    }
}

#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
use bincode::{deserialize, serialize};

#[test]
fn serialize_and_deserialize() -> Result<()> {
    let mut before: Edges<4> = Edges::new();
    before.insert(Label::Alpha(0), 42);
    let bytes: Vec<u8> = serialize(&before)?;
    let after: Edges<4> = deserialize(&bytes)?;
    assert_eq!(42, after.into_iter().next().unwrap().1);
    Ok(())
}
