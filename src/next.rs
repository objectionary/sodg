// Copyright (c) 2022-2025 Objectionary.com
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
    /// Get next unique ID of a vertex.
    ///
    /// This ID will never be returned by [`Sodg::next_id`] again. Also, this ID will not
    /// be equal to any of the existing IDs of vertices.
    ///
    /// # Panics
    ///
    /// May panic if not enough IDs are available.
    #[inline]
    pub fn next_id(&mut self) -> usize {
        let mut id = self.next_v;
        id = self
            .vertices
            .iter()
            .find(|(v, vtx)| vtx.branch == 0 && *v >= id)
            .map(|(v, _)| v)
            .unwrap();
        let next = id + 1;
        if next > self.next_v {
            self.next_v = next;
        }
        id
    }
}

#[test]
fn simple_next_id() {
    let mut g: Sodg<16> = Sodg::empty(256);
    assert_eq!(0, g.next_id());
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
}

#[test]
fn calculates_next_id() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(42);
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
}

#[test]
fn next_id_after_inject() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    assert_eq!(0, g.next_id());
    assert_eq!(2, g.next_id());
}

#[test]
fn next_id_after_sequence() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    g.add(1);
    assert_eq!(2, g.next_id());
    assert_eq!(3, g.next_id());
}

#[test]
fn next_id_after_zero() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert_eq!(1, g.next_id());
    assert_eq!(2, g.next_id());
}
